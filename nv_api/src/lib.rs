use nvcore::{ecs::Id, mir::Mir};

pub struct ContextInternal {
    pub mir: Mir,
}
impl ContextInternal {
    pub fn create_project(&mut self, name: String, desc: String) {
        self.mir.create_project(name, desc);
    }
    pub fn add_entity(&mut self) -> ffi::Id {
        //convert to string
        ffi::Id::from_internal_id(self.mir.add_entity())
    }
    pub fn add_field_component(
        &mut self,
        entity: ffi::Id,
        field_name: String,
        field_value: String,
    ) {
        let field_comp = nvcore::ecs::component::Field {
            name: field_name,
            value: field_value,
        };
        self.mir.add_component(entity.into(), field_comp)
    }
}
pub fn new_ctx() -> *mut ContextInternal {
    Box::into_raw(Box::new(ContextInternal { mir: Mir::new() }))
}
impl ffi::Id {
    fn from_internal_id(id: Id) -> Self {
        let mem = id.to_be_bytes();
        ffi::Id { id: mem }
    }
    fn to_string(&self) -> String {
        unsafe {
            let integer: u128 = std::mem::transmute(self.id);
            //format as hex
            format!("{:x}", integer)
        }
    }
}
impl From<ffi::Id> for Id {
    fn from(id: ffi::Id) -> Self {
        let mem = id.id;
        Id::from_be_bytes(mem)
    }
}

#[cxx::bridge]
mod ffi {

    #[namespace = "nvr"]
    #[derive(PartialOrd, Ord, PartialEq, Eq)]
    struct Id {
        id: [u8; 16],
    }

    #[namespace = "components"]
    pub struct Field {
        pub name: String,
        pub value: String,
    }

    #[namespace = "components"]
    pub struct Video {
        description: String,
        video_name: String,
        video_type: String,
        video_data: String,
    }
    #[namespace = "components"]
    pub struct Audio {
        description: String,
        audio_name: String,
        audio_type: String,
        audio_data: String,
    }

    #[namespace = "components"]
    pub struct Image {
        name: String,
        description: String,
        image_data: String,
    }

    #[namespace = "components"]

    pub struct Name {
        pub name: String,
        pub aliases: Vec<String>,
    }
    #[namespace = "components"]
    pub struct CharacterNameFormat {
        pub given_name: String,
        pub other_names: Vec<String>,
        pub family_name: String,
    }
    #[namespace = "components"]
    pub struct CharacterName {
        pub name: CharacterNameFormat,
        pub aliases: Vec<String>,
    }

    #[namespace = "nvr"]
    extern "Rust" {
        type ContextInternal;

        pub fn new_ctx() -> *mut ContextInternal;
        pub fn create_project(&mut self, name: String, desc: String);
        pub fn add_entity(&mut self) -> Id;
        pub fn add_field_component(&mut self, entity: Id, field_name: String, field_value: String);
        pub fn to_string(self: &Id) -> String;

    }
}
