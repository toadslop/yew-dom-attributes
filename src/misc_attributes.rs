use std::{
    collections::LinkedList,
    fmt::{Debug, Display},
};

use strum::Display;
use web_sys::Element;
use yew::NodeRef;

use crate::attribute_collection::AttributeCollection;

/// A struct representing an artibrary set of HTML attributes to be passed to the underlying component.
/// Should be used as a prop for a Yew component.
#[derive(Default)]
pub struct MiscAttrs(LinkedList<Box<dyn DomAttribute>>);

impl MiscAttrs {
    pub fn new() -> Self {
        Self(LinkedList::new())
    }

    /// Add a key-value pair attribute. This will be rendered to the DOM like this:
    /// <div key="value">
    pub fn add_attribute(&mut self, key: Box<dyn DomAttribute>) {
        self.0.push_back(key);
    }
}

impl AttributeCollection for MiscAttrs {
    /// Call the render method within the rendered method of a component, passing in the NodeRef of the component.
    /// This will then inject the props.
    fn inject(&self, node_ref: &NodeRef) {
        if let Some(elem) = node_ref.cast::<Element>() {
            for key in &self.0 {
                elem.set_attribute(key.to_string().as_str(), key.get_value().as_str())
                    .unwrap();
            }
        }
    }
}

pub trait DomAttribute: Display {
    fn get_value(&self) -> String;
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum AriaAttributes {
    AriaAutocomplete(AriaAutocomplete),
    AriaActivedescendant(String),
}

impl DomAttribute for AriaAttributes {
    fn get_value(&self) -> String {
        match self {
            AriaAttributes::AriaAutocomplete(value) => value.to_string(),
            AriaAttributes::AriaActivedescendant(value) => value.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display)]
#[strum(serialize_all = "kebab-case")]
pub enum AriaAutocomplete {
    None,
    Inline,
    List,
    Both,
}
