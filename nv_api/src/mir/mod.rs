use std::collections::VecDeque;

use nvcore::ecs::EntityManager;
use nvcore::Project;

pub struct Action(pub fn(*mut Mir));

pub struct Mir {
    em: EntityManager,
    proj: Project,
    event_queue: EventQueue,
    action_queue: VecDeque<Action>,
}
impl Mir {
    pub fn new() -> Self {
        Mir {
            em: EntityManager::new(),
            proj: Project::new_empty(),
            event_queue: EventQueue::new(),
            action_queue: VecDeque::new(),
        }
    }
    pub extern "C" fn say_hello(&self) {
        println!("Hello from Mir");
    }
}

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
