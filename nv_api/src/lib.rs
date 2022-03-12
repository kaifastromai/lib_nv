use nvcore::{ecs::Id, mir::Mir};

pub struct ContextInternal {
    pub mir: Mir,
}
impl ContextInternal {
    pub fn create_project(&mut self, name: String, desc: String) {
        self.mir.create_project(name, desc);
    }
    pub fn add_entity(&mut self) -> String {
        //convert to string
        self.mir.add_entity().to_string()
    }
    pub fn add_field_component(&mut self, entity: String, field_name: String, field_value: String) {
        use std::str::FromStr;
        let entity_id = Id::from_str(&entity).unwrap();
        let field_comp = nvcore::ecs::component::Field {
            name: field_name,
            value: field_value,
        };
        self.mir.add_component(entity_id, field_comp)
    }
}
pub fn new_ctx() -> *mut ContextInternal {
    Box::into_raw(Box::new(ContextInternal { mir: Mir::new() }))
}
#[cxx::bridge]
mod ffi {
    #[namespace = "nvr"]
    extern "Rust" {
        type ContextInternal;
        pub fn new_ctx() -> *mut ContextInternal;
        pub fn create_project(&mut self, name: String, desc: String);
        pub fn add_entity(&mut self) -> String;
        pub fn add_field_component(
            &mut self,
            entity: String,
            field_name: String,
            field_value: String,
        );

    }
}
