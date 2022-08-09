use std::{collections::HashMap, rc::Rc};

use domatt::{Attribute, HtmlAttributes};
use yew::{Callback, Context, Properties};

use crate::events::events::EventType;

use super::{aria_props::AriaPropsHandler, custom_attributes::CustomPropsHandler, DomInjector};

/// Properties for a generic Yew components.
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct HtmlElementProps {
    attributes_to_add: HashMap<String, Option<String>>,
    attributes_to_remove: Vec<String>,
    listeners_to_add: HashMap<String, EventType>,
    listeners_to_remove: Vec<String>,
    /// A callback used to pass changes to button props from the child back up to the parent.
    /// This is necessary to inform the parent that attributes and listeners were either
    /// added or removed from the DOM. If this is not used properly, your component will
    /// not know that it happened and will try again on the next rerender.
    on_props_update: Callback<Rc<HtmlElementProps>>,
}

/// A trait to be implemented by any type that accepts button props.
pub trait HtmlElementPropsHandler: super::private::PropsGetterSetter {
    fn add_html_prop(&mut self, prop: HtmlAttributes) {
        let key = prop.get_key();
        let val = prop.get_val();
        self.get_props_to_add().insert(key, val);
    }

    fn remove_html_prop(&mut self, prop: HtmlAttributes) {
        self.get_props_to_remove().push(prop.to_string())
    }
}

impl super::private::PropsGetterSetter for HtmlElementProps {
    fn get_props_to_add(&mut self) -> &mut HashMap<String, Option<String>> {
        &mut self.attributes_to_add
    }

    fn get_props_to_remove(&mut self) -> &mut Vec<String> {
        &mut self.attributes_to_remove
    }
}

impl super::private::ListenerGetterSetter for HtmlElementProps {
    fn get_listeners_to_add(&mut self) -> &mut HashMap<String, EventType> {
        &mut self.listeners_to_add
    }

    fn get_listeners_to_remove(&mut self) -> &mut Vec<String> {
        &mut self.listeners_to_remove
    }
}

impl AriaPropsHandler for HtmlElementProps {}
impl HtmlElementPropsHandler for HtmlElementProps {}
impl CustomPropsHandler for HtmlElementProps {}
impl DomInjector for HtmlElementProps {
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
