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
    use super::ProcessAction;
    use domatt::events::Event;
    use std::rc::Rc;

    pub trait PropsGetterSetter {
        fn get_attributes(&mut self) -> &mut Vec<(ProcessAction, String, Option<String>)>;
    }

    pub trait ListenerGetterSetter {
        fn get_listeners(&mut self) -> &mut Vec<(ProcessAction, String, Option<Rc<dyn Event>>)>;
    }
}

/// A trait to be implemented by any element that accepts listeners.
/// Handles adding and removing listeners as well as injecting them to the DOM.
pub trait DomInjector: private::ListenerGetterSetter + private::PropsGetterSetter {
    /// Creates a simple new DOM injector instance.
    fn new<T: Component>() -> Self
    where
        T: yew::Component;

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
            let listeners = self.get_listeners();
            inject_listeners(&elem, active_listeners, listeners);

            let attributes = self.get_attributes();
            inject_attributes(&elem, attributes);
        }
    }
}

fn inject_attributes(
    elem: &Element,
    attributes: &mut Vec<(ProcessAction, String, Option<String>)>,
) {
    for (action, key, value) in attributes.drain(..) {
        match action {
            ProcessAction::Add => elem
                .set_attribute(&key, &value.unwrap_or_default())
                .expect("that there should be no problem adding the attribute."),
            ProcessAction::Remove => elem
                .remove_attribute(&key)
                .expect("that there should be no problem removing the attribute"),
        }
    }
}

fn inject_listeners(
    elem: &Element,
    active_listeners: &mut HashMap<String, Rc<EventListener>>,
    listeners: &mut Vec<(ProcessAction, String, Option<Rc<dyn Event>>)>,
) {
    let mut listener_holder = HashMap::new();
    for (action, listener_id, event) in listeners.drain(..) {
        match action {
            ProcessAction::Add => {
                let event = event
                    .expect("there to be an event. This is a logic error and a bug in the code.");
                let event_type = event.get_event_type().to_owned();
                let cb = event.get_callback();
                let listener = EventListener::new(elem, event_type, move |ev| (*cb)(ev));
                listener_holder.insert(listener_id, Rc::new(listener));
            }
            ProcessAction::Remove => {
                active_listeners.remove(&listener_id);
            }
        };
    }
    active_listeners.extend(listener_holder);
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessAction {
    Add,
    Remove,
}

macro_rules! prop_handler {
    ($name:ident, $attr_type:ident ) => {
        #[derive(Debug, Properties, PartialEq, Clone)]
        pub struct $name {
            attributes: Vec<(ProcessAction, String, Option<String>)>,
            listeners: Vec<(ProcessAction, String, Option<Rc<dyn Event>>)>,
            /// A callback used to pass changes to button props from the child back up to the parent.
            /// This is necessary to inform the parent that attributes and listeners were either
            /// added or removed from the DOM. If this is not used properly, your component will
            /// not know that it happened and will try again on the next rerender.
            on_props_update: Option<Callback<Rc<$name>>>,
        }

        impl $name {
            pub fn add_attribute(&mut self, attribute: Box<dyn $attr_type>) {
                let action = ProcessAction::Add;
                let key = String::from(attribute.get_key());
                let val = attribute.get_val().map(String::from);
                self.attributes.push((action, key, val))
            }

            pub fn remove_attribute(&mut self, key: &str) {
                let action = ProcessAction::Remove;
                self.attributes.push((action, key.to_owned(), None))
            }

            pub fn add_listener(&mut self, id: &str, event: Rc<dyn Event>) {
                let action = ProcessAction::Add;
                self.listeners.push((action, id.to_owned(), Some(event)))
            }

            pub fn remove_listener(&mut self, id: String) {
                let action = ProcessAction::Remove;
                self.listeners.push((action, id.to_owned(), None))
            }
        }

        impl super::private::PropsGetterSetter for $name {
            fn get_attributes(&mut self) -> &mut Vec<(ProcessAction, String, Option<String>)> {
                &mut self.attributes
            }
        }

        impl super::private::ListenerGetterSetter for $name {
            fn get_listeners(
                &mut self,
            ) -> &mut Vec<(ProcessAction, String, Option<Rc<dyn Event>>)> {
                &mut self.listeners
            }
        }

        impl DomInjector for $name {
            fn new<T>() -> Self
            where
                T: yew::Component,
            {
                Self {
                    attributes: Vec::new(),
                    listeners: Vec::new(),
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
                    attributes: Vec::new(),
                    listeners: Vec::new(),
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
