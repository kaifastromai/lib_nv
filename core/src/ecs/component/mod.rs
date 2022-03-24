pub mod archetypes;
pub mod relationship;
use common::exports::serde::*;
use common::type_id::*;

use std::{
    default,
    path::{Path, PathBuf},
};

use nvproc::{component, Component};

pub struct ArchetypeComponent {
    pub archetype_name: String,
}

pub trait BinaryTy {
    fn to_bytes(&self) -> Vec<u8>;
}

#[component]
pub struct StringField {
    pub name: String,
    pub value: String,
}
#[component]
pub struct NumericalField {
    pub name: String,
    pub value: f32,
}
#[component]
pub struct Video {
    description: String,
    video_name: String,
    video_type: String,
    video_data: PathBuf,
}
#[component]
pub struct Audio {
    description: String,
    audio_name: String,
    audio_type: String,
    audio_data: PathBuf,
}
#[component]
pub struct Image {
    name: String,
    description: String,
    image_data: PathBuf,
}

#[nvproc::serde_derive]
pub enum EGender {
    Male,
    Female,
    Custom(String),
}
//implement default for EGender
impl Default for EGender {
    fn default() -> Self {
        Self::Male
    }
}

#[component]
pub struct Age {
    age: f32,
}
#[component]
pub struct Gender {
    pub gender: EGender,
}

#[component]
pub struct Name {
    pub name: String,
    pub aliases: Vec<String>,
}
#[derive(Default)]
#[nvproc::serde_derive]
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
