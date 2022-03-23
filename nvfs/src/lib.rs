/*!  /[Nvfs] is the Novella Virtual File System. It is used to serialize and deserialize a .nv project.
 * It is designed to be able to store and retrieve data from a .nv file, and to stream data from disk to memory, as needed.
 * It based on a sqlite database.
*/
use rusqlite::Connection;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use utils::exports::{
    anyhow::{anyhow, Result},
    serde::*,
};


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
            Inode::Item(inode) => Some(inode.data),
        }
    }
    //Only valid on directories
    pub fn get_child_item_ids(&self) -> Option<Vec<u64>> {
        match self {
            Inode::Dir(dinode) => Some(dinode.data.clone()),
            Inode::Item(_) => None,
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
    pub data: u64,
}
//An inode representing a directory
pub struct DinodeData {
    pub id: u64,
    pub parent_id: u64,
    pub ext: ExtTypes,
    pub name: String,
    pub data: Vec<u64>,
}
pub trait BinarySerializeTy: 'static {}
pub struct Vfs {
    pub data: Vec<Box<dyn BinarySerializeTy>>,
    pub inodes: HashMap<u64, Inode>,
    pub index_used_list: Vec<u64>,
}

impl Vfs {
    pub fn new() -> Self {
        Vfs {
            data: Vec::new(),
            inodes: HashMap::from([(
                0,
                Inode::Dir(DinodeData {
                    id: 0,
                    parent_id: 0,
                    ext: ExtTypes::Dir,
                    name: "nv_root".to_string(),
                    data: Vec::new(),
                }),
            )]),
            index_used_list: vec![0],
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
    pub fn verify_path(&self, path: PathBuf) -> bool {
        let id = self.get_node_from_path(path);
        id.is_ok()
    }
    pub fn add_item<T: BinarySerializeTy + Serialize>(
        &mut self,
        item: T,
        path: PathBuf,
        item_name: String,
    ) -> Result<()> {
        let inode = Inode::Item(InodeData {
            id: self.index_used_list.len() as u64,
            parent_id: self.get_node_from_path_recursive(path.clone(), 0).unwrap(),
            ext: ExtTypes::Binary,
            name: item_name,
            data: self.data.len() as u64,
        });
        //check path is valid
        if !self.verify_path(path) {
            return Err(anyhow!("Path is invalid"));
        }
        self.index_used_list.push(inode.id());
        self.inodes.insert(inode.id(), inode);
        self.data.push(Box::new(item));
        Ok(())
    }
    fn get_node_from_path_recursive(&self, path: PathBuf, parent_node: u64) -> Result<u64> {
        match path.iter().nth(1) {
            //this is not the last directory
            Some(_) => {
                let p = path.iter().next().unwrap();
                //find id of node
                if let Inode::Dir(dir) = self.get_node(parent_node) {
                    let id = dir.data.iter().find(|x| {
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
                    let id = dir.data.iter().find(|x| {
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
    //The path should not be implicitely in terms of the nv_root;
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
                    for id in dir.data.iter() {
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
                    id: self.index_used_list.len() as u64,
                    parent_id: parent_node,
                    ext: ExtTypes::Dir,
                    name: path.to_string_lossy().to_string(),
                    data: Vec::new(),
                };
                //add the dinode to the index used list
                self.index_used_list.push(dinode.id);
                if let Inode::Dir(dir) = self.get_node_mut(parent_node) {
                    dir.data.push(dinode.id);
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
    use rusqlite::params;

    use super::*;
    #[derive(Serialize, Deserialize, Debug)]
    #[serde(crate = "utils::exports::serde")]
    pub struct TestStruct {
        pub id: u32,
        pub name: String,
        pub description: String,
    }
    impl BinarySerializeTy for TestStruct {}

    #[test]
    fn test_serialize() {
        let test_struct = TestStruct {
            id: 1,
            name: String::from("test"),
            description: String::from("test"),
        };
        let conn = Connection::open("test.nv").unwrap();
        conn.execute(
            r"
        CREATE TABLE IF NOT EXISTS test_table(
            id INTEGER PRIMARY KEY,
            name TEXT,
            description TEXT
        );
        ",
            [],
        )
        .unwrap();
        let mut stmt = conn
            .prepare(
                r"
        INSERT INTO test_table(id, name, description) 
        VALUES(?, ?, ?);
        ",
            )
            .unwrap();
        stmt.execute(params![
            test_struct.id,
            test_struct.name,
            test_struct.description
        ])
        .unwrap();
        //get the value back
        let mut stmt = conn
            .prepare(
                r"
        SELECT id, name, description FROM test_table;
        ",
            )
            .unwrap();
        let mut rows = stmt.query_map([], |row| row.get(0)).unwrap();
        let row: u32 = rows.next().unwrap().unwrap();
        assert_eq!(test_struct.id, row);
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
    fn test_add_payload() {
        let mut vfs = Vfs::new();
        vfs.create_dir(PathBuf::from("/test")).unwrap();
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
    }
}
