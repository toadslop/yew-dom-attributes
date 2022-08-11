use super::{custom_attributes::CustomPropsHandler, DomInjector};
use crate::events::events::EventType;
use std::{collections::HashMap, rc::Rc};
use yew::{Callback, Context, Properties};

/// Button properties for a Yew button component.
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct CustomProps {
    attributes_to_add: HashMap<String, Option<String>>,
    attributes_to_remove: Vec<String>,
    listeners_to_add: HashMap<String, EventType>,
    listeners_to_remove: Vec<String>,
    /// A callback used to pass changes to button props from the child back up to the parent.
    /// This is necessary to inform the parent that attributes and listeners were either
    /// added or removed from the DOM. If this is not used properly, your component will
    /// not know that it happened and will try again on the next rerender.
    on_props_update: Callback<Rc<CustomProps>>,
}

impl super::private::PropsGetterSetter for CustomProps {
    fn get_props_to_add(&mut self) -> &mut HashMap<String, Option<String>> {
        &mut self.attributes_to_add
    }

    fn get_props_to_remove(&mut self) -> &mut Vec<String> {
        &mut self.attributes_to_remove
    }
}

impl super::private::ListenerGetterSetter for CustomProps {
    fn get_listeners_to_add(&mut self) -> &mut HashMap<String, EventType> {
        &mut self.listeners_to_add
    }

    fn get_listeners_to_remove(&mut self) -> &mut Vec<String> {
        &mut self.listeners_to_remove
    }
}

impl CustomPropsHandler for CustomProps {}

impl DomInjector for CustomProps {
    fn new<T, F, R>(ctx: &Context<T>, func: F) -> Self
    where
        F: Fn(Rc<Self>) -> R + 'static,
        T: yew::Component,
        <T as yew::Component>::Message: std::convert::From<R>,
    {
        let on_props_update = ctx.link().callback(func);

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
