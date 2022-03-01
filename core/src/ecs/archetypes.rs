//Archetypes map to bevy's bundles

use super::*;

//An archetype is an entity that has a predefined set of components

pub struct ArchetypeDescriptor {
    pub components: Vec<TypeId>,
}
impl ArchetypeDescriptor {
    pub fn new<T: ComponentTy>(v: Vec<T>) -> Self {
        ArchetypeDescriptor {
            components: v.iter().map(|c| TypeId::of::<T>()).collect(),
        }
    }
}
impl<T: ComponentTy> From<&'static [&T]> for ArchetypeDescriptor {
    fn from(v: &'static [&T]) -> Self {
        ArchetypeDescriptor {
            components: v.iter().map(|c| TypeId::of::<T>()).collect(),
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
    pub description: String,
    pub father: Option<Id>,
    pub mother: Option<Id>,
    pub children: Vec<Id>,
    pub spouse: Option<Id>,
}
impl ArchetypeTy for CharacterArchetype {
    fn describe(&self) -> ArchetypeDescriptor {
        todo!()
    }
}
//Given that bevy has bundles, the only point of this is to store user-made archetypes
struct SerializableArchetype {}
