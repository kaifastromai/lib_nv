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
    video_data: Vec<u8>,
}
#[derive(Component, Default)]
pub struct Audio {
    description: String,
    audio_name: String,
    audio_type: String,
    audio_data: Vec<u8>,
}
#[derive(Component, Default)]
pub struct Image {
    name: String,
    description: String,
    image_data: Vec<u8>,
}
#[derive(Component, Default)]
pub struct BinaryDatum {
    data: Vec<u8>,
}
