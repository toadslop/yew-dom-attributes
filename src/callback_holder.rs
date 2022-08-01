use nanoid::nanoid;
use std::{collections::HashMap, hash::Hash};
use wasm_bindgen::{closure::WasmClosure, prelude::Closure, JsCast};
use web_sys::Element;
use yew::{html::onabort::Event, NodeRef};

use crate::listener_injector::{AddListenerError, ListenerInjector, RemoveListenerError};

#[derive(Debug)]
struct ClosureWrapper<T>(Closure<T>)
where
    Closure<T>: WasmClosure;

impl<T> Hash for ClosureWrapper<T>
where
    Closure<T>: WasmClosure,
    T: WasmClosure,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.into_js_value().as_string().hash(state);
    }
}

#[derive(Debug)]
pub struct CallbackHolder {
    callbacks: HashMap<String, (String, Closure<dyn FnMut(web_sys::Event)>)>,
    callbacks_to_remove: HashMap<String, (String, Closure<dyn FnMut(web_sys::Event)>)>,
}

impl CallbackHolder {
    pub fn new() -> Self {
        Self {
            callbacks: HashMap::new(),
            callbacks_to_remove: HashMap::new(),
        }
    }

    pub fn add_callback(
        &mut self,
        callback: impl Callback + 'static,
    ) -> Option<(String, Closure<(dyn FnMut(Event) + 'static)>)> {
        let id = nanoid!();
        let event_type = callback.get_event_type().clone();
        let closure = Closure::<dyn FnMut(web_sys::Event)>::new(move |event: web_sys::Event| {
            callback.get_callback().emit(event)
        });

        self.callbacks.insert(id, (event_type, closure))
    }

    pub fn remove_callback<T>(
        &mut self,
        callback: impl Callback + 'static,
    ) -> Option<(String, Closure<(dyn FnMut(Event) + 'static)>)> {
        match self.callbacks.remove(&callback.get_id()) {
            Some((event_type, closure)) => self
                .callbacks_to_remove
                .insert(callback.get_id(), (event_type, closure)),
            None => None,
        }
    }

    fn cleanup_callbacks(
        &self,
        elem: &Element,
        callbacks_to_remove: &HashMap<String, (String, Closure<dyn FnMut(web_sys::Event)>)>,
    ) -> Result<(), AddListenerError> {
        for (_id, (event_type, closure)) in callbacks_to_remove.iter() {
            match elem
                .remove_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
            {
                Ok(_) => todo!(),
                Err(_err) => return Err(AddListenerError::new(event_type.to_owned())),
            }
        }

        Ok(())
    }
}

impl ListenerInjector for CallbackHolder {
    fn inject_listeners(&mut self, node_ref: &NodeRef) -> Result<(), AddListenerError> {
        if let Some(elem) = node_ref.cast::<Element>() {
            match self.cleanup_callbacks(&elem, &self.callbacks_to_remove) {
                Ok(_) => self.callbacks_to_remove.clear(),
                Err(_) => (),
            };

            for (_id, (event_type, closure)) in self.callbacks.iter() {
                match elem
                    .add_event_listener_with_callback(event_type, closure.as_ref().unchecked_ref())
                {
                    Ok(_) => (),
                    Err(_err) => return Err(AddListenerError::new(event_type.to_owned())),
                }
                // closure.forget();
            }
        }
        Ok(())
    }

    /// This function should be called on the destroy method of the Yew component.
    /// It should remove all the listeners.
    fn cleanup_listeners(&mut self, node_ref: &NodeRef) -> Result<(), RemoveListenerError> {
        if let Some(elem) = node_ref.cast::<Element>() {
            match self.cleanup_callbacks(&elem, &self.callbacks_to_remove) {
                Ok(_) => self.callbacks_to_remove.clear(),
                Err(_) => (),
            };

            match self.cleanup_callbacks(&elem, &self.callbacks) {
                Ok(_) => self.callbacks.clear(),
                Err(_) => (),
            };
        }
        Ok(())
    }
}

pub trait Callback {
    type Event;

    fn get_event_type(&self) -> String;
    fn get_callback(&self) -> &yew::Callback<Event>;
}
