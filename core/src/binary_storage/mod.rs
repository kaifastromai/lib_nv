/*! [BinaryStorage] manages the storage and streaming of binary data (audio, video, images, large manuscripts, etc...)
Because there is potentially a lot of data to be stored, (all of which cannot be kept in memory at once),
 the BinaryStorage is designed to be able to stream data from disk to memory,as needed.
*/

use crate::ecs::{component::*, ComponentTy, Id};
use utils::exports::serde;
use utils::uuid;

pub struct VirtualPath(PathBuf);
pub trait BinaryStorageTy: Serialize {
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Serialize, Deserialize)]
#[serde(crate="utils::exports::serde")]
pub struct BinaryFileItem<T: BinaryStorageTy> {
    pub id: Id,
    pub path: PathBuf,
    pub data: T,
}
#[derive(Serialize, Deserialize)]
#[serde(crate="utils::exports::serde")]
pub struct BinaryInternalItem<T: BinaryStorageTy> {
    pub id: Id,
    pub data: T,
}
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
pub enum StorageKind {
    ///The data is stored inside of a .nv file
    Internal(InternalStorage),
    ///The data is stored in a file on the disk, and is accessed via a path
    File(PathBuf),
}
pub trait StorageTy {
    fn fetch(&self, id: Id) -> Option<Vec<u8>>;
}
pub struct BinaryStorage {
    //The path to the directory where the binary data is stored
    pub path: PathBuf,
}


#[derive(Serialize, Deserialize)]
#[serde(crate="utils::exports::serde")]
pub struct InternalStorage {}
