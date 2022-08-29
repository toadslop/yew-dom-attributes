use domatt::events::Event;
use gloo_events::EventListener;
use std::{collections::HashMap, rc::Rc};
use web_sys::Element;
use yew::{Callback, Component, Context, NodeRef};

pub mod anchor_props;
pub mod button_props;
pub mod global_props;
pub mod svg_props;

mod private {
    use domatt::events::Event;
    use std::{collections::HashMap, rc::Rc};

    pub trait PropsGetterSetter {
        fn get_attributes(&mut self) -> &mut HashMap<String, Option<String>>;
    }

    pub trait ListenerGetterSetter {
        fn get_listeners(&mut self) -> &mut HashMap<String, Option<Rc<dyn Event>>>;
    }
}

/// A trait to be implemented by any element that accepts listeners.
/// Handles adding and removing listeners as well as injecting them to the DOM.
pub trait DomInjector: private::ListenerGetterSetter + private::PropsGetterSetter {
    /// Creates a simple new DOM injector instance.
    fn new() -> Self;

    /// This function is used when you need to dynamically update the props using events.
    /// Once the changes to the props are complete, this will return the updated props,
    /// which you can store as state inside the component that created the props instance.
    fn with_update_callback<T: Component, F, R>(ctx: &Context<T>, func: F) -> Self
    where
        F: Fn(Rc<Self>) -> R + 'static,
        T: yew::Component,
        <T as yew::Component>::Message: std::convert::From<R>;

    /// This function returns a callback that takes the props struct itelf. This is used
    /// to pass changes to props struct from the child back up to the parent.
    /// This is necessary to inform the parent that attributes and listeners were either
    /// added or removed from the DOM. If this is not used properly, your component will
    /// not know that it happened and will try again on the next rerender.
    fn get_props_update_callback(&self) -> Option<&Callback<Rc<Self>>>;

    /// The active_listeners parameter should be stored in the host Component so the listeners it contained will be
    /// dropped when that Component is destroyed.
    fn inject(
        &mut self,
        node_ref: &NodeRef,
        active_listeners: &mut HashMap<String, Rc<EventListener>>,
    ) where
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

fn inject_attributes(elem: &Element, attributes: &mut HashMap<String, Option<String>>) {
    for attr_name in elem.get_attribute_names().iter() {
        let name = &attr_name
            .as_string()
            .expect("an attribute name to be representable as a string");
        if name != "class" {
            elem.remove_attribute(name)
                .expect("removing an attribute to be successful");
        }
    }
    for (key, value) in attributes.drain() {
        elem.set_attribute(key.as_ref(), &value.unwrap_or_default())
            .expect("that there should be no problem adding the attribute.")
    }
}

fn inject_listeners(
    elem: &Element,
    active_listeners: &mut HashMap<String, Rc<EventListener>>,
    listeners: &mut HashMap<String, Option<Rc<dyn Event>>>,
) {
    let mut listener_holder = HashMap::new();
    for (_id, listener) in active_listeners.drain() {
        drop(listener);
    }

    for (listener_id, event) in listeners.drain() {
        let event =
            event.expect("there to be an event. This is a logic error and a bug in the code.");
        let event_type = event.get_event_type().to_owned();
        let cb = event.get_callback();
        let listener = EventListener::new(elem, event_type, move |ev| (*cb)(ev));
        listener_holder.insert(listener_id, Rc::new(listener));
    }
    active_listeners.extend(listener_holder);
}

macro_rules! prop_handler {
    ($name:ident, $attr_type:ident ) => {
        #[derive(Debug, Properties, PartialEq, Clone)]
        pub struct $name {
            attributes: HashMap<String, Option<String>>,
            listeners: HashMap<String, Option<Rc<dyn Event>>>,
            /// A callback used to pass changes to button props from the child back up to the parent.
            /// This is necessary to inform the parent that attributes and listeners were either
            /// added or removed from the DOM. If this is not used properly, your component will
            /// not know that it happened and will try again on the next rerender.
            on_props_update: Option<Callback<Rc<$name>>>,
        }

        impl $name {
            pub fn add_attribute(&mut self, attribute: Box<dyn $attr_type>) {
                let key = attribute.get_key().to_owned();
                let val = attribute.get_val().map(String::from);
                self.attributes.insert(key, val);
            }

            pub fn remove_attribute(&mut self, key: &str) {
                self.attributes.remove(key);
            }

            pub fn has_attribute(&self, key: &str) -> bool {
                self.attributes.contains_key(key)
            }

            pub fn add_listener(&mut self, id: &str, event: Rc<dyn Event>) {
                self.listeners.insert(id.to_owned(), Some(event));
            }

            pub fn remove_listener(&mut self, id: String) {
                self.listeners.remove(&id);
            }
        }

        impl super::private::PropsGetterSetter for $name {
            fn get_attributes(&mut self) -> &mut HashMap<String, Option<String>> {
                &mut self.attributes
            }
        }

        impl super::private::ListenerGetterSetter for $name {
            fn get_listeners(&mut self) -> &mut HashMap<String, Option<Rc<dyn Event>>> {
                &mut self.listeners
            }
        }

        impl DomInjector for $name {
            fn new() -> Self {
                Self {
                    attributes: HashMap::new(),
                    listeners: HashMap::new(),
                    on_props_update: None,
                }
            }

            fn with_update_callback<T, F, R>(ctx: &Context<T>, func: F) -> Self
            where
                F: Fn(Rc<Self>) -> R + 'static,
                T: yew::Component,
                <T as yew::Component>::Message: std::convert::From<R>,
            {
                let on_props_update = Some(ctx.link().callback(func));

                Self {
                    attributes: HashMap::new(),
                    listeners: HashMap::new(),
                    on_props_update,
                }
            }

            fn get_props_update_callback(&self) -> Option<&Callback<Rc<Self>>> {
                if let Some(cb) = &self.on_props_update {
                    Some(cb)
                } else {
                    None
                }
            }
        }
    };
}

pub(crate) use prop_handler;
