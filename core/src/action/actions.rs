use super::*;
use crate::{ecs::Id, mir::Mir};
use ::common::exports::*;

//Add entity
pub struct AddEntityResource {
    pub entity: Id,
}
pub fn add_entity(mir: &mut Mir, _: ()) -> Result<AddEntityResource> {
    let entity = mir.add_entity();
    Ok(AddEntityResource { entity })
}
pub fn undo_add_entity(mir: &mut Mir, rsrc: AddEntityResource) -> Result<()> {
    mir.em.remove_entity(rsrc.entity);
    Ok(())
}
