//Archetypes map to bevy's bundles

use bevy_ecs::prelude::*;
pub trait Archetype {
    fn generate(&self, world: &mut World);
}

//Given that bevy has bundles, the only point of this is to store user-made archetypes
struct SerializableArchetype {}
