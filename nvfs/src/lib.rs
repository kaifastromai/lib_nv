/*!  /[Nvfs] is the Novella Virtual File System. It is used to serialize and deserialize a .nv project.
 * It is designed to be able to store and retrieve data from a .nv file, and to stream data from disk to memory, as needed.
 * It based on a sqlite database.
*/
use rusqlite::Connection;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use utils::exports::anyhow::{anyhow, Result};

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
pub struct Vfs {
    pub data: Vec<Vec<u8>>,
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
    pub fn get(&self, path: FsPath) -> Inode {
        todo!()
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
            Some(p) => {
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
                //add the dinode to the inode list
                self.inodes.insert(dinode.id, Inode::Dir(dinode));

                //return
                return Ok(());
            }
        }

        todo!()
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
    pub struct TestStruct {
        pub id: u32,
        pub name: String,
        pub description: String,
    }

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
    fn test_inode_create() {}
}
