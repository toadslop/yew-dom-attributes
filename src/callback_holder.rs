use gloo_events::EventListener;

use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::{NodeRef, Properties};

use crate::listener_injector::{AddListenerError, ListenerInjector};

#[derive(Debug, Properties, PartialEq, Clone, Default)]
pub struct CallbackHolder<T>
where
    T: Callback + PartialEq,
{
    callbacks_to_add: Vec<T>,
}

impl<T> CallbackHolder<T>
where
    T: Callback + PartialEq,
{
    pub fn new() -> Self {
        Self {
            callbacks_to_add: Vec::new(),
        }
    }

    pub fn add_callback(&mut self, callback: T) {
        self.callbacks_to_add.push(callback);
    }
}

impl<T> ListenerInjector for CallbackHolder<T>
where
    T: Callback + PartialEq,
{
    fn inject_listeners(
        &mut self,
        node_ref: &NodeRef,
    ) -> Result<Option<Vec<EventListener>>, AddListenerError> {
        if let Some(elem) = node_ref.cast::<Element>() {
            let mut listeners = Vec::new();
            while let Some(callback) = self.callbacks_to_add.pop() {
                let cb = callback.get_callback().clone();
                let event_type = callback.get_event_type();
                let listener = EventListener::new(&elem, event_type, move |e| {
                    let event = e.clone();
                    let event = event.dyn_into::<<T as Callback>::Event>().unwrap();
                    cb.emit(event)
                });

                listeners.push(listener);
            }
            return Ok(Some(listeners));
        }
        Ok(None)
    }
}

pub trait Callback {
    type Event: JsCast + 'static;

    fn get_event_type(&self) -> String;
    fn get_callback(&self) -> &yew::Callback<Self::Event>;
}
