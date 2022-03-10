//Archetypes map to bevy's bundles

use super::component::Field;
use super::*;
struct SigType<T: ComponentTy> {
    phantom: std::marker::PhantomData<T>,
}
impl<T: ComponentTy> SigType<T> {
    pub fn into_type_id(self) -> TypeId {
        TypeId::of::<T>()
    }
}
macro_rules! signature {
    ($($comp:ident),*) => {
        {
            let mut sig=Vec::<TypeId>::new();
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
    pub components: Vec<TypeId>,
}
impl ArchetypeDescriptor {
    pub fn new_empty() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    pub fn new(components: Vec<TypeId>) -> Self {
        Self { components }
    }

    pub fn with_component<T: ComponentTy>(mut self) -> Self {
        self.components.push(TypeId::of::<T>());
        self
    }
}

impl<T: ComponentTy> From<T> for SigType<T> {
    fn from(_: T) -> Self {
        SigType {
            phantom: std::marker::PhantomData,
        }
    }
}

pub trait ArchetypeTy<'a> {
    fn describe(&self) -> ArchetypeDescriptor;
    fn consume(self) -> Vec<&'a dyn ComponentTy>;
}
pub struct CharacterArchetype<'a> {
    archetype_name: String,
    data: Vec<&'a dyn ComponentTy>,
}
impl<'a> ArchetypeTy<'a> for CharacterArchetype<'a> {
    fn describe(&self) -> ArchetypeDescriptor {
        let descriptor = ArchetypeDescriptor::new(signature!(Field, Field, Field, Field));
        descriptor
    }
    fn consume(self) -> Vec<&'a dyn ComponentTy> {
        let name = Field {
            name: String::from("name"),
            value: String::from(""),
        };
        let description = Field {
            name: String::from("description"),
            value: String::from(""),
        };
        let age = Field {
            name: String::from("age"),
            value: String::from(""),
        };
        let height = Field {
            name: String::from("height"),
            value: String::from(""),
        };
        self.data
    }
}

//Given that bevy has bundles, the only point of this is to store user-made archetypes
struct SerializableArchetype {}
