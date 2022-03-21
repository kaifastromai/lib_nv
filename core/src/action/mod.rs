//! Code for the action system in Novella.
//! Actions represent something that induces change in the current state of the engine, and also define
//! a method of possibly undoing said change
#![feature(associated_type_bounds)]
use utils::prelude::anyhow::{anyhow, Result};
use nvproc::{undo_action, Resource};
use std::{
    any::Any,
    collections::{HashMap, VecDeque},
};
use utils::uuid;
macro_rules! action {
    ($name:ident, $resource:ident,$param:ident) => {
        pub struct $name {
            pub action: Box<dyn Fn(&mut Mir, &mut Param<$param>) -> Result<Resrc<$resource>>>,
            pub undo_action: Box<dyn Fn(&mut Mir, &mut Resrc<$resource>) -> Result<()>>,
        }
    };
}
use crate::mir::{Mir, MirData};
use dyn_clone::DynClone;
//Resource type. Indicates whether a type can be used as a resource. Such a type must be cloneable.
pub trait ResrcTy {
    fn get_mut(&mut self) -> &mut dyn Any;
}
pub trait ParamTy: Clone {
    fn get_param(self) -> Box<dyn Any>;
}
pub struct Param<T: ParamTy>(T);

impl ResrcTy for () {
    fn get_mut(&mut self) -> &mut dyn Any {
        self
    }
}

//A thin wrapper around a resource.
pub struct Resrc<T>(T);
impl<T> Resrc<T> {
    pub fn into_type(self) -> T {
        self.0
    }
}
impl<T> Resrc<T> {
    pub fn new(t: T) -> Self {
        Resrc(t)
    }
}
impl ResrcTy for Box<dyn ResrcTy> {
    fn get_mut(&mut self) -> &mut dyn Any {
        self.get_mut()
    }
}
//impl deref for Resrc<T> that returns the inner type
impl<T> std::ops::Deref for Resrc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait ActionTy {
    fn exec(&mut self, mir: &mut MirData) -> Result<Box<dyn ResrcTy>>;
    fn undo(&mut self, mir: &mut MirData, rsrc: Resrc<&mut dyn ResrcTy>) -> Result<()>;
    fn action_id(&self) -> u128;
    fn set_id(&mut self, id: u128);
}

//An action represents something that induce a change in the current state of the kernel.
//An action contains two functions, one that executes the action, and optionaly one that undoes the action.
//Th executor takes a reference to the mir, and a generic parameter P that is the type of the parameter
//Th undoer takes a reference to the mir, and a generic parameter R that is the type of the resource
pub struct Action<'a, R: ResrcTy, P: Clone> {
    pub action_id: u128,
    pub param: P,
    exec: &'a dyn Fn(&mut MirData, P) -> Result<Box<R>>,
    undo: Option<&'a dyn Fn(&mut MirData, Resrc<&R>) -> Result<()>>,
    pub is_complete: bool,
}
impl<'a, R: ResrcTy, P: Clone> Action<'a, R, P> {
    //Create a new action, specifying the function to execute and the function that undoes the action
    pub fn new(
        exec: &'a impl Fn(&mut MirData, P) -> Result<Box<R>>,
        undo: &'a impl Fn(&mut MirData, Resrc<&R>) -> Result<()>,
        param: P,
    ) -> Self {
        Action {
            is_complete: false,
            exec,
            undo: Some(undo),
            param,
            action_id: 0,
        }
    }
    //Create a new action, only specifying the function to execute. This action will NOT be undoable
    pub fn new_pure(exec: &'a impl Fn(&mut MirData, P) -> Result<Box<R>>, param: P) -> Self {
        Action {
            is_complete: false,
            exec,
            undo: None,
            param,
            action_id: 0,
        }
    }
    pub fn exec(&mut self, mir: &mut MirData) -> Result<Box<R>> {
        self.is_complete = true;
        (self.exec)(mir, self.param.clone())
    }
    pub fn undo(&mut self, mir: &mut MirData, resrc: Resrc<&R>) -> Result<()> {
        if self.is_complete {
            let res = match self.undo {
                Some(u) => Ok(u),
                None => Err(anyhow!("This action has no undo!")),
            }?;
            (res)(mir, resrc);
            self.is_complete = false;
        };
        return Err(anyhow!(
            "Action {} has not yet been completed and cannot be undone!",
            self.action_id
        ));
    }
}
impl<'a, R: ResrcTy + Clone + 'static, P: Clone> ActionTy for Action<'a, R, P> {
    fn exec(&mut self, mir: &mut MirData) -> Result<Box<dyn ResrcTy>> {
        let res = self.exec(mir)?;
        //convert R to Box<dyn ResrcTy>
        let r = (Box::from(*res) as Box<dyn ResrcTy>);
        //convert to to box
        Ok(r)
    }
    fn undo(&mut self, mir: &mut MirData, resrc: Resrc<&mut dyn ResrcTy>) -> Result<()> {
        let rsrc = resrc.into_type();
        let r = rsrc.get_mut().downcast_ref::<R>().unwrap();
        let boxed = Box::from(r.clone());
        self.undo(mir, Resrc::new(r))
    }
    fn action_id(&self) -> u128 {
        self.action_id
    }
    fn set_id(&mut self, id: u128) {
        self.action_id = id;
    }
}

