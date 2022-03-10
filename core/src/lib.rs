#![allow(dead_code, unused_imports, unused_assignments, warnings)]
#![feature(const_generics_defaults)]

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
mod action;
pub mod ecs;
pub mod mir;
use ecs::{Entman, Id};
use utils::{text::TextChunk, uuid};

pub struct Progression {
    id: Id,
    name: String,
    description: String,
    involved_entities: Vec<Id>,
    text: String,
    ordering: u32,
}
//impl Progression
impl Progression {
    pub fn new(id: Id, name: String, description: String, text: String, ordering: u32) -> Self {
        Progression {
            id,
            name,
            description,
            involved_entities: Vec::new(),
            text,
            ordering,
        }
    }
    pub fn add_involved_entity(&mut self, entity: Id) {
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

//A manuscript contains a collection of progressions.

pub struct Manuscript {
    id: Id,
    name: String,
    description: String,
    progressions: HashMap<Id, Progression>,
}
impl Manuscript {
    pub fn new(id: Id, name: String, description: String) -> Self {
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
    pub fn get_progression(&self, progression_id: Id) -> Option<&Progression> {
        self.progressions.get(&progression_id)
    }
    pub fn get_progression_mut(&mut self, progression_id: Id) -> Option<&mut Progression> {
        self.progressions.get_mut(&progression_id)
    }
    pub fn remove_progression(&mut self, progression_id: Id) -> Option<Progression> {
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

pub struct ProjectMetaData {
    author: String,
    description: String,
    name: String,
    version: String,
    creation_date: DateTime<Utc>,
    total_edit_time: chrono::Duration,
    last_edit_date: chrono::DateTime<Utc>,
}
impl ProjectMetaData {
    pub fn new() -> Self {
        ProjectMetaData {
            author: String::new(),
            description: String::new(),
            name: String::new(),
            version: String::new(),
            creation_date: Utc::now(),
            total_edit_time: chrono::Duration::zero(),
            last_edit_date: Utc::now(),
        }
    }
}
pub struct Project {
    pub id: Id,
    pub project_meta_data: ProjectMetaData,
    pub description: String,
    pub manuscripts: HashMap<Id, Manuscript>,
    pub scenes: HashMap<Id, Scene>,
    pub arcs: HashMap<Id, Arc>,
    pub timelines: HashMap<Id, Timeline>,
}
impl Project {
    pub fn new(name: &str, description: &str) -> Self {
        Project {
            id: uuid::generate(),
            project_meta_data: ProjectMetaData::new(),
            description: String::from(description),
            manuscripts: HashMap::new(),
            scenes: HashMap::new(),
            arcs: HashMap::new(),
            timelines: HashMap::new(),
        }
    }
    pub fn new_empty() -> Self {
        Project {
            id: uuid::generate(),
            project_meta_data: ProjectMetaData::new(),
            description: String::new(),
            manuscripts: HashMap::new(),
            scenes: HashMap::new(),
            arcs: HashMap::new(),
            timelines: HashMap::new(),
        }
    }
    pub fn add_manuscript(&mut self, manuscript: Manuscript) {
        self.manuscripts.insert(manuscript.id, manuscript);
    }
    pub fn get_manuscript(&self, manuscript_id: Id) -> Option<&Manuscript> {
        self.manuscripts.get(&manuscript_id)
    }
    pub fn get_manuscript_mut(&mut self, manuscript_id: Id) -> Option<&mut Manuscript> {
        self.manuscripts.get_mut(&manuscript_id)
    }
    pub fn remove_manuscript(&mut self, manuscript_id: Id) -> Option<Manuscript> {
        self.manuscripts.remove(&manuscript_id)
    }
    pub fn get_all_manuscripts(&self) -> Vec<&Manuscript> {
        self.manuscripts.values().collect()
    }

    pub fn deserialize(data: &str) -> Result<Self> {
        todo!()
    }
}

//A reference to any object that implements Referenceable
pub struct Timeline {}
pub struct Arc {}
pub struct Scene {
    pub id: Id,
    pub name: String,
    pub description: String,
}

#[cfg(test)]
mod test;
