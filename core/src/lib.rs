#![feature(min_specialization)]
#![allow(dead_code, unused_imports, unused_assignments, warnings)]
#![feature(generic_arg_infer)]
#![feature(trait_upcasting)]

use chrono::{DateTime, Utc};
use std::collections::HashMap;
mod action;
pub mod binary_storage;
pub mod ecs;
pub mod map;
pub mod mir;
use common::exports::serde::*;
use common::exports::*;
use common::{
    exports::anyhow::{anyhow, Result},
    text::TextChunk,
    uuid,
};
use ecs::{Entman, Id};

///A [Note] represents a note that can be created by the user.

#[nvproc::bincode_derive]
pub struct Note {
    id: Id,
    pub name: String,
    pub description: String,
    pub note: String,
    pub involved_entities: Vec<Id>,
    time_meta: TimeMetaData,
}
#[nvproc::bincode_derive]

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
    pub fn new(name: String, description: String, text: String, ordering: u32) -> Self {
        Progression {
            id: common::uuid::gen_128(),
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

#[nvproc::bincode_derive]

pub struct Manuscript {
    id: Id,
    name: String,
    description: String,
    progressions: HashMap<Id, Progression>,
}
impl Manuscript {
    pub fn new(name: String, description: String) -> Self {
        Manuscript {
            id: common::uuid::gen_128(),
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
//impl Hash for Manuscript based on id
impl std::hash::Hash for Manuscript {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl bincode::Encode for TimeMetaData {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        let date_string_creation = self.creation_date.timestamp();
        let date_string_last_modified = self.last_modified_date.timestamp();
        date_string_last_modified.encode(encoder)?;
        date_string_creation.encode(encoder)?;
        Ok(())
    }
}
impl bincode::Decode for TimeMetaData {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let date_string_creation = i64::decode(decoder)?;
        let date_string_last_modified = i64::decode(decoder)?;
        let creation_date = DateTime::<Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(date_string_creation, 0),
            Utc,
        );
        let last_modified_date = DateTime::<Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(date_string_last_modified, 0),
            Utc,
        );
        Ok(TimeMetaData {
            creation_date,
            last_modified_date,
        })
    }
}

#[derive(Clone)]
pub struct TimeMetaData {
    creation_date: DateTime<Utc>,
    last_modified_date: DateTime<Utc>,
}
impl TimeMetaData {
    pub fn new() -> Self {
        TimeMetaData {
            creation_date: Utc::now(),
            last_modified_date: Utc::now(),
        }
    }
    pub fn get_creation_date(&self) -> DateTime<Utc> {
        self.creation_date
    }
    pub fn get_last_modified_date(&self) -> DateTime<Utc> {
        self.last_modified_date
    }
    pub fn set_last_modified_date(&mut self, date: DateTime<Utc>) {
        self.last_modified_date = date;
    }
}
#[nvproc::bincode_derive]

pub struct ProjectMetaData {
    author: String,
    description: String,
    name: String,
    version: String,
    time_meta: TimeMetaData,
}
impl ProjectMetaData {
    pub fn new() -> Self {
        ProjectMetaData {
            author: String::new(),
            description: String::new(),
            name: String::new(),
            version: String::new(),
            time_meta: TimeMetaData::new(),
        }
    }
}
#[nvproc::bincode_derive]
pub struct Project {
    pub id: Id,
    pub project_meta_data: ProjectMetaData,
    pub description: String,
    pub manuscripts: HashMap<Id, Manuscript>,
    pub scenes: HashMap<Id, Scene>,
    pub arcs: HashMap<Id, WorldArc>,
    pub timelines: HashMap<Id, Timeline>,
    pub notes: HashMap<Id, Note>,
}
impl Project {
    pub fn new(description: &str) -> Self {
        Project {
            id: uuid::gen_128(),
            project_meta_data: ProjectMetaData::new(),
            description: String::from(description),
            manuscripts: HashMap::new(),
            scenes: HashMap::new(),
            arcs: HashMap::new(),
            timelines: HashMap::new(),
            notes: HashMap::new(),
        }
    }
    pub fn new_empty() -> Self {
        Project {
            id: uuid::gen_128(),
            project_meta_data: ProjectMetaData::new(),
            description: String::new(),
            manuscripts: HashMap::new(),
            scenes: HashMap::new(),
            arcs: HashMap::new(),
            timelines: HashMap::new(),
            notes: HashMap::new(),
        }
    }
    pub fn set_meta_data(&mut self, meta_data: ProjectMetaData) {
        self.project_meta_data = meta_data;
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
}

///An [Event] represents a mostly singular point in the narrative with a well defined time and place.
#[nvproc::bincode_derive]
#[nvproc::serde_derive]
pub struct Event {
    id: Id,
    pub name: String,
    pub location: String,
    pub description: String,
    involved_entities: Vec<Id>,
}

///An [WorldArc] is a series of events that involve many [Entity]s.
#[nvproc::bincode_derive]
#[nvproc::serde_derive]
pub struct WorldArc {
    id: Id,
    pub name: String,
    pub description: String,
    pub events: Vec<Event>,
}
#[nvproc::bincode_derive]

pub struct Timeline {}
#[nvproc::bincode_derive]

pub struct Arc {}
#[nvproc::bincode_derive]

pub struct Scene {
    pub id: Id,
    pub name: String,
    pub description: String,
}

///A simple generic system for tracking time
pub mod khronos;

#[cfg(test)]
mod test;
