use std::collections::VecDeque;

use crate::action::request::Reqman;
use crate::action::Actman;
use crate::ecs::Id;
use crate::ecs::archetypes;
use crate::ecs::ComponentTy;
use crate::ecs::Entity;
use crate::ecs::Entman;
use crate::Project;

pub struct Mir<'a> {
    pub em: Entman,
    pub proj: Project,
    pub reqman: Reqman<'a>,
    pub actman: Actman<'a>,
}
impl<'a> Mir<'a> {
    pub fn new() -> Self {
        let m = Mir {
            em: Entman::new(),
            proj: Project::new_empty(),
            reqman: todo!(),
            actman: todo!(),
        };
    }
    //adds an entity
    pub fn add_entity(&mut self, class: String) -> Id {
        self.em.add_entity(class)
    }
    pub fn add_component<T: ComponentTy>(&mut self, entity: Id, component: T) {
        self.em.add_component(entity, component);
    }
    pub fn add_archetype<T: archetypes::Archetype>(&mut self, entity: Id, archetype: T) {
        self.em.add_archetype(entity, archetype);
    }

    pub fn create_project(&mut self, name: String, desc: String) {
        self.proj.name = name;
        self.proj.description = desc;
    }
    pub fn get_entity_count(&self) -> usize {
        self.em.get_entity_count()
    }
}
