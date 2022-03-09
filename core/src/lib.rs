#![allow(dead_code, unused_imports, unused_assignments, warnings)]
#![feature(const_generics_defaults)]

use std::collections::HashMap;

pub type IndexType = u128;
pub trait Referanceable {
    fn get_ref(&self) -> Reference;
}
mod action;
pub mod ecs;
pub mod mir;
use ecs::Entman;
use utils::uuid;
pub trait MapType {
    fn get_map(&self) -> &dyn MapType;
}
pub struct Point<T: num::Integer> {
    x: T,
    y: T,
}
pub struct MapPointOfInterest {
    point: Point<i32>,
    name: String,
    description: TextChunk,
}
pub struct MapTypeImage {}
pub struct Map<T> {
    _entity: Option<IndexType>,
    description: TextChunk,
    map_name: String,
    map_type: T,
    map_data: Vec<u8>,
}
pub struct Progression {
    id: IndexType,
    name: String,
    description: String,
    involved_entities: Vec<IndexType>,
    text: TextChunk,
    ordering: u32,
}
//impl Progression
impl Progression {
    pub fn new(
        id: IndexType,
        name: String,
        description: String,
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
impl Referanceable for Progression {
    fn get_ref(&self) -> Reference {
        Reference {
            id: self.id,
            display_name: self.name.clone(),
        }
    }
}

//A manuscript contains a collection of progressions.

pub struct Manuscript {
    id: IndexType,
    name: String,
    description: String,
    progressions: HashMap<IndexType, Progression>,
}
impl Manuscript {
    pub fn new(id: IndexType, name: String, description: String) -> Self {
        Manuscript {
            id,
            name,
            description,
            progressions: HashMap::new(),
        }
    }
    pub fn add_progression(&mut self, progression: Progression) -> Result<(), String> {
        //check if we already have a progression with this ordering
        if self
            .progressions
            .iter()
            .any(|(_, p)| p.ordering == progression.ordering)
        {
            Err(format!(
                "Manuscript already has a progression with ordering {}",
                progression.ordering
            )
            .to_string())
        } else {
            self.progressions.insert(progression.id, progression);
            Ok(())
        }
    }
    pub fn get_progression(&self, progression_id: IndexType) -> Option<&Progression> {
        self.progressions.get(&progression_id)
    }
    pub fn get_progression_mut(&mut self, progression_id: IndexType) -> Option<&mut Progression> {
        self.progressions.get_mut(&progression_id)
    }
    pub fn remove_progression(&mut self, progression_id: IndexType) -> Option<Progression> {
        self.progressions.remove(&progression_id)
    }
    pub fn get_all_progressions(&self) -> Vec<&Progression> {
        self.progressions.values().collect()
    }
    pub fn get_ordered_progressions(&self) -> Vec<&Progression> {
        let mut sorted = self.progressions.values().collect::<Vec<&Progression>>();
        sorted.sort_by(|a, b| a.ordering.cmp(&b.ordering));
        sorted
    }
}
impl Referanceable for Manuscript {
    fn get_ref(&self) -> Reference {
        Reference {
            id: self.id,
            display_name: self.name.clone(),
        }
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

pub struct Project {
    pub id: IndexType,
    pub name: String,
    pub description: String,
    pub manuscripts: HashMap<IndexType, Manuscript>,
}
impl Project {
    pub fn new(name: &str, description: &str) -> Self {
        Project {
            id: uuid::generate(),
            name: String::from(name),
            description: String::from(description),
            manuscripts: HashMap::new(),
        }
    }
    pub fn new_empty() -> Self {
        Project {
            id: uuid::generate(),
            name: String::new(),
            description: String::new(),
            manuscripts: HashMap::new(),
        }
    }
    pub fn add_manuscript(&mut self, manuscript: Manuscript) {
        self.manuscripts.insert(manuscript.id, manuscript);
    }
    pub fn get_manuscript(&self, manuscript_id: IndexType) -> Option<&Manuscript> {
        self.manuscripts.get(&manuscript_id)
    }
    pub fn get_manuscript_mut(&mut self, manuscript_id: IndexType) -> Option<&mut Manuscript> {
        self.manuscripts.get_mut(&manuscript_id)
    }
    pub fn remove_manuscript(&mut self, manuscript_id: IndexType) -> Option<Manuscript> {
        self.manuscripts.remove(&manuscript_id)
    }
    pub fn get_all_manuscripts(&self) -> Vec<&Manuscript> {
        self.manuscripts.values().collect()
    }
    pub fn get_all_live_references(&self) -> Vec<Reference> {
        let mut references = Vec::new();
        //add all progression references
        for manuscript in self.get_all_manuscripts() {
            for progression in manuscript.get_all_progressions() {
                references.push(Reference {
                    display_name: progression.name.clone(),
                    id: progression.id,
                })
            }
        }
        //add all manuscript references
        for manuscript in self.get_all_manuscripts() {
            references.push(manuscript.get_ref());
        }

        references
    }
}

//A reference to any object that implements Referenceable
pub struct Reference {
    pub display_name: String,
    pub id: IndexType,
}
pub struct TextChunk {
    buffer: String,
}

pub struct Timeline {}
pub struct Arc {}
pub struct Scene {
    pub id: IndexType,
    pub name: String,
    pub description: String,
}

#[cfg(test)]
mod test;
