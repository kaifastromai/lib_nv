/*!  /[Nvfs] is the Novella Virtual File System. It is used to serialize and deserialize a .nv project.
 * It is designed to be able to store and retrieve data from a .nv file, and to stream data from disk to memory, as needed.
 * It based on a sqlite database.
*/

#![feature(min_specialization)]
use common::{
    exports::{
        anyhow::{anyhow, Result},
        serde::de::DeserializeOwned,
        serde::*,
    },
    type_id::{TypeId, TypeIdTy},
};

use nvproc::TypeId;
use rusqlite::{types::Type, Connection};
use std::any::Any;
use std::collections::HashMap;
use std::fs::*;
use std::path::{Path, PathBuf};

pub trait BinarySerdeTy: TypeIdTy + Serialize + DeserializeOwned {}

impl<T: Serialize + DeserializeOwned + TypeIdTy> BinarySerdeTy for T {}
pub trait BinaryTy: 'static {
    fn get_any(&self) -> &dyn Any;
}
impl<T> BinaryTy for T
where
    T: 'static + Serialize + Clone + TypeIdTy,
{
    fn get_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "common::exports::serde")]
#[repr(C)]
pub struct BinaryStorage {
    pub component_type: TypeId,
    component: Vec<u8>,
}

impl BinaryStorage {
    pub fn new<T: BinarySerdeTy>(component: T) -> Self {
        let data = bincode::serialize(&component).unwrap();
        Self {
            component: data,
            component_type: T::get_type_id(),
        }
    }
    pub fn from_bytes<T: BinarySerdeTy + DeserializeOwned>(data: &[u8]) -> Result<T> {
        let bs: BinaryStorage = bincode::deserialize(data)?;
        if bs.component_type != TypeId::of::<T>() {
            return Err(anyhow!("TypeId mismatch"));
        }
        let component: T = bincode::deserialize(&bs.component)?;
        Ok(component)
    }
}
pub enum FsPath {
    ///The data is stored inside of a .nv file
    Internal(PathBuf),
    ///The data is stored in a file on the disk, and is accessed via a path
    File(PathBuf),
}
pub enum Inode {
    Dir(DinodeData),
    Item(InodeData),
}
impl Inode {
    pub fn id(&self) -> u64 {
        match self {
            Inode::Dir(dinode) => dinode.id,
            Inode::Item(inode) => inode.id,
        }
    }
    pub fn name(&self) -> String {
        match self {
            Inode::Dir(dinode) => dinode.name.clone(),
            Inode::Item(inode) => inode.name.clone(),
        }
    }
    pub fn parent_id(&self) -> u64 {
        match self {
            Inode::Dir(dinode) => dinode.parent_id,
            Inode::Item(inode) => inode.parent_id,
        }
    }
    pub fn is_dir(&self) -> bool {
        match self {
            Inode::Dir(_) => true,
            Inode::Item(_) => false,
        }
    }
    pub fn is_item(&self) -> bool {
        match self {
            Inode::Dir(_) => false,
            Inode::Item(_) => true,
        }
    }
    pub fn ext(&self) -> ExtTypes {
        match self {
            Inode::Dir(dinode) => dinode.ext.clone(),
            Inode::Item(inode) => inode.ext.clone(),
        }
    }
    //only valid on items
    pub fn get_data(&self) -> Option<u64> {
        match self {
            Inode::Dir(_) => None,
            Inode::Item(inode) => Some(inode.payload),
        }
    }
    ///Only valid on directories
    pub fn dir_get_child_item_ids(&self) -> Option<Vec<u64>> {
        match self {
            Inode::Dir(dinode) => Some(dinode.children.clone()),
            Inode::Item(_) => None,
        }
    }
    ///Only valid for directories. Adds a child item to the directory's children
    pub fn dir_add_item(&mut self, item_id: u64) -> Result<()> {
        match self {
            Inode::Dir(dinode) => {
                dinode.children.push(item_id);
                Ok(())
            }
            Inode::Item(_) => Err(anyhow!("Can't add item to a non-directory")),
        }
    }
    ///Only valid on items. Sets the payload of the item
    pub fn item_set_payload(&mut self, payload: u64) -> Result<()> {
        match self {
            Inode::Dir(_) => Err(anyhow!("Can't set data on a directory")),
            Inode::Item(inode) => {
                inode.payload = payload;
                Ok(())
            }
        }
    }
    pub fn item_get_payload_id(&self) -> Result<u64> {
        match self {
            Inode::Dir(_) => Err(anyhow!("Can't get payload on a directory")),
            Inode::Item(inode) => Ok(inode.payload),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExtTypes {
    Video,
    Audio,
    Image,
    Binary,
    Component,
    String(String),
    // a special extension for a directory
    Dir,
}
//An inode representing a file
pub struct InodeData {
    pub id: u64,
    pub parent_id: u64,
    pub ext: ExtTypes,
    pub name: String,
    pub payload: u64,
}
//An inode representing a directory
pub struct DinodeData {
    pub id: u64,
    pub parent_id: u64,
    pub ext: ExtTypes,
    pub name: String,
    pub children: Vec<u64>,
}
pub struct Vfs {
    pub data: HashMap<u64, Box<dyn BinaryTy>>,
    pub inodes: HashMap<u64, Inode>,
    pub node_used_list: Vec<u64>,
    pub data_used_list: Vec<u64>,
}

impl Vfs {
    pub fn new() -> Self {
        Vfs {
            data: HashMap::new(),
            inodes: HashMap::from([(
                0,
                Inode::Dir(DinodeData {
                    id: 0,
                    parent_id: 0,
                    ext: ExtTypes::Dir,
                    name: "nv_root".to_string(),
                    children: Vec::new(),
                }),
            )]),
            node_used_list: vec![0],
            data_used_list: Vec::new(),
        }
    }
    pub fn get_node(&self, id: u64) -> &Inode {
        self.inodes.get(&id).unwrap()
    }
    pub fn get_node_mut(&mut self, id: u64) -> &mut Inode {
        self.inodes.get_mut(&id).unwrap()
    }
    pub fn get_node_from_path(&self, path: PathBuf) -> Result<&Inode> {
        //skip root
        let path_start = path.strip_prefix("/").unwrap();
        let path_buf = PathBuf::from(path_start);
        let id = self.get_node_from_path_recursive(path_buf, 0)?;
        Ok(self.get_node(id))
    }
    pub fn get_item_payload<T: BinaryTy>(&self, id: u64) -> Result<&T> {
        let data = self
            .data
            .get(&id)
            .ok_or_else(|| anyhow!("No data for id {}", id))?;
        let any_ref = data
            .get_any()
            .downcast_ref()
            .ok_or_else(|| anyhow!("Could not downcast to {}", std::any::type_name::<T>()))?;
        Ok(any_ref)
    }
    pub fn get_node_mut_from_path(&mut self, path: PathBuf) -> Result<&mut Inode> {
        //skip root
        let path_start = path.strip_prefix("/").unwrap();
        let path_buf = PathBuf::from(path_start);
        let id = self.get_node_from_path_recursive(path_buf, 0)?;
        Ok(self.get_node_mut(id))
    }
    pub fn verify_path(&self, path: PathBuf) -> bool {
        let id = self.get_node_from_path(path);
        id.is_ok()
    }
    pub fn add_item<T: BinaryTy + Serialize>(
        &mut self,
        item: T,
        path: PathBuf,
        item_name: String,
    ) -> Result<()> {
        let id = self.new_node_id();
        let parent_dir = self.get_node_mut_from_path(path)?;
        parent_dir.dir_add_item(id)?;

        let mut inode = Inode::Item(InodeData {
            id,
            parent_id: parent_dir.id(),
            ext: ExtTypes::Binary,
            name: item_name,
            payload: 0,
        });
        {
            let item_id = self.add_item_entry(item);
            inode.item_set_payload(item_id)?;
        }
        self.new_inode(inode);
        Ok(())
    }

    fn add_item_entry<T: BinaryTy>(&mut self, item: T) -> u64 {
        let id = self.new_item_id();
        self.data.insert(id, Box::new(item));
        id
    }
    fn new_item_id(&mut self) -> u64 {
        let mut id = common::uuid::gen_64();
        while self.data_used_list.contains(&id) {
            id = common::uuid::gen_64();
        }
        self.data_used_list.push(id);
        id
    }
    fn new_node_id(&mut self) -> u64 {
        let mut id = common::uuid::gen_64();
        while self.node_used_list.contains(&id) {
            id = common::uuid::gen_64();
        }
        self.node_used_list.push(id);
        id
    }
    fn new_inode(&mut self, node: Inode) {
        self.inodes.insert(node.id(), node);
    }
    fn get_node_from_path_recursive(&self, path: PathBuf, parent_node: u64) -> Result<u64> {
        match path.iter().nth(1) {
            //this is not the last directory
            Some(_) => {
                let p = path.iter().next().unwrap();
                //find id of node
                if let Inode::Dir(dir) = self.get_node(parent_node) {
                    let id = dir.children.iter().find(|x| {
                        let node = self.get_node(**x);
                        if let Inode::Dir(dir) = node {
                            dir.name == p.to_str().unwrap()
                        } else {
                            false
                        }
                    });
                    if let Some(id) = id {
                        let new_path = path.strip_prefix(path.iter().next().unwrap()).unwrap();
                        let new_path_buf = PathBuf::from(new_path);
                        self.get_node_from_path_recursive(new_path_buf, *id)
                    } else {
                        Err(anyhow!("Could not find path"))
                    }
                } else {
                    return Err(anyhow!("Parent node is not a directory"));
                }
            }

            //this is the last item, just check for the item
            None => {
                if let Inode::Dir(dir) = self.get_node(parent_node) {
                    let id = dir.children.iter().find(|x| {
                        let node = self.get_node(**x);
                        node.name() == path.file_name().unwrap().to_str().unwrap()
                    });
                    if let Some(id) = id {
                        Ok(*id)
                    } else {
                        Err(anyhow!("Could not find path"))
                    }
                } else {
                    return Err(anyhow!("Parent node is not a directory"));
                }
            }
        }
    }

    fn path_to_inode(&self, path: PathBuf) -> Inode {
        todo!()
    }

    //The path should be implicitely in terms of the nv_root;
    pub fn create_dir(&mut self, path: PathBuf) -> Result<()> {
        let starting_path = path.iter().next();
        let sub_path = path.strip_prefix(starting_path.unwrap()).unwrap();
        //into PathBuf
        let sub_path_buf = PathBuf::from(sub_path);
        //call recursive
        self.create_dir_recursive(sub_path_buf, 0)
    }
    fn create_dir_recursive(&mut self, path: PathBuf, parent_node: u64) -> Result<()> {
        match path.iter().nth(1) {
            //theres more to go
            Some(_) => {
                let p = path.iter().next().unwrap();
                //find the path with the name in the parent node
                if let Inode::Dir(dir) = self.get_node(parent_node) {
                    for id in dir.children.iter() {
                        if let Inode::Dir(dir) = self.get_node(*id) {
                            if dir.name == p.to_str().unwrap() {
                                let sub_path =
                                    path.strip_prefix(path.iter().next().unwrap()).unwrap();
                                let sub_path_buf = PathBuf::from(sub_path);
                                let id = *id;
                                return self.create_dir_recursive(sub_path_buf, id);
                            }
                        }
                    }
                }
                Ok(())
            }
            //this is the last path, create the dinode
            None => {
                //create the dinode
                let dinode = DinodeData {
                    id: self.node_used_list.len() as u64,
                    parent_id: parent_node,
                    ext: ExtTypes::Dir,
                    name: path.to_string_lossy().to_string(),
                    children: Vec::new(),
                };
                //add the dinode to the index used list
                self.node_used_list.push(dinode.id);
                if let Inode::Dir(dir) = self.get_node_mut(parent_node) {
                    dir.children.push(dinode.id);
                }
                //add the dinode to the inode list
                self.inodes.insert(dinode.id, Inode::Dir(dinode));
                //add the dinode to the parent node list

                //return
                Ok(())
            }
        }
    }
}

impl Default for Vfs {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Nvfs {
    pub connection: Connection,
}

impl Nvfs {
    //open with path to the .nv file
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self> {
        let connection = Connection::open(path)?;
        Ok(Self { connection })
    }
    pub fn create_table(&self, table_name: &str, columns: &[&str]) -> Result<()> {
        let mut statement = self.connection.prepare(
            format!(
                "CREATE TABLE IF NOT EXISTS {} ({})",
                table_name,
                columns.join(",")
            )
            .as_str(),
        )?;
        statement.execute([])?;
        Ok(())
    }
    pub fn insert_into_table(
        &self,
        table_name: &str,
        columns: &[&str],
        values: &[&str],
    ) -> Result<()> {
        let mut statement = self.connection.prepare(
            format!(
                "INSERT INTO {} ({}) VALUES ({})",
                table_name,
                columns.join(","),
                values.join(",")
            )
            .as_str(),
        )?;
        statement.execute([])?;
        Ok(())
    }
    pub fn req(&self, path: FsPath) {}
}

#[cfg(test)]
mod test_super {
    use super::*;
    use rusqlite::params;
    use std::io::{Read, Write};
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(crate = "common::exports::serde")]
    pub struct TestStruct {
        pub id: u32,
        pub name: String,
        pub description: String,
    }
    // impl<T: 'static> BinarySerializeTy for T {
    //     fn get_any(&self) -> &dyn Any {
    //         self
    //     }
    // }

    //implement BinarySerializeType for TestStruct
    // impl BinarySerializeTy for TestStruct {
    //     fn get_any(&self) -> &dyn Any {
    //         self
    //     }
    // }

    #[test]
    fn test_serialize() {
        // let test_struct = TestStruct {
        //     id: 1,
        //     name: String::from("test"),
        //     description: String::from("test"),
        // };
        // let conn = Connection::open("test.nv").unwrap();
        // conn.execute(
        //     r"
        // CREATE TABLE IF NOT EXISTS test_table(
        //     id INTEGER PRIMARY KEY,
        //     name TEXT,
        //     description TEXT
        // );
        // ",
        //     [],
        // )
        // .unwrap();
        // let mut stmt = conn
        //     .prepare(
        //         r"
        // INSERT INTO test_table(id, name, description)
        // VALUES(?, ?, ?);
        // ",
        //     )
        //     .unwrap();
        // stmt.execute(params![
        //     test_struct.id,
        //     test_struct.name,
        //     test_struct.description
        // ])
        // .unwrap();
        // //get the value back
        // let mut stmt = conn
        //     .prepare(
        //         r"
        // SELECT id, name, description FROM test_table;
        // ",
        //     )
        //     .unwrap();
        // let mut rows = stmt.query_map([], |row| row.get(0)).unwrap();
        // let row: u32 = rows.next().unwrap().unwrap();
        // assert_eq!(test_struct.id, row);
    }
    #[test]
    fn test_dir_create() {
        let mut vfs = Vfs::new();
        vfs.create_dir(PathBuf::from("/test")).unwrap();
        vfs.create_dir(PathBuf::from("/test/test2")).unwrap();
        vfs.create_dir(PathBuf::from("/test/test3")).unwrap();
        let inode1 = vfs
            .get_node_from_path(PathBuf::from("/test/test2"))
            .unwrap();
        let inode2 = vfs
            .get_node_from_path(PathBuf::from("/test/test3"))
            .unwrap();

        assert_eq!(inode1.name(), "test2");
        assert_eq!(inode2.name(), "test3");
    }

    #[test]
    fn test_add_payload() -> Result<()> {
        let mut vfs = Vfs::new();
        vfs.create_dir(PathBuf::from("/test")).unwrap();
        vfs.create_dir(PathBuf::from("/test/test1")).unwrap();
        vfs.create_dir(PathBuf::from("/test/test2")).unwrap();
        let ts1 = TestStruct {
            id: 1,
            name: String::from("test"),
            description: String::from("test"),
        };
        let ts2 = TestStruct {
            id: 2,
            name: String::from("test2"),
            description: String::from("test2"),
        };
        vfs.add_item(
            ts1,
            PathBuf::from("/test/test1"),
            "test1.struct".to_string(),
        )
        .unwrap();
        vfs.add_item(
            ts2,
            PathBuf::from("/test/test2"),
            "test2.struct".to_string(),
        )
        .unwrap();
        let inode1 = vfs
            .get_node_from_path(PathBuf::from("/test/test1/test1.struct"))
            .unwrap();
        let inode2 = vfs
            .get_node_from_path(PathBuf::from("/test/test2/test2.struct"))
            .unwrap();
        assert_eq!(inode1.name(), "test1.struct");
        assert_eq!(inode2.name(), "test2.struct");
        let item1 = vfs
            .get_item_payload::<TestStruct>(inode1.item_get_payload_id()?)
            .unwrap();
        let item2 = vfs
            .get_item_payload::<TestStruct>(inode2.item_get_payload_id()?)
            .unwrap();
        assert_eq!(item1.id, 1);
        assert_eq!(item2.id, 2);

        Ok(())
    }
    #[test]
    fn test_serde() {
        let ts = TestStruct {
            id: 1,
            name: String::from("test"),
            description: String::from("test"),
        };
        let bs = BinaryStorage::new(ts);
        let bytes = bincode::serialize(&bs).unwrap();
        //write to disk
        let mut file = File::create("test.struct").unwrap();
        file.write_all(&bytes).unwrap();
        //read from disk
        let mut file = File::open("test.struct").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        let ts2 = BinaryStorage::from_bytes::<TestStruct>(&bytes).unwrap();
        assert_eq!(ts2.id, 1);
        assert_eq!(ts2.name, "test");
    }
}
