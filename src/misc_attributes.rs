use std::collections::HashMap;

use web_sys::Element;
use yew::NodeRef;

use crate::attribute_injector::{AttributeInjector, SetAttributeError};

/// A struct representing an artibrary set of HTML attributes to be passed to the underlying component.
/// Should be used as a prop for a Yew component.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct MiscAttrs(HashMap<String, Option<String>>);

impl MiscAttrs {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Add a key-value pair attribute. This will be rendered to the DOM like this:
    /// <div key="value">
    pub fn add_attribute(&mut self, key: String, value: String) {
        self.0.insert(key, Some(value));
    }

    /// Add a boolean attribute to the element. It will take a single key and render it to the DOM
    /// as follows:
    /// <div key ></div>
    pub fn add_boolean_attribute(&mut self, key: String) {
        self.0.insert(key, None);
    }
}

impl From<HashMap<String, Option<String>>> for MiscAttrs {
    fn from(base_map: HashMap<String, Option<String>>) -> Self {
        Self(base_map)
    }
}

impl AttributeInjector for MiscAttrs {
    /// Call the render method within the rendered method of a component, passing in the NodeRef of the component.
    /// This will then inject the props.
    fn inject(&mut self, node_ref: &NodeRef) -> Result<(), SetAttributeError> {
        if let Some(elem) = node_ref.cast::<Element>() {
            for (key, maybe_val) in &self.0 {
                let val = maybe_val.clone().unwrap_or("".to_string());
                match elem.set_attribute(&key, &val) {
                    Ok(()) => (),
                    Err(_msg) => return Err(SetAttributeError::new(key.to_owned(), Some(val))),
                }
            }
        }
        Ok(())
    }
}
