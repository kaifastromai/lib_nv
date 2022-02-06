//! Code for the action system in Novella.
//! Actions represent something that induces change in the current state of Novella, and also define
//! a method of possibly undoing said change

pub enum EActionResult {
    None(()),
    Entity(IndexType),
}
use std::collections::VecDeque;

use crate::{ecs::EntityManager, IndexType};

pub type Result = std::result::Result<(), &'static str>;
///The [`StaticAction`] trait represents an action that can all be executed once,
///On creation, an action must immediately evaluate and execute whatever functionality.
/// An action can be ['clean']ed as well.
pub trait StaticAction {
    fn get_is_complete(&self) -> bool;
    //Allows the action to do any necessary work before it is deleted
    fn clean(&mut self);
}

///An undoable action is an action that can be undone and redone
pub trait UndoableAction {
    fn undo(&mut self) -> Result;
    fn redo(&mut self) -> Result;
    fn get_is_complete(&self) -> bool;
    fn clean(&mut self);
}
pub enum Action<'a> {
    Undoable(Box<dyn UndoableAction + 'a>),
    Static(Box<dyn StaticAction + 'a>),
}
impl<'a> Action<'a> {
    fn get_is_complete(&self) {
        match self {
            Action::Undoable(u) => u.get_is_complete(),
            Action::Static(s) => s.get_is_complete(),
        };
    }
    fn clean(&mut self) {
        match self {
            Action::Undoable(u) => u.clean(),
            Action::Static(s) => s.clean(),
        }
    }
}
///A structure containing the list of all recent [`Action`]'s
pub struct ActionStack<'a> {
    //Maximum amount of actions that will be kept in history
    max_actions: u32,
    actions: VecDeque<Action<'a>>,
    cursor: i32,
}
impl<'a> ActionStack<'a> {
    pub fn new(max_actions: u32) -> Self {
        Self {
            max_actions,
            actions: Default::default(),
            cursor: -1,
        }
    }

    pub fn undo(&mut self) -> Result {
        if self.cursor < 0 {
            return Err("Action stack is empty");
        }
        match &mut self.actions[self.cursor as usize] {
            Action::Static(s) => return Err("Action is not undoable"),
            Action::Undoable(u) => {
                u.undo()?;
                self.cursor = (-1).max(self.cursor - 1);
            }
        }
        Ok(())
    }

    pub fn add_action(&mut self, action: Action<'a>) {
        //if we add a new action and the cursor is not at the end, delete all actions after the cursor
        if self.cursor < self.actions.len() as i32 - 1 && !self.actions.is_empty() {
            self.actions
                .drain((self.cursor + 1) as usize..)
                .for_each(|mut a| a.clean());
            self.cursor = self.actions.len() as i32;
        } else {
            self.cursor += 1;
        }

        self.actions.push_back(action);

        //if we have more actions than the max, delete the oldest action
        if self.actions.len() as i32 > self.max_actions as i32 {
            self.actions.pop_front().unwrap().clean();
        };
    }

    ///Executes the previous action in the list of actions
    /// # Returns
    /// * `Result` - A Result containing the action that was executed, or an error if the action was not executed
    ///
    pub fn redo(&mut self) -> Result {
        if self.actions.len() == 0 {
            return Err("Action stack is empty");
        }
        if let Action::Undoable(u) = self.actions.back_mut().unwrap() {
            u.redo();
            self.cursor = (-1).max(self.cursor - 1);
        } else {
        }
        Ok(())
    }
    pub fn get_actions(&self) -> &VecDeque<Action> {
        &self.actions
    }
    //This clears the action stack, and deletes all actions
    pub fn clean(&mut self) {
        for action in self.actions.iter_mut() {
            action.clean();
        }
        self.actions.clear();
    }
}

pub mod actions {
    use std::ops::Add;

    use crate::{ecs::EntityManager, IndexType};

    use super::EActionResult;


}
#[cfg(test)]
mod test;
