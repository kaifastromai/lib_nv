///The event system maintains a queue of events to be executed by the kernel.
///  Events are added to the queue when the server receives an event from the client.
///  The event system must handle distributing the actions to the appropriate systems within the kernel.
///
use nvcore::mir::{Mir, EventQueue,Event};

pub struct UndoEvent{

}
pub struct CreateProjectEvent{
    

}




impl Event for UndoEvent{
    fn exec(&self, mir: &mut Mir){
        mir.action_stack.undo().unwrap();
    }
}
