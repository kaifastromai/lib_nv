/*! The [actions] module contains a number of preconfigured [ActionTy]'s for convenince purposes*/
use super::*;
use crate::{
    ecs::{Component, ComponentTy, ComponentTyReqs, DynamicComponent, EntityOwned, Id},
    mir::Mir,
};
use ::common::exports::*;

//----Add entity
#[derive(Clone)]
pub struct AddEntityResrc {
    pub entity: Id,
}
pub struct EntityOwnedResource {
    pub entity: EntityOwned,
}
///Add entity function.
pub fn ae_add_entity(mir: &mut Mir, p: ()) -> Result<Box<(AddEntityResrc, Id)>> {
    let entity = mir.em.add_entity();
    let mut rsrc = AddEntityResrc { entity };
    Ok(Box::new((rsrc, entity)))
}
///Undo add entity function.
pub fn au_add_entity(mir: &mut Mir, r: Resrc<&AddEntityResrc>) -> Result<()> {
    //remove entity
    mir.em.remove_entity(r.entity);
    Ok(())
}
///Remove an entity with the given [Id]
pub fn ae_remove_entity(mir: &mut Mir, id: Id) -> Result<Box<(EntityOwned, ())>> {
    let entity_owned = mir.em.get_entity_owned(id)?;
    mir.em.remove_entity(id);
    Ok(Box::new((entity_owned, ())))
}
///Undo remove entity function.
pub fn au_remove_entity(mir: &mut Mir, entity_owned: Resrc<&EntityOwned>) -> Result<()> {
    mir.em.entity_from_owned(entity_owned.into_type().clone());
    Ok(())
}
///Add component to an entity with the given [Id]
pub fn ae_add_component<C: Clone + ComponentTyReqs + serde::Serialize + Clone>(
    mir: &mut Mir,
    p: (Id, C),
) -> Result<Box<((Id, common::type_id::TypeId), ())>> {
    let tid = p.1.get_component_type_id();
    mir.em.add_component(p.0, p.1)?;
    let dynamic_component = mir.em.get_component_mut::<C>(p.0)?.into_dynamic();
    Ok(Box::new(((p.0, tid), ())))
}
///Undo add component to an entity with the given [Id]
pub fn au_add_component(mir: &mut Mir, r: Resrc<&(Id, common::type_id::TypeId)>) -> Result<()> {
    mir.em.remove_component_by_type_id(r.1, r.0 .0)
}

type Executor<P, Rsrc: ResrcTy, Rv: RvTy> = fn(&mut Mir, P) -> Result<Box<(Rsrc, Rv)>>;
type Undoer<R> = fn(&mut Mir, Resrc<&R>) -> Result<()>;

///Generates a StaticAction. the arguments are:
/// - the name of the action
/// - the Param type of the action
/// - the resource type of the action
/// - the Executor function of the action
/// - the Undoer function of the action. This is an Option<[Undoer<R>]>
/// - the id of this action. This must be unique. This is used to identify the action in the action manager.
/// All static actions should have uppercase snake case names that begin with "AP_" for pure actions (actions that are not undoable) and "AU_" for undoable actions.
// macro_rules! static_action {
//     ($name:ident, $param:tt, $resrc:tt,$exec:tt,$undo:tt,$id:literal) => {
//         static $name: StaticAction<$resrc, $param, Executor<$param, $resrc>, Undoer<$resrc>> =
//             StaticAction::new($param, $exec, $undo, $id);
//     };
// }

///Action constructors generate actions.
pub trait ActionConstructorTy {
    ///The type of action this constructor creates.
    type Ac: ActionTy;
    ///The type resource the action will need
    type Rsrc: ResrcTy;
    ///The type of parameter that the executor takes.
    type P: Clone;
    ///The type of value the executor returns, by default the unit tuple.
    type Rv: RvTy = ();
    ///The executor function of the action.
    type E = Executor<Self::P, Self::Rsrc, Self::Rv>;
    ///The undoer function of the action.
    type U = Undoer<Self::Rsrc>;
    ///Construct the action, returning it.
    fn construct(&self) -> Self::Ac;
}

pub struct AddEntityConstructor {}
impl ActionConstructorTy for AddEntityConstructor {
    type Ac = StaticAction<Self::Rsrc, Self::P, Self::Rv, Self::E, Self::U>;
    type Rsrc = AddEntityResrc;
    type P = ();
    type Rv = Id;
    fn construct(&self) -> Self::Ac {
        Self::Ac::new_static(Self::P::default(), ae_add_entity, Some(au_add_entity), 0)
    }
}
pub struct RemoveEntityConstructor {}
impl ActionConstructorTy for RemoveEntityConstructor {
    type Ac = StaticAction<Self::Rsrc, Self::P, Self::Rv, Self::E, Self::U>;
    type Rsrc = EntityOwned;
    type P = Id;
    fn construct(&self) -> Self::Ac {
        Self::Ac::new_static(
            Self::P::default(),
            ae_remove_entity,
            Some(au_remove_entity),
            1,
        )
    }
}
pub struct AddComponentConstructor<T: ComponentTyReqs + Clone + serde::Serialize> {
    pub component: T,
    pub entity: Id,
}
impl<T: ComponentTy + Clone + serde::Serialize> ActionConstructorTy for AddComponentConstructor<T> {
    type Ac = StaticAction<Self::Rsrc, Self::P, Self::Rv, Self::E, Self::U>;
    type Rsrc = (Id, common::type_id::TypeId);
    type P = (Id, T);

    fn construct(&self) -> Self::Ac {
        Self::Ac::new_static(
            (self.entity, self.component.clone()),
            ae_add_component,
            Some(au_add_component),
            2,
        )
    }
}
