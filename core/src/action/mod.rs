//! Code for the action system in Novella.
//! Actions represent something that induces change in the current state of Novella, and also define
//! a method of possibly undoing said change

use crate::ecs::EntityManager;

pub type Result = std::result::Result<(), &'static str>;
///The [`Action`] trait represents the generic interface for an executable action, and it's corrosponding [`Self::undo`].
///
pub trait Action {
    fn exec(&mut self) -> Result;
    fn undo(&mut self) -> Result;
    fn get_undoable(&self) -> bool;
    fn get_is_complete(&self) -> bool;
}
///A structure containing the list of all recent [`Action`]'s
pub struct ActionStack {
    //Maximum amount of actions that will be kept in history
    max_actions: u32,
    actions: Vec<Box<dyn Action>>,
    current_action: usize,
}
impl ActionStack {
    fn new(max_actions: u32) -> Self {
        Self {
            max_actions,
            actions: Default::default(),
            current_action: 0,
        }
    }
    ///Adds an action to the list of actions
    /// # Arguments
    /// * `Action` - The action to be added
    /// # Returns
    /// * `Result` - A Result containing the action that was added, or an error if the action was not added
    /// # Examples
    /// ```
    /// use nv::core::action::Events;
    /// use nv::core::action::Action;
    ///
    /// struct TestAction {
    ///    name: String,
    /// }
    /// impl Action for TestAction {
    ///    fn exec(&mut self) -> Result {
    ///       println!("{}", self.name);
    ///      Ok(())
    ///   }
    ///  fn undo(&mut self) -> Result {
    ///     println!("{}", self.name);
    ///    Ok(())
    /// }
    /// }
    /// let mut events = Events::new(10);
    /// events.add_action(Box::new(TestAction {
    ///   name: String::from("Test Action"),
    /// })).unwrap();
    /// ```
    ///
    /// ```
    pub fn add_action(&mut self, action: Box<dyn Action>) -> Result {
        if self.actions.len() >= self.max_actions as usize {
            self.actions.remove(0);
        }
        //If current action is not the last action, then remove all actions after the current action
        if self.current_action != self.actions.len() - 1 {
            self.actions.drain(self.current_action..);
        }

        self.actions.push(action);
        Ok(())
    }
    ///Undoes the last action in the list of actions
    /// # Returns
    /// * `Result` - A Result containing the action that was undone, or an error if the action was not undone
    /// # Examples
    /// ```
    /// use nv::core::action::Events;
    /// use nv::core::action::Action;
    /// use nv::core::action::Result;
    /// struct TestAction {
    ///   name: String,
    /// }
    /// impl Action for TestAction {
    ///  fn exec(&mut self) -> Result {
    ///    println!("{}", self.name);
    ///   Ok(())
    /// }
    /// fn undo(&mut self) -> Result {
    ///  println!("{}", self.name);
    ///  Ok(())
    /// }
    /// }
    /// let mut events = Events::new(10);
    /// events.add_action(Box::new(TestAction {
    ///  name: String::from("Test Action"),
    /// })).unwrap();
    /// events.undo().unwrap();
    /// ```
    pub fn undo(&mut self) -> Result {
        if self.actions.len() == 0 {
            return Err("Action stack is empty");
        }
        let res = self.actions[self.current_action].undo();
        self.current_action = self.current_action.saturating_sub(1);
        res
    }

    ///Executes the last action in the list of actions
    /// # Returns
    /// * `Result` - A Result containing the action that was executed, or an error if the action was not executed
    ///
    pub fn redo(&mut self) -> Result {
        if self.actions.len() == 0 {
            return Err("Action stack is empty");
        }
        self.actions.last_mut().unwrap().exec()
    }
}

pub mod actions {
    use crate::{ecs::EntityManager, IndexType};

    pub struct AddEntityAction<'a> {
        em: &'a mut EntityManager,
        entity_id: IndexType,
        entity_class: String,
        is_complete: bool,
    }
    impl<'a> AddEntityAction<'a> {
        pub fn new(em: &'a mut EntityManager, entity_class: String) -> Self {
            Self {
                em,
                entity_id: 0,
                entity_class,
                is_complete: false,
            }
        }
    }
    impl<'a> super::Action for AddEntityAction<'a> {
        fn exec(&mut self) -> super::Result {
            self.entity_id = self.em.create_entity(self.entity_class.clone());
            self.is_complete = true;

            Ok(())
        }
        fn undo(&mut self) -> super::Result {
            if (self.is_complete) {
                self.em.delete_entity(self.entity_id);
                Ok(())
            } else {
                Err("The action has not been executed yet and cannot be undone")
            }
        }
        fn get_undoable(&self) -> bool {
            true
        }
        fn get_is_complete(&self) -> bool {
            self.is_complete
        }
    }
    pub struct DeleteEntityAction<'a> {
        em: &'a mut EntityManager,
        entity_id: IndexType,
        entity_class: String,
        is_complete: bool,
    }
    impl<'a> DeleteEntityAction<'a> {
        pub fn new(em: &'a mut EntityManager, entity_id: IndexType) -> Self {
            Self {
                em,
                entity_id,
                entity_class: String::new(),
                is_complete: false,
            }
        }
    }
    impl<'a> super::Action for DeleteEntityAction<'a> {
        fn exec(&mut self) -> super::Result {
            self.entity_class = self
                .em
                .get_entity(self.entity_id)
                .unwrap()
                .entity_class
                .clone();
            self.em.delete_entity(self.entity_id);
            self.is_complete = true;

            Ok(())
        }
        fn undo(&mut self) -> super::Result {
            if (self.is_complete) {
                self.em.create_entity(self.entity_class.clone());
                Ok(())
            } else {
                Err("The action has not been executed yet and cannot be undone")
            }
        }
        fn get_undoable(&self) -> bool {
            true
        }
        fn get_is_complete(&self) -> bool {
            self.is_complete;
            let i=32;
            let i3=3;
            let b=true;
            match i{
                i3=>{
                    println!("{}",i3);
                },

            };
            true


        }
    }
}
#[cfg(test)]
mod test_actions;
