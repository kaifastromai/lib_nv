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

pub trait ArchetypeTy {
    fn describe(&self) -> ArchetypeDescriptor;
}
pub struct CharacterArchetype {
    pub archetype_name: String,
    pub name: String,
    pub sex: String,
    pub age: u32,
    pub bio: String,
}
impl ArchetypeTy for CharacterArchetype {
    fn describe(&self) -> ArchetypeDescriptor {
        let descriptor = ArchetypeDescriptor::new(signature!(Field, Field, Field, Field));
        descriptor
    }
}

//Given that bevy has bundles, the only point of this is to store user-made archetypes
struct SerializableArchetype {}
