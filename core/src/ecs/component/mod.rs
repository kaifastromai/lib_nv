pub mod archetypes;
pub mod relationship;
use common::type_id::*;
use linkme::distributed_slice;
use nvproc::TypeId;

use common::components::COMPONENTS;
use std::{
    any::Any,
    path::{Path, PathBuf},
};

use nvproc::{component, Component};

use super::{ComponentTy, Id};

pub enum BinaryDataType {
    Audio,
    Video,
    Image,
    Other,
}
#[distributed_slice(COMPONENTS)]
static BINARY_DATA: &'static str = "BinaryData";
pub struct BinaryData {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub data_type: BinaryDataType,
    pub data_path: PathBuf,
}
impl ComponentTy for BinaryData {
    fn get_any(&self) -> &dyn Any {
        self
    }

    fn get_component_name(&self) -> &'static str {
        "BinaryData"
    }

    fn clean(&mut self) {
        todo!()
    }

    fn get_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
pub struct ArchetypeComponent {
    pub archetype_name: String,
}

#[component]
pub struct Field {
    pub name: String,
    pub value: String,
}

#[component]
pub struct Name {
    pub name: String,
    pub aliases: Vec<String>,
}
#[derive(Default)]
pub struct CharacterNameFormat {
    pub given_name: String,
    pub other_names: Vec<String>,
    pub family_name: String,
}

#[component]
pub struct CharacterName {
    pub name: CharacterNameFormat,
    pub aliases: Vec<String>,
}
#[component]
pub struct BinaryDatum {
    data: PathBuf,
}

#[cfg(test)]
mod test_component {
    use super::*;

    #[test]
    fn test_type_id() {}
}
