#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_assignments)]

use nvcore::{
    ecs::{component::components::*, ComponentId, Entity, Id},
    mir::Mir,
};
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
        let field_comp = nvcore::ecs::component::components::StringFieldComponent {
            name: field_name,
            value: field_value,
        };
        self.mir.add_component(entity.into(), field_comp)
    }
    pub fn get_all_living_entities(&self) -> Vec<ffi::Id> {
        self.mir
            .get_all_living_entities()
            .iter()
            .map(|id| ffi::Id::from_internal_id(*id))
            .collect()
    }
    pub fn get_field_component_with_id(&self, field_id: ffi::Id) -> *mut ffi::StringFieldComponent {
        let field_comp = self
            .mir
            .get_component_with_id::<StringFieldComponent>(field_id.into())
            .unwrap();
        //convert to raw pointer
        let field_comp_ptr = std::ptr::addr_of!(field_comp.component);
        //#We know that the memory layout ought to be the same as the C++ struct
        unsafe {
            std::mem::transmute::<*const StringFieldComponent, *mut ffi::StringFieldComponent>(
                field_comp_ptr,
            )
        }
    }
    pub fn get_name_component_with_id(&self, name_id: ffi::Id) -> *mut ffi::NameComponent {
        let name_comp = self
            .mir
            .get_component_with_id::<NameComponent>(name_id.into())
            .unwrap();
        //convert to raw pointer
        let name_comp_ptr = std::ptr::addr_of!(name_comp.component);
        //#We know that the memory layout ought to be the same as the C++ struct
        unsafe {
            std::mem::transmute::<*const NameComponent, *mut ffi::NameComponent>(name_comp_ptr)
        }
    }
    //Video component
    pub fn get_binary_component_with_id(&self, video_id: ffi::Id) -> *mut ffi::BinaryData {
        // let video_comp = self
        //     .mir
        //     .get_component_with_id::<BinaryData>(video_id.into())
        //     .unwrap();
        // //convert to raw pointer
        // let video_comp_ptr = std::ptr::addr_of!(video_comp.component);
        // //#We know that the memory layout ought to be the same as the C++ struct
        // unsafe { std::mem::transmute::<*const BinaryData, *mut ffi::BinaryData>(video_comp_ptr) }
        todo!()
    }
}
pub fn new_ctx() -> *mut ContextInternal {
    Box::into_raw(Box::new(ContextInternal { mir: Mir::new() }))
}
//Safety
//There should be only reference to the context
pub unsafe fn drop(ctx: *mut ContextInternal) {
    Box::from_raw(ctx);
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
impl From<ffi::Id> for ComponentId {
    fn from(id: ffi::Id) -> Self {
        ComponentId::from(Id::from(id))
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
    pub enum ComponentTypes {
        Field,
        Name,
        Video,
        Audio,
        Image,
    }

    #[namespace = "components"]
    pub struct StringFieldComponent {
        pub name: String,
        pub value: String,
    }

    #[namespace = "components"]
    pub struct BinaryData {
        description: String,
        video_name: String,
        video_type: String,
        video_data: String,
    }

    pub struct NameComponent {
        pub name: String,
        pub aliases: Vec<String>,
    }
    #[namespace = "components"]
    struct CharacterNameFormat {
        pub given_name: String,
        pub other_names: Vec<String>,
        pub family_name: String,
    }
    #[namespace = "components"]
    pub struct CharacterNameComponent {
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
        pub fn get_all_living_entities(&self) -> Vec<Id>;
        pub unsafe fn drop(ctx: *mut ContextInternal);
        pub fn get_field_component_with_id(&self, field_id: Id) -> *mut StringFieldComponent;

    }
}
