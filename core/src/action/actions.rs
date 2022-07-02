use super::*;
use crate::{ecs::Id, mir::Mir};
use ::common::exports::*;

//----Add entity
#[derive(Clone, Resource)]
pub struct AddEntityResource {
    pub entity: Id,
}
pub fn ac_add_entity(mir: &mut Mir, p: ()) -> Result<Box<dyn ResrcTy>> {
    let entity = mir.em.add_entity();
    let mut rsrc = AddEntityResource { entity };
    Ok(Box::new(rsrc))
}
pub fn ac_add_entity_undo(mir: &mut Mir, r: Resrc<()>) -> Result<()> {
    Ok(())
}
pub fn unac_action_add_entity() {}
type sa_add_entity = StaticAction<(), (), ac_add_entity, ac_add_entity_undo>;
