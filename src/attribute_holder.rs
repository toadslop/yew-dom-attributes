use std::{collections::HashSet, fmt::Display, hash::Hash};

use web_sys::Element;
use yew::{NodeRef, Properties};

use crate::attribute_injector::{AttributeInjector, SetAttributeError};

#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct AttributeHolder<T>
where
    T: Attribute + Hash + Eq,
{
    attributes: HashSet<T>,
}

impl<T> AttributeHolder<T>
where
    T: Attribute + Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            attributes: HashSet::new(),
        }
    }

    pub fn add_attribute(&mut self, attribute: T) {
        self.attributes.insert(attribute);
    }
}

impl<T> AttributeInjector for AttributeHolder<T>
where
    T: Attribute + Hash + Eq,
{
    fn inject(&self, node_ref: &NodeRef) -> Result<(), SetAttributeError> {
        if let Some(elem) = node_ref.cast::<Element>() {
            for attribute in self.attributes.iter() {
                match elem.set_attribute(
                    &attribute.get_key(),
                    &attribute.get_val().unwrap_or_default(),
                ) {
                    Ok(()) => (),
                    Err(_) => {
                        return Err(SetAttributeError::new(
                            attribute.get_key(),
                            attribute.get_val(),
                        ))
                    }
                }
            }
        }
        Ok(())
    }
}

pub trait Attribute: Display {
    fn get_key(&self) -> String;
    fn get_val(&self) -> Option<String>;
}
