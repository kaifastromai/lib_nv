use std::collections::VecDeque;

use crate::action::request::Reqman;
use crate::action::Actman;
use crate::ecs::component::archetypes;
use crate::ecs::ComponentId;
use crate::ecs::ComponentTy;
use crate::ecs::ComponentTyReqs;
use crate::ecs::Entity;
use crate::ecs::Entman;
use crate::ecs::Id;
use crate::Project;
use common::exports::anyhow::{anyhow, Result};

pub struct Mir<'a> {
    pub proj: Project,
    pub em: Entman,
    reqman: Reqman,
    actman: Actman<'a>,
}
impl<'a> Mir<'a> {
    pub fn new() -> Self {
        Mir {
            proj: Project::new_empty(),
            em: Entman::new(),
            reqman: Reqman::new(),
            actman: Actman::new(),
        }
    }
    //adds an entity
    pub fn add_entity(&mut self) -> Id {
        self.em.add_entity()
    }
    pub fn add_component<T: ComponentTy + common::exports::serde::Serialize + Clone>(
        &mut self,
        entity: Id,
        component: T,
    ) {
        self.em.add_component(entity, component);
    }
    pub fn add_archetype<T: crate::ecs::component::archetypes::ArchetypeTy>(
        &mut self,
        entity: Id,
        archetype: T,
    ) {
        todo!()
    }

    pub fn create_project(&mut self, name: String, desc: String) {
        self.proj.project_meta_data.name = name;
        self.proj.description = desc;
    }
    pub fn get_entity_count(&self) -> usize {
        self.em.get_entity_count()
    }
    pub fn get_entity(&self, id: Id) -> Entity {
        self.em.get_entity_clone(id)
    }
    pub fn get_all_living_entities(&self) -> Vec<Id> {
        self.em.get_all_living_entities()
    }
    pub fn get_entity_component_by_id<T: ComponentTyReqs>(
        &self,
        entity: Id,
        component_id: ComponentId,
    ) -> Result<&crate::ecs::Component<T>> {
        self.em.get_entity_component_by_id(entity, component_id)
    }
    pub fn get_component_with_id<T: ComponentTyReqs>(
        &self,
        component_id: ComponentId,
    ) -> Result<&crate::ecs::Component<T>> {
        self.em.get_component_with_id(component_id)
    }
}
