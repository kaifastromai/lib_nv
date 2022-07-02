use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;

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
use common::exports::*;

impl<'a> bincode::Encode for Mir<'a> {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        self.proj.encode(encoder)?;
        self.em.encode(encoder)?;
        Ok(())
    }
}
impl<'a> bincode::Decode for Mir<'a> {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let proj = Project::decode(decoder)?;
        let em = Entman::decode(decoder)?;
        Ok(Mir {
            proj,
            em,
            actman: Actman::new(),
            reqman: Reqman::new(),
        })
    }
}
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

    pub fn create_project(&mut self, name: String, desc: String) {
        self.proj.project_meta_data.name = name;
        self.proj.description = desc;
    }
    ///Run any function or closure on Mir
    pub fn exec<F: Fn(&mut Mir) -> R, R>(&mut self, f: F) -> R {
        f(self)
    }

    pub fn load_from_file(path: &str) -> Result<Mir> {
        let mut br = BufReader::new(File::open(path)?);
        let mir: Mir = bincode::decode_from_reader(br, bincode::config::standard())?;
        Ok(mir)
    }
}
#[cfg(test)]
mod test_mir {
    use crate::ProjectMetaData;

    use super::*;

    #[test]
    fn test_serde() {
        let mut mir = Mir::new();
        mir.proj.set_metadata(ProjectMetaData {
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
