use super::*;
use crate::mir::Mir;
use anyhow::{anyhow, Result};

//----Add entity
#[derive(Clone,Resource)]
pub struct AddEntityResource {
    pub entity: crate::ecs::bevy_ecs::entity::Entity,
}
pub fn action_add_entity(mir: &mut Mir) -> Result<Box<dyn ResrcTy>> {
    let entity = mir.em.add_entity("Test".to_string());
    let mut rsrc = AddEntityResource {
        entity,
    };
    Ok(Box::new(rsrc))
}



//-----
