//! Code for the action system in Novella.
//! Actions represent something that induces change in the current state of the engine, and also define
//! a method of possibly undoing said change
#![feature(associated_type_bounds)]
use ::common::exports::anyhow::{anyhow, Result};
use ::common::uuid;
use nvproc::{undo_action, Resource};
use std::cell::Cell;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use std::{
    any::Any,
    collections::{HashMap, VecDeque},
};

macro_rules! action {
    ($name:ident, $resource:ident,$param:ident) => {
        pub struct $name {
            pub action: Box<dyn Fn(&mut Mir, &mut Param<$param>) -> Result<Resrc<$resource>>>,
            pub undo_action: Box<dyn Fn(&mut Mir, &mut Resrc<$resource>) -> Result<()>>,
        }
    };
}
use crate::mir::Mir;
use dyn_clone::DynClone;
//Resource type. Indicates whether a type can be used as a resource. Such a type must be cloneable.
pub trait ResrcTy {
    fn get_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> ResrcTy for T {
    fn get_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}
pub trait ParamTy: Clone {
    fn get_param(self) -> Box<dyn Any>;
}
pub struct Param<T: ParamTy>(T);

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

//impl deref for Resrc<T> that returns the inner type
impl<T> std::ops::Deref for Resrc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait ActionTy {
    fn exec(&mut self, mir: &mut Mir) -> Result<(Box<dyn ResrcTy>, Box<dyn RvTy>)>;
    fn undo(&mut self, mir: &mut Mir, rsrc: Resrc<&mut dyn ResrcTy>) -> Result<()>;
    fn action_id(&self) -> u128;
    fn set_id(&mut self, id: u128);
}
pub trait RvTy: Any + dyn_clone::DynClone {}
impl<T: Any + dyn_clone::DynClone> RvTy for T {}
dyn_clone::clone_trait_object!(RvTy);
pub struct ReturnValue<Rv: RvTy + Clone> {
    rv: Arc<Mutex<Option<Box<dyn RvTy>>>>,
    phantom: std::marker::PhantomData<Rv>,
}
impl<Rv: RvTy + Clone> ReturnValue<Rv> {
    pub fn new(rv: Option<Rv>) -> Self {
        let rv_val = match rv {
            Some(rv) => Some(Box::new(rv) as Box<dyn RvTy>),
            None => None,
        };
        Self {
            rv: Arc::new(Mutex::new(rv_val)),
            phantom: std::marker::PhantomData,
        }
    }
    pub fn from(rv: Arc<Mutex<Option<Box<dyn RvTy>>>>) -> Self {
        Self {
            rv,
            phantom: std::marker::PhantomData,
        }
    }
    ///Fill this return value with a new value.
    pub fn fill(&mut self, rv: Rv) {
        let mut rv_val = self.rv.lock().unwrap();
        *rv_val = Some(Box::new(rv) as Box<dyn RvTy>);
    }
    pub fn get(&self) -> Result<Rv> {
        let rv_val = self.rv.lock().unwrap();
        match *rv_val {
            Some(ref rv) => Ok(*(rv.clone() as Box<dyn Any>).downcast::<Rv>().unwrap()),

            None => Err(anyhow!("No return value")),
        }
    }
}

///Executor function is a function that takes a mutable reference to the Mir, a parameter P, and returns
/// a Result with a boxed resource Rsrc, and return value Rv. The boxed resource is used to store any state that
/// is needed by the [UndoTy] function. The return value Rv is a value that will be returned to the caller of the
/// action
pub trait ExecTy<P, Rsrc: ResrcTy, Rv: RvTy>: Fn(&mut Mir, P) -> Result<Box<(Rsrc, Rv)>> {}
impl<P, Rsrc: ResrcTy, Rv: RvTy, F: Fn(&mut Mir, P) -> Result<Box<(Rsrc, Rv)>>> ExecTy<P, Rsrc, Rv>
    for F
{
}
pub trait UndoTy<Rsrc: ResrcTy>: Fn(&mut Mir, Resrc<&Rsrc>) -> Result<()> {}
impl<Rsrc: ResrcTy, F: Fn(&mut Mir, Resrc<&Rsrc>) -> Result<()>> UndoTy<Rsrc> for F {}
pub struct StaticAction<Rsrc: ResrcTy, P: Clone, Rv: RvTy, E: ExecTy<P, Rsrc, Rv>, U: UndoTy<Rsrc>>
{
    pub action_id: u128,
    pub param: P,
    exec: E,
    undo: Option<U>,
    pub is_complete: bool,
    phantom: std::marker::PhantomData<Rsrc>,
    phantom2: std::marker::PhantomData<Rv>,
}
impl<Rsrc: ResrcTy, P: Clone, Rv: RvTy, E: ExecTy<P, Rsrc, Rv>, U: UndoTy<Rsrc>>
    StaticAction<Rsrc, P, Rv, E, U>
{
    const fn new_static(p: P, e: E, u: Option<U>, id: u128) -> Self {
        StaticAction {
            action_id: id,
            param: p,
            exec: e,
            undo: u,
            is_complete: false,
            phantom: std::marker::PhantomData,
            phantom2: std::marker::PhantomData,
        }
    }
    fn new(p: P, e: E, u: Option<U>) -> Self {
        StaticAction {
            action_id: uuid::gen_128(),
            param: p,
            exec: e,
            undo: u,
            is_complete: false,
            phantom: std::marker::PhantomData,
            phantom2: std::marker::PhantomData,
        }
    }
    pub fn exec(&mut self, mir: &mut Mir) -> Result<Box<(Rsrc, Rv)>> {
        self.is_complete = true;
        (self.exec)(mir, self.param.clone())
    }
    pub fn undo(&mut self, mir: &mut Mir, resrc: Resrc<&Rsrc>) -> Result<()> {
        if self.is_complete {
            let res = match &self.undo {
                Some(u) => Ok(u),
                None => Err(anyhow!("This action has no undo!")),
            }?;
            (res)(mir, resrc);
            self.is_complete = false;
        }else {
            return Err(anyhow!(
            "Action {} has not yet been completed and cannot be undone!",
            self.action_id
        ));
        }
        Ok(())
    }
}
impl<
        Rsrc: ResrcTy + Clone + 'static,
        P: Clone,
        Rv: RvTy,
        E: ExecTy<P, Rsrc, Rv>,
        U: UndoTy<Rsrc>,
    > ActionTy for StaticAction<Rsrc, P, Rv, E, U>
{
    fn exec(&mut self, mir: &mut Mir) -> Result<(Box<dyn ResrcTy>, Box<dyn RvTy>)> {
        let res = self.exec(mir)?;
        //convert R to Box<dyn ResrcTy>
        let r = (
            Box::from(res.0) as Box<dyn ResrcTy>,
            Box::from(res.1) as Box<dyn RvTy>,
        );
        //convert to to box
        Ok(r)
    }

    fn undo(&mut self, mir: &mut Mir, rsrc: Resrc<&mut dyn ResrcTy>) -> Result<()> {
        let rsrc = rsrc.into_type();
        let r = rsrc.get_mut().downcast_ref::<Rsrc>().unwrap();
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
    ///Return values for actions. Not all actions will have a return value, and so it 
    /// represented as an Option. 
    pub return_values: HashMap<u128, Option<Arc<Mutex<Option<Box<dyn RvTy>>>>>>,
    //indicates position in undo
    pub cursor: ActionCursor,
}
//implement actman
impl<'ac> Actman<'ac> {
    pub fn new() -> Self {
        Self {
            actions: VecDeque::new(),
            resources: HashMap::new(),
            cursor: ActionCursor::new(),
            return_values: HashMap::new(),
        }
    }
    pub fn register_action<T: ActionTy + 'ac>(&mut self, action: T) {
        //if the actioncursor is not at the front of the queue,
        //we must invalidate everything after the cursor
        if self.cursor.cursor > 0 {
            self.actions.drain(self.cursor.cursor as usize..);
        }
        self.return_values.insert(action.action_id(), None);
        self.cursor = self.actions.len().into();
        self.actions.push_front(Box::new(action));
    }

    ///Register a new action that is expected to return a value.
    pub fn register_action_with_rv<
        Rsrc: ResrcTy + Clone + 'static,
        Param: Clone + 'static,
        Rv: RvTy + Clone + 'static,
        E: ExecTy<Param, Rsrc, Rv> + 'static,
        U: UndoTy<Rsrc> + 'static,
    >(
        &mut self,
        action: StaticAction<Rsrc, Param, Rv, E, U>,
    ) -> ReturnValue<Rv> {
        //if the actioncursor is not at the front of the queue,
        //we must invalidate everything after the cursor
        if self.cursor.cursor > 0 {
            self.actions.drain(self.cursor.cursor as usize..);
        }
        self.return_values
            .insert(action.action_id(), Some(Arc::new(Mutex::new(None))));
        let r = self.return_values.get(&action.action_id()).unwrap().clone();

        self.cursor = self.actions.len().into();
        self.actions.push_front(Box::new(action));
        ReturnValue::from(r.unwrap())
    }
    //Advances the action cursor forward by one. If the cursor is in sync with the latest action
    //(at the front of queue),this does nothing. Otherwise, it executes the action at the current cursors location,
    //and advances by 1
    pub fn advance(&mut self, mir: &mut Mir) -> Result<()> {
        if !self.cursor.is_valid() {
            return Err(anyhow!("Cursor is not valid!"));
        }
        let mut action = self.actions.get_mut(self.cursor.into()).unwrap();

        //execute and collect any resources
        let (rsrc, retval) = action.exec(mir)?;
        let a = self
            .return_values
            .get_mut(&action.action_id())
            .expect("Action id not found!");

        match a {
            Some(r) => {
                let mut rv = a.as_ref().unwrap().lock().unwrap();
                *rv = Some(retval);
            }
            None => {}
        }

        //generate a resource id
        let resource_id = uuid::gen_128();
        action.set_id(resource_id);
        self.resources.insert(resource_id, rsrc);
        //advance the cursor
        self.cursor += 1;
        Ok(())
    }
    //Move the action cursor backward by one, undoing the action the cursor was pointing at
    pub fn regress(&mut self, mir: &mut Mir) -> Result<()> {
        let mut action = self.actions.pop_back().unwrap();
        let action_id = action.action_id();
        let r = &mut *self.resources.get_mut(&action_id).unwrap();
        action.undo(mir, Resrc::new(r.as_mut()))?;
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
mod actions;
pub mod request;
#[cfg(test)]
mod test;
