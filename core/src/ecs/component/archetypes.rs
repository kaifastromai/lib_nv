//Archetypes map to bevy's bundles

use crate::ecs::component::components::*;
use crate::ecs::component::relationship::*;
use crate::ecs::*;
use common::exports::serde::*;
use common::exports::serde_json::*;
use nvproc::arch_sig;

use super::relationship::Relationship;
struct SigType<T: ComponentTypeIdTy> {
    phantom: std::marker::PhantomData<T>,
}
impl<T: ComponentTypeIdTy> SigType<T> {
    pub fn into_type_id(self) -> ComponentTypeId {
        ComponentTypeId {
            0: TypeId::of::<T>(),
        }
    }
}

//An archetype is an entity that has a predefined set of components
macro_rules! archetype {
    ( $($signature:tt),* ) => {};
}

pub struct ArchetypeGraph {
    pub sig: Vec<String>,
    pub name: String,
}

#[nvproc::bincode_derive]
#[nvproc::serde_derive]
pub struct ArchetypeDescriptor {
    name: String,
    signature: Signature,
    components: Vec<EComponentGraphTypes>,
}
impl ArchetypeDescriptor {
    pub fn new_empty() -> Self {
        Self {
            name: "".to_string(),
            signature: Signature::new(),
            components: Vec::new(),
        }
    }
    pub fn new(name: String, components: Vec<EComponentGraphTypes>) -> Self {
        Self {
            name,
            signature: components
                .iter()
                .map(|c| c.get_type_id_ref())
                .collect::<Vec<TypeId>>()
                .into(),
            components,
        }
    }

    pub fn with_component<T: ComponentTypeIdTy>(mut self) -> Self {
        self.signature.add(TypeId::of::<T>());
        self
    }
    pub fn take_components(self) -> Vec<EComponentGraphTypes> {
        self.components
    }
    pub fn get_signature(&self) -> Signature {
        self.signature.clone()
    }

    pub fn get_signature_ref(&self) -> &Signature {
        &self.signature
    }
}

impl<T: ComponentTypeIdTy> From<T> for SigType<T> {
    fn from(_: T) -> Self {
        SigType {
            phantom: std::marker::PhantomData,
        }
    }
}

pub trait ArchetypeTy {
    fn describe(&self) -> ArchetypeDescriptor;
}
#[derive(Copy, Clone)]
pub struct CharacterArchetype {}

pub struct Archetype<T: ArchetypeTy> {
    archetype: T,
}
impl ArchetypeTy for CharacterArchetype {
    fn describe(&self) -> ArchetypeDescriptor {
        let descriptor = ArchetypeDescriptor::new(
            "Character".to_string(),
            arch_sig!([
                CharacterNameComponent {
                    name: CharacterNameFormat {
                        given_name: "Given name".to_string(),
                        other_names: Vec::new(),
                        family_name: "Family name".to_string()
                    },
                    aliases: Vec::new()
                },
                NumericalFieldComponent {
                    name: "Age".to_string(),
                    value: 0.0
                },
                NumericalFieldComponent {
                    name: "Height".to_string(),
                    value: 0.0
                },
                NumericalFieldComponent {
                    name: "Weight".to_string(),
                    value: 0.0
                },
                RelationshipComponent {
                    relationship: Relationship::new(
                        "Mother".to_string(),
                        ERelationship::parent_child(Parent::Mother, Child::Son),
                        (0, 0)
                    )
                },
                RelationshipComponent {
                    relationship: Relationship::new(
                        "Father".to_string(),
                        ERelationship::parent_child(Parent::Father, Child::Son),
                        (0, 0)
                    )
                },
                StringFieldComponent {
                    name: "Description".to_string(),
                    value: "".to_string()
                },
            ]),
        );
        descriptor
    }
}

///An archetype serialized from json data
pub struct JsonArchetype {
    archetype_name: String,
    pub json_string: String,
}
impl JsonArchetype {
    pub fn new(archetype_name: String, json_string: String) -> Self {
        Self {
            archetype_name,
            json_string,
        }
    }
}

impl ArchetypeTy for JsonArchetype {
    fn describe(&self) -> ArchetypeDescriptor {
        let archetype_descriptor: ArchetypeDescriptor =
            serde_json::from_str(&self.json_string).unwrap();
        archetype_descriptor
    }
}
impl<T: ArchetypeTy> ArchetypeTy for Archetype<T> {
    fn describe(&self) -> ArchetypeDescriptor {
        self.archetype.describe()
    }
}

#[cfg(test)]
mod test_archetype {
    use std::io::{Read, Write};

    use super::*;

    #[test]
    fn test_json_serde() {
        let character_archetype = CharacterArchetype {};
        let describe = character_archetype.describe();
        //serialize
        let json = serde_json::to_string(&describe).unwrap();
        //write to file
        let mut file = std::fs::File::create("character_archetype.json").unwrap();
        file.write_all(json.as_bytes()).unwrap();

        //read from file
        let mut file = std::fs::File::open("character_archetype.json").unwrap();
        let mut json_str = String::new();
        file.read_to_string(&mut json_str).unwrap();
        let archetype_descriptor: ArchetypeDescriptor = serde_json::from_str(&json_str).unwrap();
        //assert names are equal
        assert_eq!(describe.name, archetype_descriptor.name);
        //assert sigs are equal
        assert_eq!(describe.signature, archetype_descriptor.signature);
    }
}
