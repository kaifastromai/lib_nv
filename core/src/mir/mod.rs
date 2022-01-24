mod action;
use std::collections::VecDeque;

use crate::ecs::{Components, EntityManager};
use crate::{IndexType, Manuscript, Project};
use action::{Action, ActionStack};

///Mir is the interface by which to communicate with internal kernel. It is the only way to interact with the kernel.
//It is a "mirror" of all actions that can be performed from outside the kernel.
///The exec method of an event takes a reference to the mir, and mir is responsible for executing the event.
/// Mir owns all data in the kernel, and is responsible for updating the kernel.
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
    //Starts mir service. Mir will start listening for events from the client, and consume them.
    pub fn exec(&mut self) {
        while let Some(event) = self.event_queue.events.pop_front() {
            event.exec(self);
        }
    }
    //Mir must expose all public functions of EntityManager and Project
    //------Project functions------
    pub fn add_manuscript(&mut self, manuscript: Manuscript) {
        self.proj.add_manuscript(manuscript);
    }
    pub fn get_manuscript(&self, id: IndexType) -> Option<&Manuscript> {
        self.proj.get_manuscript(id)
    }
    pub fn get_manuscript_mut(&mut self, id: IndexType) -> Option<&mut Manuscript> {
        self.proj.get_manuscript_mut(id)
    }
    pub fn remove_manuscript(&mut self, id: IndexType) {
        self.proj.remove_manuscript(id);
    }
    pub fn get_all_manuscripts(&self) -> Vec<&Manuscript> {
        self.proj.get_all_manuscripts()
    }
    pub fn get_all_live_references(&self) -> Vec<crate::Reference> {
        self.proj.get_all_live_references()
    }
    //------EntityManager functions------
    pub fn create_entity(&mut self, entity_class: String) -> IndexType {
        self.em.create_entity(entity_class)
    }
    pub fn delete_entity(&mut self, id: IndexType) {
        self.em.delete_entity(id);
    }
    pub fn add_component<T: crate::ecs::Component>(&mut self, id: IndexType, props: T::Properties) {
        self.em.add_component::<T>(id, props);
    }
    pub fn get_component<T: crate::ecs::Component>(&self, id: IndexType) -> Option<&T> {
        self.em.get_component::<T>(id)
    }
    pub fn get_component_mut<T: crate::ecs::Component>(&mut self, id: IndexType) -> Option<&mut T> {
        self.em.get_component_mut::<T>(id)
    }

    pub fn get_entity(&self, id: IndexType) -> Option<&crate::ecs::Entity> {
        self.em.get_entity(id)
    }
    pub fn get_entity_mut(&mut self, id: IndexType) -> Option<&mut crate::ecs::Entity> {
        self.em.get_entity_mut(id)
    }
    pub fn get_entities_by_class(&self, class: &str) -> Vec<&crate::ecs::Entity> {
        self.em.get_entities_by_class(class)
    }
    pub fn get_entities_by_class_mut(&mut self, class: &str) -> Vec<&mut crate::ecs::Entity> {
        self.em.get_entities_by_class_mut(class)
    }
    pub fn add_from_entity_graph(&mut self, entity_graph: crate::ecs::EntityGraph) {
        self.em.add_from_entity_graph(entity_graph);
    }
    pub fn merge_components(&mut self, components: Components) {
        self.em.merge_components(components);
    }
    pub fn strip_entity(&mut self, id: IndexType) {
        self.em.strip_entity(id);
    }
    pub fn get_components(&self, id: IndexType) -> Components {
        self.em.get_components(id)
    }
    fn mark_entity_for_deletion(&mut self, id: IndexType) {
        self.em.mark_entity_for_deletion(id);
    }
    fn unmark_entity_for_deletion(&mut self, id: IndexType) {
        self.em.unmark_entity_for_deletion(id);
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
    fn add_event(&mut self, event:impl Event + 'static) {
        self.events.push_back(Box::new(event));
    }   
}