//Actman manages the actions and any resources that they may need.

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActionCursor {
    cursor: i32,
}

impl ActionCursor {
    pub fn new() -> Self {
        ActionCursor { cursor: -1 }
    }
    pub fn is_valid(&self) -> bool {
        self.cursor >= 0
    }
    pub fn reset(&mut self) {
        self.cursor = -1;
    }
}
//overload + operator for action cursor
impl std::ops::Add<usize> for ActionCursor {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        ActionCursor {
            cursor: self.cursor + rhs as i32,
        }
    }
}
//overload - operator for action cursor
impl std::ops::Sub<usize> for ActionCursor {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self::Output {
        ActionCursor {
            //clamped subtract to -1
            cursor: (-1 as i32).max(self.cursor - rhs as i32),
        }
    }
}
impl<T: num::Num + num::ToPrimitive> From<T> for ActionCursor {
    fn from(t: T) -> Self {
        ActionCursor {
            cursor: t.to_i32().unwrap(),
        }
    }
}
impl From<ActionCursor> for usize {
    fn from(ac: ActionCursor) -> Self {
        ac.cursor as usize
    }
}
impl From<ActionCursor> for i32 {
    fn from(ac: ActionCursor) -> Self {
        ac.cursor
    }
}
//impl addassign for action cursor
impl std::ops::AddAssign<usize> for ActionCursor {
    fn add_assign(&mut self, rhs: usize) {
        self.cursor += rhs as i32;
    }
}
//impl subtractassign for action cursor
impl std::ops::SubAssign<usize> for ActionCursor {
    fn sub_assign(&mut self, rhs: usize) {
        self.cursor = (*self - rhs).cursor;
    }
}

//Manages actions and their resources
pub struct Actman<'ac> {
    pub actions: VecDeque<Box<dyn ActionTy + 'ac>>,
    pub resources: HashMap<u128, Box<dyn ResrcTy>>,
    pub mir_ref: &'ac mut MirData,
    //indicates position in undo
    pub cursor: ActionCursor,
}
//implement actman
impl<'ac> Actman<'ac> {
    pub fn new(mir_ref: &'ac mut MirData) -> Self {
        Self {
            actions: VecDeque::new(),
            resources: HashMap::new(),
            mir_ref,
            cursor: ActionCursor::new(),
        }
    }
    pub fn register_action<T: ActionTy + 'ac>(&mut self, action: T) {
        //if the actioncursor is not at the front of the queue,
        //we must invalidate everything after the cursor
        if self.cursor.cursor > 0 {
            self.actions.drain(self.cursor.cursor as usize..);
        }
        self.cursor = self.actions.len().into();
        self.actions.push_front(Box::new(action));
    }
    //Advances the action cursor forward by one. If the cursor is in sync with the latest action
    //(at the front of queue),this does nothing. Otherwise, it executes the action at the current cursors location,
    //and advances by 1
    pub fn advance(&mut self) -> Result<()> {
        if !self.cursor.is_valid() {
            return Err(anyhow!("Cursor is not valid!"));
        }
        let mut action = self.actions.get_mut(self.cursor.into()).unwrap();
        //execute and collect any resources
        let rsrc = action.exec(self.mir_ref)?;
        //generate a resource id
        let resource_id = uuid::generate();
        action.set_id(resource_id);
        self.resources.insert(resource_id, rsrc);
        //advance the cursor
        self.cursor += 1;
        Ok(())
    }
    //Move the action cursor backward by one, undoing the action the cursor was pointing at
    pub fn regress(&mut self) -> Result<()> {
        let mut action = self.actions.pop_back().unwrap();
        let action_id = action.action_id();
        let r = &mut *self.resources.get_mut(&action_id).unwrap();

        action.undo(self.mir_ref, Resrc::new(r.as_mut()))?;
        self.cursor -= 1;
        Ok(())
    }
    //Disposes of everything in the current queue,resetting cursor to zero and
    //and deallocating any resources
    pub fn dispose(&mut self) {
        self.cursor.reset();
        //disposes of resources
        self.resources.clear();
        self.actions.clear();
    }
}
mod common;
pub mod request;
#[cfg(test)]
mod test;
