#![allow(dead_code, unused_imports, unused_assignments, warnings)]

use std::collections::HashMap;
use uuid::Uuid;

type IndexType = u128;

pub mod ecs;
use ecs::EntityManager;
pub trait MapType {
    fn get_map(&self) -> &dyn MapType;
}
pub struct Point<T: num::Integer> {
    x: T,
    y: T,
}
pub struct MapPointOfInterest {
    point: Point<i32>,
    name: &'static str,
    description: TextChunk,
}
pub struct MapTypeImage {}
pub struct Map<T> {
    _entity: Option<IndexType>,
    description: TextChunk,
    map_name: &'static str,
    map_type: T,
    map_data: Vec<u8>,
}
pub struct Progression {
    id: IndexType,
    name: &'static str,
    description: &'static str,
    involved_entities: Vec<IndexType>,
    text: TextChunk,
    ordering: u32,
}
//impl Progression
impl Progression {
    pub fn new(
        id: IndexType,
        name: &'static str,
        description: &'static str,
        text: TextChunk,
        ordering: u32,
    ) -> Self {
        Progression {
            id,
            name,
            description,
            involved_entities: Vec::new(),
            text,
            ordering,
        }
    }
    pub fn add_involved_entity(&mut self, entity: IndexType) {
        self.involved_entities.push(entity);
    }
}
//impl PartialEq and Hash for Progression based on entity id
impl PartialEq for Progression {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
//impl Hash for Progression based on entity id
impl std::hash::Hash for Progression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub struct Manuscript {
    id: IndexType,
    name: &'static str,
    description: &'static str,
    progressions: HashMap<IndexType, Progression>,
}
impl Manuscript {
    pub fn new(id: IndexType, name: &'static str, description: &'static str) -> Self {
        Manuscript {
            id,
            name,
            description,
            progressions: HashMap::new(),
        }
    }
    pub fn add_progression(&mut self, progression: Progression) {
        self.progressions.insert(progression.id, progression);
    }
    pub fn get_progression(&self, progression_id: IndexType) -> Option<&Progression> {
        self.progressions.get(&progression_id)
    }
}
//impl PartialEq and Hash for Manuscript based on entity id
impl PartialEq for Manuscript {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
//impl Hash for Manuscript based on entity id
impl std::hash::Hash for Manuscript {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

struct Project {
    name: &'static str,
    description: &'static str,
    manuscripts: HashMap<IndexType, Manuscript>,
    entity_manager: EntityManager,
}
impl Project {
    pub fn new(name: &'static str, description: &'static str) -> Self {
        Project {
            entity_manager: EntityManager::new(),
            name,
            description,
            manuscripts: HashMap::new(),
        }
    }
    pub fn add_manuscript(&mut self, manuscript: Manuscript) {
        self.manuscripts.insert(manuscript.id, manuscript);
    }
    pub fn get_manuscript(&self, manuscript_id: IndexType) -> Option<&Manuscript> {
        self.manuscripts.get(&manuscript_id)
    }
}
pub struct TextChunk {
    buffer: String,
}

pub struct Timeline {}
pub struct Arc {}
pub struct Scene {}

#[cfg(test)]
mod test;
