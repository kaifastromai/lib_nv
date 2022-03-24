//Archetypes map to bevy's bundles

use crate::ecs::component::*;
use crate::ecs::*;
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

macro_rules! arch_sig {
    ($($comp:ident),*) => {
        {
            let mut sig=Vec::<ComponentTypeId>::new();
            $(
                let sigtype=SigType::<$comp>{phantom:std::marker::PhantomData};
                sig.push(sigtype.into_type_id());
            )*
            sig
        }
    };
}

//An archetype is an entity that has a predefined set of components
macro_rules! archetype {
    ( $($signature:tt),* ) => {};
}
pub struct ArchetypeDescriptor {
    components: Vec<TypeId>,
}
impl ArchetypeDescriptor {
    pub fn new_empty() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    pub fn new(components: Vec<impl ComponentTypeIdTy>) -> Self {
        Self {
            components: components.iter().map(|c| c.get_type_id_ref()).collect(),
        }
    }

    pub fn with_component<T: ComponentTypeIdTy>(mut self) -> Self {
        self.components.push(TypeId::of::<T>());
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
    fn consume(self) -> Vec<Box<dyn ComponentTy>>;
}
pub struct CharacterArchetype {
    archetype_name: &'static str,
    data: Vec<Box<dyn ComponentTy>>,
}
impl ArchetypeTy for CharacterArchetype {
    fn describe(&self) -> ArchetypeDescriptor {
        let descriptor = ArchetypeDescriptor::new(arch_sig!(Field, Field, Field, Field));
        descriptor
    }
    fn consume(mut self) -> Vec<Box<dyn ComponentTy>> {
        let name = Field {
            name: String::from("name"),
            value: String::from(""),
        };
        self.data.push(Box::new(name));
        let description = Field {
            name: String::from("description"),
            value: String::from(""),
        };
        self.data.push(Box::new(description));
        let age = Field {
            name: String::from("age"),
            value: String::from(""),
        };
        self.data.push(Box::new(age));
        let height = Field {
            name: String::from("height"),
            value: String::from(""),
        };
        self.data.push(Box::new(height));
        self.data
    }
}

//Given that bevy has bundles, the only point of this is to store user-made archetypes
struct SerializableArchetype {}
