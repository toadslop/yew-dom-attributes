use std::fmt::{Display, Formatter};
use yew::NodeRef;

/// This trait is used to define how a struct containing event callbacks should be converted into
/// JavaScript and injected to the DOM.
pub trait ListenerInjector {
    /// The inject function takes a NodeRef in which to set the event listeners.
    /// You'll need to convert the NodeRef to an Element and use set_attribute.
    fn inject_listeners(&mut self, node_ref: &NodeRef) -> Result<(), AddListenerError>;

    // This function should be called on the destroy method of the Yew component.
    // It should remove all the listeners.
    // fn cleanup_listeners(&mut self, node_ref: &NodeRef) -> Result<(), RemoveListenerError>;
}

/// This error indicates that an injection of an attribute to the DOM failed.
#[derive(Debug, Clone)]
pub struct AddListenerError {
    listener_type: String,
}

/// This error indicates that an injection of an attribute to the DOM failed.
#[derive(Debug, Clone)]
pub struct RemoveListenerError {
    listener_type: String,
}

impl AddListenerError {
    pub fn new(listener_type: String) -> Self {
        Self { listener_type }
    }
}

impl Display for AddListenerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to insert attribute to the DOM. Listener type: {}",
            self.listener_type
        )
    }
}

impl RemoveListenerError {
    pub fn new(listener_type: String) -> Self {
        Self { listener_type }
    }
}

impl Display for RemoveListenerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to insert attribute to the DOM. Listener type: {}",
            self.listener_type
        )
    }
}
