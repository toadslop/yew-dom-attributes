use domatt::events::Event;
use gloo_events::EventListener;
use std::{collections::HashMap, rc::Rc};
use web_sys::Element;
use yew::NodeRef;

pub mod anchor_props;
pub mod button_props;
pub mod global_props;
pub mod svg_props;

mod private {
    use domatt::events::Event;
    use std::{collections::HashMap, rc::Rc};

    pub trait PropsGetterSetter {
        fn get_attributes(&self) -> &HashMap<String, Option<String>>;
    }

    pub trait ListenerGetterSetter {
        fn get_listeners(&self) -> &HashMap<String, Rc<dyn Event>>;
    }
}

/// A trait to be implemented by any element that accepts listeners.
/// Handles adding and removing listeners as well as injecting them to the DOM.
pub trait DomInjector: private::ListenerGetterSetter + private::PropsGetterSetter {
    /// Creates a simple new DOM injector instance.
    fn new() -> Self;

    /// The active_listeners parameter should be stored in the host Component so the listeners it contained will be
    /// dropped when that Component is destroyed.
    fn inject(&self, node_ref: &NodeRef, active_listeners: &mut HashMap<String, EventListener>)
    where
        Self: Sized,
    {
        if let Some(elem) = node_ref.cast::<Element>() {
            let attributes = self.get_attributes();
            inject_attributes(&elem, attributes);

            let listeners = self.get_listeners();
            inject_listeners(&elem, active_listeners, listeners);
        }
    }
}

fn inject_attributes(elem: &Element, attributes: &HashMap<String, Option<String>>) {
    for attr_name in elem.get_attribute_names().iter() {
        let name = &attr_name
            .as_string()
            .expect("an attribute name to be representable as a string");
        if name != "class" {
            elem.remove_attribute(name)
                .expect("removing an attribute to be successful");
        }
    }
    for (key, value) in attributes.iter() {
        elem.set_attribute(key.as_ref(), &value.clone().unwrap_or_default())
            .expect("that there should be no problem adding the attribute.")
    }
}

fn inject_listeners(
    elem: &Element,
    active_listeners: &mut HashMap<String, EventListener>,
    listeners: &HashMap<String, Rc<dyn Event>>,
) {
    active_listeners.retain(|event_id, _| listeners.contains_key(event_id));

    let mut listener_holder = HashMap::new();

    for (listener_id, event) in listeners.iter() {
        if !active_listeners.contains_key(listener_id) {
            let event_type = event.get_event_type().to_owned();
            let cb = event.get_callback();
            let listener = EventListener::new(elem, event_type, move |ev| (*cb)(ev));
            listener_holder.insert(listener_id.to_owned(), listener);
        }
    }
    active_listeners.extend(listener_holder);
}

macro_rules! prop_handler {
    ($name:ident, $attr_type:ident ) => {
        #[derive(Debug, Properties, PartialEq, Clone)]
        pub struct $name {
            attributes: HashMap<String, Option<String>>,
            listeners: HashMap<String, Rc<dyn Event>>,
        }

        impl $name {
            pub fn add_attribute(&mut self, attribute: Box<dyn $attr_type>) -> Option<String> {
                let key = attribute.get_key().to_owned();
                let val = attribute.get_val().map(String::from);
                match self.attributes.insert(key, val) {
                    Some(attr) => attr,
                    None => None,
                }
            }

            pub fn remove_attribute(&mut self, key: &str) -> Option<String> {
                match self.attributes.remove(key) {
                    Some(attr) => attr,
                    None => None,
                }
            }

            pub fn has_attribute(&self, key: &str) -> bool {
                self.attributes.contains_key(key)
            }

            pub fn has_event_type(&self, key: &str) -> bool {
                for (_, event) in self.listeners.iter() {
                    if event.get_event_type() == key {
                        return true;
                    };
                }
                false
            }

            pub fn get_attribute(&self, key: &str) -> Option<&String> {
                match self.attributes.get(key) {
                    Some(attr) => attr.as_ref().map(|val| val),
                    None => None,
                }
            }

            pub fn add_listener(&mut self, id: &str, event: Rc<dyn Event>) {
                self.listeners.insert(id.to_owned(), event);
            }

            pub fn remove_listener(&mut self, id: String) {
                self.listeners.remove(&id);
            }
        }

        impl super::private::PropsGetterSetter for $name {
            fn get_attributes(&self) -> &HashMap<String, Option<String>> {
                &self.attributes
            }
        }

        impl super::private::ListenerGetterSetter for $name {
            fn get_listeners(&self) -> &HashMap<String, Rc<dyn Event>> {
                &self.listeners
            }
        }

        impl DomInjector for $name {
            fn new() -> Self {
                Self {
                    attributes: HashMap::new(),
                    listeners: HashMap::new(),
                }
            }
        }
    };
}

pub(crate) use prop_handler;
