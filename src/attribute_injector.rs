use std::fmt::{Display, Formatter};
use yew::NodeRef;

/// This trait is used to define how a struct containing attributes should be converted into
/// html attributes and injected to the DOM.
pub trait AttributeInjector {
    /// The inject function takes a NodeRef in which to inject the fields of the struct as html attributes.
    /// You'll need to convert the NodeRef to an Element and use set_attribute.
    fn inject(&mut self, node_ref: &NodeRef) -> Result<(), SetAttributeError>;
}

/// This error indicates that an injection of an attribute to the DOM failed.
#[derive(Debug, Clone)]
pub struct SetAttributeError {
    key: String,
    value: Option<String>,
}

impl SetAttributeError {
    pub fn new(key: String, value: Option<String>) -> Self {
        Self { key, value }
    }
}

impl Display for SetAttributeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = if let Some(value) = &self.value {
            value.to_owned()
        } else {
            "None".into()
        };
        write!(
            f,
            "Failed to insert attribute to the DOM. Key: {} Value: {}",
            self.key, value
        )
    }
}
