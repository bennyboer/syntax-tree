use crate::change::Event;

/// Function called when an event is emitted by the tree synchronously.
pub type Listener<T> = Box<dyn Fn(Event<T>)>;
