use std::collections::VecDeque;

use crate::action::request::Reqman;
use crate::action::Actman;
use crate::ecs::archetypes;
use crate::ecs::ComponentTy;
use crate::ecs::Entity;
use crate::ecs::Entman;
use crate::ecs::Id;
use crate::Project;

pub struct MirData {
    pub em: Entman,
    pub proj: Project,
}
#[repr(C)]
pub struct Mir {
    pub data: MirData,
}
impl Mir {
    pub fn new() -> Self {
        let data = MirData {
            em: Entman::new(),
            proj: Project::new_empty(),
        };
        let mut m = Mir { data };
        // m.actman = Some(Actman::new(&mut m.data));
        // m.reqman = Some(Reqman::new(&mut m.data));
        m
    }
    //adds an entity
    pub fn add_entity(&mut self) -> Id {
        self.data.em.add_entity()
    }
    pub fn add_component<T: ComponentTy>(&mut self, entity: Id, component: T) {
        self.data.em.add_component(entity, component);
    }
    pub fn add_archetype<'a, T: archetypes::ArchetypeTy<'a>>(&mut self, entity: Id, archetype: T) {
        todo!()
    }

    pub fn create_project(&mut self, name: String, desc: String) {
        self.data.proj.project_meta_data.name = name;
        self.data.proj.description = desc;
    }
    pub fn get_entity_count(&self) -> usize {
        self.data.em.get_entity_count()
    }
    pub fn get_entity(&self, id: Id) -> Entity {
        self.data.em.get_entity_clone(id)
    }
}
