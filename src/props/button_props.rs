use domatt::{Attribute, ButtonHtmlAttributes};
use std::{collections::HashMap, rc::Rc};
use yew::{Callback, Properties};

use crate::events::events::EventType;

use super::{
    aria_props::AriaPropsHandler, custom_attributes::CustomPropsHandler,
    html_element_props::HtmlElementPropsHandler, DomInjector,
};

/// Button properties for a Yew button component.
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ButtonProps {
    attributes_to_add: HashMap<String, Option<String>>,
    attributes_to_remove: Vec<String>,
    listeners_to_add: HashMap<String, EventType>,
    listeners_to_remove: Vec<String>,
    /// A callback used to pass changes to button props from the child back up to the parent.
    /// This is necessary to inform the parent that attributes and listeners were either
    /// added or removed from the DOM. If this is not used properly, your component will
    /// not know that it happened and will try again on the next rerender.
    on_props_update: Callback<Rc<ButtonProps>>,
}

impl super::private::PropsGetterSetter for ButtonProps {
    fn get_props_to_add(&mut self) -> &mut HashMap<String, Option<String>> {
        &mut self.attributes_to_add
    }

    fn get_props_to_remove(&mut self) -> &mut Vec<String> {
        &mut self.attributes_to_remove
    }
}

impl super::private::ListenerGetterSetter for ButtonProps {
    fn get_listeners_to_add(&mut self) -> &mut HashMap<String, EventType> {
        &mut self.listeners_to_add
    }

    fn get_listeners_to_remove(&mut self) -> &mut Vec<String> {
        &mut self.listeners_to_remove
    }
}

/// A trait to be implemented by any type that accepts button props.
pub trait ButtonPropsHandler: super::private::PropsGetterSetter {
    fn add_button_prop(&mut self, prop: ButtonHtmlAttributes) {
        let key = prop.get_key();
        let val = prop.get_val();
        self.get_props_to_add().insert(key, val);
    }

    fn remove_button_prop(&mut self, prop: ButtonHtmlAttributes) {
        self.get_props_to_remove().push(prop.to_string())
    }
}

impl ButtonPropsHandler for ButtonProps {}
impl AriaPropsHandler for ButtonProps {}
impl HtmlElementPropsHandler for ButtonProps {}
impl CustomPropsHandler for ButtonProps {}

impl DomInjector for ButtonProps {
    fn new(on_props_update: Callback<Rc<Self>>) -> Self {
        Self {
            attributes_to_add: HashMap::new(),
            attributes_to_remove: Vec::new(),
            listeners_to_add: HashMap::new(),
            listeners_to_remove: Vec::new(),
            on_props_update,
        }
    }

    fn get_props_update_callback(&self) -> &Callback<Rc<Self>> {
        &self.on_props_update
    }
}
