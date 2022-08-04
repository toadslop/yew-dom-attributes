use std::collections::HashMap;

use web_sys::Element;
use yew::NodeRef;

use crate::attribute_injector::{AttributeInjector, SetAttributeError};

pub trait CustomPropsHandler: super::private::PropsGetterSetter {
    fn add_custom_prop(&mut self, prop: CustomAttribute) {
        let key = prop.get_key();
        let val = prop.get_value();
        self.get_props_to_add().insert(key, val);
    }

    fn remove_custom_prop(&mut self, prop: CustomAttribute) {
        self.get_props_to_remove().push(prop.get_key())
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CustomAttribute {
    key: String,
    value: Option<String>,
}

impl CustomAttribute {
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value: Some(value),
        }
    }

    pub fn new_boolean_attribute(key: String) -> Self {
        Self { key, value: None }
    }

    fn get_key(&self) -> String {
        self.key.clone()
    }

    fn get_value(&self) -> Option<String> {
        self.value.clone()
    }
}

// Old stuff to delete
// replace later
// delete

/// A struct representing an artibrary set of HTML attributes to be passed to the underlying component.
/// Should be used as a prop for a Yew component.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct CustomAttrs(HashMap<String, Option<String>>);

impl CustomAttrs {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Add a key-value pair attribute. This will be rendered to the DOM like this:
    /// <div key="value">
    pub fn add_attribute(&mut self, key: String, value: String) -> bool {
        match self.0.insert(key, Some(value)) {
            Some(_) => true,
            None => false,
        }
    }

    /// Add a boolean attribute to the element. It will take a single key and render it to the DOM
    /// as follows:
    /// <div key ></div>
    pub fn add_boolean_attribute(&mut self, key: String) -> bool {
        match self.0.insert(key, None) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn remove_attribute(&mut self, key: String) -> bool {
        match self.0.remove(&key) {
            Some(_) => true,
            None => false,
        }
    }
}

impl From<HashMap<String, Option<String>>> for CustomAttrs {
    fn from(base_map: HashMap<String, Option<String>>) -> Self {
        Self(base_map)
    }
}

impl AttributeInjector for CustomAttrs {
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

pub trait CustomAttributeReceiver {
    fn add_attribute(&mut self, key: String, value: String) -> bool;

    fn remove_attribute(&mut self, key: String) -> bool;

    fn add_boolean_attribute(&mut self, key: String) -> bool;
}
