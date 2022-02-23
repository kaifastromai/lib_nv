pub mod archetypes;
pub mod prelude;
use bevy_ecs::prelude::*;

use super::*;
pub use serde::{Deserialize, Serialize};
use utils::prelude::*;

pub extern crate bevy_ecs;
#[derive(serde::Serialize, serde::Deserialize, Clone, Component)]
pub struct NVEntity {
    pub class: String,
}
pub struct Field {
    name: String,
    value: String,
}
#[derive(Component)]
pub struct Video {
    description: String,
    video_name: String,
    video_type: String,
    video_data: Vec<u8>,
}
#[derive(Component)]
pub struct Audio {
    description: String,
    audio_name: String,
    audio_type: String,
    audio_data: Vec<u8>,
}
#[derive(Component)]
pub struct Image {
    name: String,
    description: String,
    image_data: Vec<u8>,
}
#[derive(Component)]
pub struct BinaryDatum {
    data: Vec<u8>,
}

pub struct EntityManager {
    world: bevy_ecs::world::World,
}
impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            world: bevy_ecs::world::World::new(),
        }
    }
    pub fn add_entity(&mut self, class: String) -> Entity {
        self.world.spawn().insert(NVEntity { class }).id()
    }
    pub fn remove_entity(&mut self, entity: Entity) {
        self.world.get_entity_mut(entity).unwrap().despawn();
    }
    pub fn add_component<T: bevy_ecs::component::Component>(
        &mut self,
        entity: Entity,
        component: T,
    ) {
        self.world.get_entity_mut(entity).unwrap().insert(component);
    }
    pub fn add_bundle<T: Bundle>(&mut self, entity: Entity, bundle: T) {
        self.world
            .get_entity_mut(entity)
            .unwrap()
            .insert_bundle(bundle);
    }
    pub fn add_archetype<T: archetypes::Archetype>(&mut self, entity: Entity, archetype: T) {
        archetype.generate(&mut self.world);
    }
    pub fn get_entity(&self, entity: Entity) -> Option<bevy_ecs::world::EntityRef> {
        self.world.get_entity(entity)
    }
    pub fn get_entity_mut(&mut self, entity: Entity) -> Option<bevy_ecs::world::EntityMut> {
        self.world.get_entity_mut(entity)
    }
    pub fn get_entity_count(&self) -> usize {
        self.world.entities().len() as usize
    }
}

#[cfg(test)]
mod test;
