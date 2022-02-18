//! Code for the action system in Novella.
//! Actions represent something that induces change in the current state of the engine, and also define
//! a method of possibly undoing said change

#![feature(associated_type_bounds)]
use std::{any::Any, collections::HashMap};

use crate::mir::Mir;

pub trait ResrcTy {
    fn get_resource(&mut self) -> &mut dyn Any;
}
#[derive(Debug)]
pub struct TestRsrc {
    pub name: String,
}
impl ResrcTy for TestRsrc {
    fn get_resource(&mut self) -> &mut dyn Any {
        self
    }
}
impl ResrcTy for () {
    fn get_resource(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Resrc<T: ResrcTy + Sized>(T);
impl<T: ResrcTy> Resrc<T> {
    pub fn into_type(self) -> T {
        self.0
    }
}

pub fn test_fn(mir: &mut Mir) -> Box<dyn ResrcTy> {
    let name = "test";
    let mut rsrc = TestRsrc {
        name: name.to_string(),
    };
    Box::new(rsrc)
}
impl ResrcTy for &dyn ResrcTy {
    fn get_resource(&mut self) -> &mut dyn Any {
        self.get_resource()
    }
}
pub fn undo_test_fn(mut rsrc: Resrc<&dyn ResrcTy>) {
    let mut rsrc: &TestRsrc = rsrc
        .0
        .get_resource()
        .downcast_ref::<TestRsrc>()
        .unwrap()
        .clone();
    println!("undo_test_fn: {}", rsrc.name);
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
    fn get_resource(&mut self) -> &mut dyn Any {
        self
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
    pub exec: &'a dyn Fn(&mut Mir) -> Box<dyn ResrcTy>,
    pub undo: &'a dyn Fn(Resrc<&dyn ResrcTy>),
}
impl<'a> Action<'a> {
    pub fn new(
        resource_id: u128,
        exec: &'a impl Fn(&mut Mir) -> Box<dyn ResrcTy>,
        undo: &'a impl Fn(Resrc<&dyn ResrcTy>),
    ) -> Self {
        Action {
            resource_id,
            exec,
            undo: undo,
        }
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
