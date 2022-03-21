/*!  /[Nvfs] is the Novella Virtual File System. It is used to serialize and deserialize a .nv project.
 * It is designed to be able to store and retrieve data from a .nv file, and to stream data from disk to memory, as needed.
 * It based on a sqlite database.
*/
use rusqlite::Connection;
use std::path::{Path, PathBuf};
use utils::prelude::anyhow::{anyhow, Result};

pub enum FsPath {
    ///The data is stored inside of a .nv file
    Internal(PathBuf),
    ///The data is stored in a file on the disk, and is accessed via a path
    File(PathBuf),
}
pub enum Inode<'a> {
    Dir(DinodeData<'a>),
    Item(InodeData),
    Uninit,
}
pub enum ExtTypes {
    Video,
    Audio,
    Image,
    Binary,
    Component,
    String(String),
}
pub struct InodeData {
    pub id: u64,
    pub parent_id: u64,
    pub ext: ExtTypes,
    pub name: String,
    pub data: u64,
}
pub struct DinodeData<'a> {
    pub id: u64,
    pub parent_id: u64,
    pub ext: ExtTypes,
    pub name: String,
    pub data: Vec<&'a Inode<'a>>,
}
pub struct Vfs<'a> {
    pub data: Vec<Vec<u8>>,
    pub inodes: Inode<'a>,
}

impl<'a> Vfs<'a> {
    pub fn new() -> Self {
        Vfs {
            data: Vec::new(),
            inodes: Inode::Dir(DinodeData {
                id: 0,
                parent_id: 0,
                ext: ExtTypes::String("root".to_string()),
                name: "nv_root".to_string(),
                data: Vec::new(),
            }),
        }
    }
    pub fn get(&self, path: FsPath) -> Inode {
        todo!()
    }
    fn path_to_inode(&self, path: PathBuf) -> Inode {
        let mut inode = Inode::Uninit;
        for x in path.iter() {}
        todo!()
    }
    pub fn create_dir(&mut self, path: FsPath) {}
}

impl<'a> Default for Vfs<'a> {
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
