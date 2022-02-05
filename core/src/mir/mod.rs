mod action;
use std::collections::VecDeque;

use crate::ecs::EntityManager;
use crate::{IndexType, Manuscript, Project};
use action::{Action, ActionStack};

///Mir is the interface by which to communicate with internal kernel. It is the only way to interact with the kernel.
//It is a "mirror" of all actions that can be performed from outside the kernel.
///The exec method of an event takes a reference to the mir, and mir is responsible for execution.
/// Mir owns all data in the kernel;
pub struct Mir<'a> {
    pub em: EntityManager,
    pub proj: Project,
    pub event_queue: EventQueue,
    pub action_stack: ActionStack<'a>,
}
impl<'a> Mir<'a> {
    fn new() -> Mir<'a> {
        Mir {
            em: EntityManager::new(),
            proj: Project::new_empty(),
            event_queue: EventQueue::new(),
            action_stack: ActionStack::new(20),
        }
    }
    pub fn say_hello(&self) {
        println!("Hello from Mir");
    }
}

//An event is recieved from the clientside and describes something that needs to be done. Mir is responsible for executing the event.
// An event is different from an action in that an event is not undoable and comes directly from the client.
//Mir takes an event, converts it to an action if appropriate, and then executes the action.
//A single event can be translated to multiple actions.

pub trait Returnable {}
pub trait Event {
    fn exec(&self, mir: &mut Mir);
}
pub struct EventQueue {
    pub events: VecDeque<Box<dyn Event>>,
}
impl EventQueue {
    fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }
    fn add_event(&mut self, event: impl Event + 'static) {
        self.events.push_back(Box::new(event));
    }
}
