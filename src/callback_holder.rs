use gloo_events::EventListener;

use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::{NodeRef, TargetCast};

use crate::listener_injector::{AddListenerError, ListenerInjector};

pub struct CallbackHolder<T: JsCast + 'static> {
    callbacks_to_add: Vec<Box<dyn Callback<Event = T>>>,
    listeners: Vec<EventListener>,
}

impl<T: JsCast + std::convert::AsRef<yew::Event>> CallbackHolder<T> {
    pub fn new() -> Self {
        Self {
            callbacks_to_add: Vec::new(),
            listeners: Vec::new(),
        }
    }

    pub fn add_callback<C: Callback>(
        &mut self,
        callback: Box<(dyn Callback<Event = T> + 'static)>,
    ) {
        self.callbacks_to_add.push(callback);
    }
}

impl<T: JsCast + std::convert::AsRef<yew::Event>> ListenerInjector for CallbackHolder<T> {
    fn inject_listeners(&mut self, node_ref: &NodeRef) -> Result<(), AddListenerError> {
        if let Some(elem) = node_ref.cast::<Element>() {
            while let Some(callback) = self.callbacks_to_add.pop() {
                let listener =
                    EventListener::new(&elem, callback.as_ref().get_event_type(), move |e| {
                        let event = e.clone();
                        let event = event.dyn_into::<T>().unwrap();
                        callback.as_ref().get_callback().emit(event)
                    });

                self.listeners.push(listener);
            }
        }
        Ok(())
    }
}

pub trait Callback {
    type Event: TargetCast;

    fn get_event_type(&self) -> String;
    fn get_callback(&self) -> &yew::Callback<Self::Event>;
}
