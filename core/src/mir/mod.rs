use std::collections::VecDeque;

use crate::ecs::archetypes;
use crate::ecs::bevy_ecs as bv;
use crate::ecs::EntityManager;
use crate::Project;

pub struct Mir {
    pub em: EntityManager,
    pub proj: Project,
}
impl Mir {
    pub fn new() -> Self {
        Mir {
            em: EntityManager::new(),
            proj: Project::new_empty(),
        }
    }
    //adds an entity
    // pub fn add_entity(&mut self, class: String) -> bv::entity::Entity {
    //     self.em.add_entity(class)
    // }
    // pub fn add_component<T: bv::component::Component>(
    //     &mut self,
    //     entity: bv::entity::Entity,
    //     component: T,
    // ) {
    //     self.em.add_component(entity, component);
    // }
    // pub fn add_archetype<T: archetypes::Archetype>(
    //     &mut self,
    //     entity: bv::entity::Entity,
    //     archetype: T,
    // ) {
    //     self.em.add_archetype(entity, archetype);
    // }

    // pub fn add_bundle<T: bv::bundle::Bundle>(&mut self, entity: bv::entity::Entity, bundle: T) {
    //     self.em.add_bundle(entity, bundle);
    // }
    // pub fn create_project(&mut self, name: String, desc: String) {
    //     self.proj.name = name;
    //     self.proj.description = desc;
    // }
}

pub trait Returnable {}
pub trait Event {
    fn exec(&self, mir: &mut Mir);
}
pub struct EventQueue {
    pub events: VecDeque<Box<dyn Event>>,
}
impl EventQueue {
    fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }
    fn add_event(&mut self, event: impl Event + 'static) {
        self.events.push_back(Box::new(event));
    }
}
