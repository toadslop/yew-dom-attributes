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
    attributes_to_remove: HashSet<T>,
}

impl<T> AttributeHolder<T>
where
    T: Attribute + Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            attributes: HashSet::new(),
            attributes_to_remove: HashSet::new(),
        }
    }

    pub fn add_attribute(&mut self, attribute: T) -> bool {
        self.attributes.insert(attribute)
    }

    pub fn remove_attribute(&mut self, attribute: T) -> bool {
        self.attributes.remove(&attribute);
        self.attributes_to_remove.insert(attribute)
    }
}

impl<T> AttributeInjector for AttributeHolder<T>
where
    T: Attribute + Hash + Eq,
{
    fn inject(&mut self, node_ref: &NodeRef) -> Result<(), SetAttributeError> {
        if let Some(elem) = node_ref.cast::<Element>() {
            for attribute in self.attributes_to_remove.iter() {
                match elem.remove_attribute(&attribute.get_key()) {
                    Ok(_) => (),
                    Err(_) => {
                        return Err(SetAttributeError::new(
                            attribute.get_key(),
                            attribute.get_val(),
                        ))
                    }
                }
            }

            self.attributes_to_remove.clear();

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
