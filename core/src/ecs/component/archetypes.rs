//Archetypes map to bevy's bundles

use crate::ecs::component::components::*;
use crate::ecs::component::relationship::*;
use crate::ecs::*;
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

pub struct ArchetypeDescriptor {
    signature: Vec<TypeId>,
    components: Vec<EComponentGraphTypes>,
}
impl ArchetypeDescriptor {
    pub fn new_empty() -> Self {
        Self {
            signature: Vec::new(),
            components: Vec::new(),
        }
    }
    pub fn new(components: Vec<EComponentGraphTypes>) -> Self {
        Self {
            signature: components.iter().map(|c| c.get_type_id_ref()).collect(),
            components,
        }
    }

    pub fn with_component<T: ComponentTypeIdTy>(mut self) -> Self {
        self.signature.push(TypeId::of::<T>());
        self
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
    fn consume(&self) -> Vec<Box<dyn ComponentTy>>;
}
pub struct CharacterArchetype {
    archetype_name: &'static str,
    data: Vec<Box<dyn ComponentTy>>,
}
pub struct Archetype<T: ArchetypeTy> {
    archetype: T,
}
impl ArchetypeTy for CharacterArchetype {
    fn describe(&self) -> ArchetypeDescriptor {
        let descriptor = ArchetypeDescriptor::new(arch_sig!([
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
        ]));
        descriptor
    }
    fn consume(&self) -> Vec<Box<dyn ComponentTy>> {
        todo!()
    }
}

//Given that bevy has bundles, the only point of this is to store user-made archetypes
struct SerializableArchetype {}
