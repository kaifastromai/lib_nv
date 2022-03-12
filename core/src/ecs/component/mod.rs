pub mod relationship;

use std::path::{Path, PathBuf};

use nvproc::Component;

pub struct ArchetypeComponent {
    pub archetype_name: String,
}

pub trait BinaryTy {
    fn to_bytes(&self) -> Vec<u8>;
}
#[derive(Component)]
pub struct BinaryData {
    pub data: Vec<Box<dyn BinaryTy + Send + Sync>>,
}
#[derive(Component, Default)]
pub struct Field {
    pub name: String,
    pub value: String,
}

#[derive(Component, Default)]
pub struct Video {
    description: String,
    video_name: String,
    video_type: String,
    video_data: PathBuf,
}
#[derive(Component, Default)]
pub struct Audio {
    description: String,
    audio_name: String,
    audio_type: String,
    audio_data: PathBuf,
}
#[derive(Component, Default)]
pub struct Image {
    name: String,
    description: String,
    image_data: PathBuf,
}

#[derive(Component, Default)]
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

#[derive(Component, Default)]
pub struct CharacterName {
    pub name: CharacterNameFormat,
    pub aliases: Vec<String>,
}
#[derive(Component, Default)]
pub struct BinaryDatum {
    data: PathBuf,
}
