use std::collections::BTreeSet;

use super::components::*;
use super::prelude;
use super::Component;
use super::EntityGraph;
use super::EntityManager;
use super::Field;

pub trait ArchetypeFactory {
    fn create_archetype(&self) -> EntityGraph;
}

pub struct Character {
    signature: prelude::EntitySignature,
    name: String,
    gender: String,
    bio: String,
    age: String,
    dob: String,
    pob: String,
}

impl Character {
    pub fn new(
        name: String,
        gender: String,
        bio: String,
        age: String,
        dob: String,
        pob: String,
    ) -> Character {
        Self {
            signature: Default::default(),
            name,
            gender,
            bio,
            age,
            dob,
            pob,
        }
    }
}
impl ArchetypeFactory for Character {
    fn create_archetype(&self) -> EntityGraph {
        let signature =
            prelude::EntitySignature::from([ComponentType::Names, ComponentType::Fields]);
        let mut entity = prelude::Entity::new("Character");
        entity.replace_signature(signature);
        let mut name = Names::new(
            entity.id(),
            NamesProp {
                name: vec!["Bob".to_string()],
            },
        );
        let mut fields = Fields::new(
            entity.id(),
            FieldsProp {
                fields: vec![
                    Field {
                        name: "Gender".to_string(),
                        value: self.gender.clone(),
                    },
                    Field {
                        name: "Biography".into(),
                        value: self.bio.clone(),
                    },
                    Field {
                        name: "Age".to_string(),
                        value: self.age.clone(),
                    },
                    Field {
                        name: "Birthdate".to_string(),
                        value: self.dob.clone(),
                    },
                    Field {
                        name: "Birthplace".to_string(),
                        value: self.pob.clone(),
                    },
                ],
            },
        );
        let mut egraph: EntityGraph = Default::default();
        egraph
            .components
            .get_mut::<Names>()
            .insert(entity.id(), name);
        egraph.entities.push(entity);

        egraph
    }
}

pub struct Place {}
