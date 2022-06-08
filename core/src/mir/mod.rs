use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::RwLock;

use crate::action::request::Reqman;
use crate::action::ActionTy;
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
use common::exports::*;

impl bincode::Encode for Mir {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        self.proj.encode(encoder)?;
        self.em.encode(encoder)?;
        Ok(())
    }
}
impl bincode::Decode for Mir {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let proj = Project::decode(decoder)?;
        let em = Entman::decode(decoder)?;
        Ok(Mir {
            proj,
            em,
            actman: Aarc::new(),
            reqman: Reqman::new(),
        })
    }
}
//A Arc wrapper around the actman 
pub struct Aarc(pub Arc<RwLock<Actman>>);
impl Aarc {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(Actman::new())))
    }
}
impl Clone for Aarc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl From<Actman> for Aarc {
    fn from(actman: Actman) -> Self {
        Self(Arc::new(RwLock::new(actman)))
    }
}
//implement deref for Marc
impl std::ops::Deref for Aarc {
    type Target = Arc<RwLock<Actman>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub struct Mir {
    pub proj: Project,
    pub em: Entman,
    reqman: Reqman,
    actman: Aarc,
}
impl Mir {
    pub fn new() -> Self {
        Mir {
            proj: Project::new_empty(),
            em: Entman::new(),
            reqman: Reqman::new(),
            actman: Aarc::new(),
        }
    }
    pub fn set_actman(&mut self, actman: Aarc) {
        self.actman = actman;
    }
    pub fn new_with_actman(actman: Actman) -> Self {
        Mir {
            proj: Project::new_empty(),
            em: Entman::new(),
            reqman: Reqman::new(),
            actman: actman.into(),
        }
    }
    ///Execute the latest action in the queue
    pub fn progress(&mut self) -> Result<()> {
        //create a new shared reference
        let mut am = self.actman.clone();
        let mut rw = am.write().unwrap();
        rw.progress(self);
        Ok(())
    }

    pub fn regress(&mut self) -> Result<()> {
        //create a new shared reference
        let mut am = self.actman.clone();
        let mut rw = am.write().unwrap();
        rw.regress(self);
        Ok(())
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
    ) -> Id {
        self.em.entity_from_archetype(archetype)
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
    pub fn get_component_ref<T: ComponentTyReqs>(
        &self,
        entity: Id,
    ) -> Result<&crate::ecs::Component<T>> {
        self.em.get_component_ref(entity)
    }
    pub fn get_component_by_id_ref<T: ComponentTyReqs>(
        &self,
        id: ComponentId,
    ) -> Result<&crate::ecs::Component<T>> {
        self.em.get_component_by_id_ref(id)
    }
    pub fn load_from_file(path: &str) -> Result<Mir> {
        let mut br = BufReader::new(File::open(path)?);
        let mir: Mir = bincode::decode_from_reader(br, bincode::config::standard())?;
        Ok(mir)
    }
    pub fn register_action<T: ActionTy + 'static>(&mut self, action: T) {
        let mut am = self.actman.write().unwrap();
        am.register_action(action)
    }
}
#[cfg(test)]
mod test_mir {
    use crate::ProjectMetaData;

    use super::*;

    #[test]
    fn test_serde() {
        let mut mir = Mir::new();
        mir.proj.set_meta_data(ProjectMetaData {
            name: "test_name".to_string(),
            author: "test_author".to_string(),
            description: "test_description".to_string(),
            version: "test_version".to_string(),
            time_meta: crate::TimeMetaData::new(),
        });

        let res = bincode::encode_to_vec(mir, bincode::config::standard()).unwrap();
        let mir2: Mir = bincode::decode_from_slice(&res, bincode::config::standard())
            .unwrap()
            .0;
        assert_eq!(mir2.proj.project_meta_data.name, "test_name");
    }
}
