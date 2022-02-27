pub mod archetypes;
mod ecs;
pub mod prelude;
use bevy_ecs::prelude::*;

use super::*;
pub use serde::{Deserialize, Serialize};
use utils::prelude::*;

type Entity = bevy_ecs::entity::Entity;
pub extern crate bevy_ecs;
#[derive(serde::Serialize, serde::Deserialize, Clone, Component)]
pub struct NVEntity {
    pub class: String,
    pub is_deleted: bool,
}
#[derive(Component)]
pub struct Field {
    name: String,
    value: String,
}
pub struct Video {
    description: String,
    video_name: String,
    video_type: String,
    video_data: Vec<u8>,
}
pub trait BinaryTy {
    fn to_bytes(&self) -> Vec<u8>;
}
pub struct BinaryData {
    pub data: Vec<Box<dyn BinaryTy + Send + Sync>>,
}
pub struct Audio {
    description: String,
    audio_name: String,
    audio_type: String,
    audio_data: Vec<u8>,
}
pub struct Image {
    name: String,
    description: String,
    image_data: Vec<u8>,
}
pub struct BinaryDatum {
    data: Vec<u8>,
}

pub struct Entman {
    world: bevy_ecs::world::World,
}
impl Entman {
    pub fn new() -> Self {
        Entman {
            world: bevy_ecs::world::World::new(),
        }
    }
    pub fn add_entity(&mut self, class: String) -> Entity {
        todo!()
    }
    pub fn remove_entity(&mut self, entity: Entity) {
        self.world.get_entity_mut(entity).unwrap().despawn();
    }
    pub fn add_component<T: bevy_ecs::component::Component>(
        &mut self,
        entity: Entity,
        component: T,
    ) {
        todo!()
    }
    pub fn add_bundle<T: Bundle>(&mut self, entity: Entity, bundle: T) {
        todo!()
    }
    pub fn add_archetype<T: archetypes::Archetype>(&mut self, entity: Entity, archetype: T) {}
    pub fn get_entity(&self, entity: Entity) -> Option<bevy_ecs::world::EntityRef> {
        todo!()
    }
    pub fn get_entity_mut(&mut self, entity: Entity) -> Option<bevy_ecs::world::EntityMut> {
        todo!()
    }
    pub fn get_entity_count(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod test;
