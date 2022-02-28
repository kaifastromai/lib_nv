//Archetypes map to bevy's bundles

use super::*;

//An archetype is an entity that has a predefined set of components

pub struct ArchetypeDescriptor {
    pub components: Vec<TypeId>,
}
pub trait ArchetypeTy {
    fn generate(&self) -> ArchetypeDescriptor;
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
    fn generate(&self) -> ArchetypeDescriptor {
        todo!()
    }
}
//Given that bevy has bundles, the only point of this is to store user-made archetypes
struct SerializableArchetype {}
