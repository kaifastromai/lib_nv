//! Code for the action system in Novella.
//! Actions represent something that induces change in the current state of the engine, and also define
//! a method of possibly undoing said change

#![feature(associated_type_bounds)]
use std::collections::HashMap;

use crate::mir::Mir;

pub trait ResrcTy {
    fn get_resource(&mut self) -> &mut dyn ResrcTy;
}
#[derive(Debug)]
pub struct TestRsrc {
    pub name: String,
}
impl ResrcTy for TestRsrc {
    fn get_resource(&mut self) -> &mut dyn ResrcTy {
        self
    }
}
pub struct Resrc<T: ResrcTy>(std::marker::PhantomData<T>);

impl<T: ResrcTy> From<Resrc<T>> for T {
    fn from(_: Resrc<T>) -> Self {
        todo!()
    }
}

pub fn test_fn(name: Resrc<TestRsrc>) {
    println!("{:?}", name);
}
//Converts a normal function into an action
pub trait IntoActionFn {
    fn into_action_fn(self) -> dyn ActionFn;
}
pub trait ActionFn {
    fn call(&self, rsrc: &dyn ResrcTy) -> bool;
}

pub struct AddEntityResource {
    pub entity: crate::ecs::bevy_ecs::entity::Entity,
}

impl ResrcTy for AddEntityResource {
    fn get_resource(&mut self) -> &mut dyn ResrcTy {
        self
    }
    fn create_resource(&mut self) -> Box<dyn ResrcTy> {
        Box::new(AddEntityResource {
            entity: crate::ecs::bevy_ecs::entity::Entity::from_raw(0),
        })
    }
}
pub fn action_add_entity(mir: &mut Mir, rsrc: &mut AddEntityResource) -> bool {
    let e = mir.em.add_entity("Name".to_string());
    rsrc.entity = e;
    return true;
}
pub fn undo_add_entity(mir: &mut Mir, rsrc: &mut AddEntityResource) {
    mir.em.remove_entity(rsrc.entity);
}
pub struct Action<'a> {
    pub resource_id: u128,
    pub func: &'a dyn ActionFn,
}
impl<'a> Action<'a> {
    pub fn new(resource_id: u128, func: &'a impl ActionFn) -> Self {
        Action { resource_id, func }
    }
}

pub trait ResourceTy {
    type Resource: ResrcTy;
    fn gen() -> Self::Resource;
}

//impl ResourceTy for AddEntity
impl ResourceTy for AddEntityResource {
    type Resource = AddEntityResource;

    fn gen() -> Self::Resource {
        AddEntityResource {
            entity: crate::ecs::bevy_ecs::entity::Entity::from_raw(0),
        }
    }
}
//Actman manages the actions and any resources that they may need.
pub struct Actman<'a> {
    pub actions: HashMap<u128, Action<'a>>,
    pub resources: HashMap<u128, Box<dyn ResrcTy>>,
    pub mir_ref: &'a mut Mir,
}
//implement actman
impl<'a> Actman<'a> {
    pub fn new(mir_ref: &'a mut Mir) -> Self {
        Self {
            actions: HashMap::new(),
            resources: HashMap::new(),
            mir_ref,
        }
    }
    pub fn register_action<T: ResourceTy + 'static>(&mut self, action: Action<'a>) {
        let id = action.resource_id;
        self.actions.insert(id, action);
        self.resources.insert(id, Box::new(T::gen()));
    }
    pub fn exec(&mut self, action_id: u128) {}
}
#[cfg(test)]
mod test;
