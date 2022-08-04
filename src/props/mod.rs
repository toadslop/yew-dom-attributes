use std::collections::HashMap;

use gloo_events::EventListener;
use web_sys::Element;
use yew::NodeRef;

use crate::events::events::EventType;

pub mod button_props;
pub mod html_element_props;
pub mod svg_props;
// TODO: create builder classes that allow users to build props, step by step specifying the
// capacity for each prop holder. That way they can prevent allocation for props that they
// will never use.

mod private {
    use gloo_events::EventListener;
    use std::collections::HashMap;
    use wasm_bindgen::JsCast;
    use web_sys::Element;
    use yew::Callback;

    use crate::events::events::{EventType, GenericEvents, MouseEvents, TouchEvents};

    pub trait PropsGetterSetter {
        fn get_props_to_add(&mut self) -> &mut HashMap<String, Option<String>>;

        fn get_props_to_remove(&mut self) -> &mut Vec<String>;
    }

    pub trait ListenerGetterSetter {
        fn get_listeners_to_add(&mut self) -> &mut HashMap<String, EventType>;

        fn get_listeners_to_remove(&mut self) -> &mut Vec<String>;
    }

    pub fn build_event_listener<TEvent: JsCast + 'static>(
        elem: &Element,
        cb: Callback<TEvent>,
        event_type: String,
    ) -> EventListener {
        EventListener::new(&elem, event_type, move |e| {
            let event = e.clone();
            cb.emit(event.dyn_into::<TEvent>().unwrap())
        })
    }

    pub fn build_mouse_event(elem: &Element, ev: MouseEvents) -> EventListener {
        let event_type = ev.to_string();
        let cb = match ev {
            MouseEvents::AuxClick(cb) => cb,
            MouseEvents::Click(cb) => cb,
            MouseEvents::ContextMenu(cb) => cb,
            MouseEvents::DblClick(cb) => cb,
            MouseEvents::MouseDown(cb) => cb,
            MouseEvents::MousEenter(cb) => cb,
            MouseEvents::MouseLeave(cb) => cb,
            MouseEvents::MouseMove(cb) => cb,
            MouseEvents::MouseOut(cb) => cb,
            MouseEvents::MouseOver(cb) => cb,
            MouseEvents::MouseUp(cb) => cb,
        };
        build_event_listener(elem, cb, event_type)
    }

    pub fn build_generic_event(elem: &Element, ev: GenericEvents) -> EventListener {
        let event_type = ev.to_string();
        let cb = match ev {
            GenericEvents::Abort(cb) => cb,
            GenericEvents::Cancel(cb) => cb,
            GenericEvents::CanPlay(cb) => cb,
            GenericEvents::CanPlayThrough(cb) => cb,
            GenericEvents::Change(cb) => cb,
            GenericEvents::Close(cb) => cb,
            GenericEvents::CueChange(cb) => cb,
            GenericEvents::DurationChange(cb) => cb,
            GenericEvents::Emptied(cb) => cb,
            GenericEvents::Ended(cb) => cb,
            GenericEvents::Error(cb) => cb,
            GenericEvents::FormData(cb) => cb,
            GenericEvents::Invalid(cb) => cb,
            GenericEvents::Load(cb) => cb,
            GenericEvents::LoadedData(cb) => cb,
            GenericEvents::LoadedMetadata(cb) => cb,
            GenericEvents::Pause(cb) => cb,
            GenericEvents::Play(cb) => cb,
            GenericEvents::Playing(cb) => cb,
            GenericEvents::RateChange(cb) => cb,
            GenericEvents::Reset(cb) => cb,
            GenericEvents::Resize(cb) => cb,
            GenericEvents::Scroll(cb) => cb,
            GenericEvents::SecurityPolicyViolation(cb) => cb,
            GenericEvents::Seeked(cb) => cb,
            GenericEvents::Seeking(cb) => cb,
            GenericEvents::Select(cb) => cb,
            GenericEvents::SlotChange(cb) => cb,
            GenericEvents::Stalled(cb) => cb,
            GenericEvents::Suspend(cb) => cb,
            GenericEvents::TimeUpdate(cb) => cb,
            GenericEvents::Toggle(cb) => cb,
            GenericEvents::VolumeChange(cb) => cb,
            GenericEvents::Waiting(cb) => cb,
            GenericEvents::Copy(cb) => cb,
            GenericEvents::Cut(cb) => cb,
            GenericEvents::Paste(cb) => cb,
            GenericEvents::SelectionChange(cb) => cb,
            GenericEvents::SelectStart(cb) => cb,
            GenericEvents::Show(cb) => cb,
            GenericEvents::PointerLockChange(cb) => cb,
            GenericEvents::PointerLockError(cb) => cb,
        };
        build_event_listener(elem, cb, event_type)
    }

    pub fn build_touch_event(elem: &Element, ev: TouchEvents) -> EventListener {
        let event_type = ev.to_string();
        let cb = match ev {
            TouchEvents::TouchCancel(cb) => cb,
            TouchEvents::TouchEnd(cb) => cb,
            TouchEvents::TouchMove(cb) => cb,
            TouchEvents::TouchStart(cb) => cb,
        };
        build_event_listener(elem, cb, event_type)
    }
}

/// A trait to be implemented by any element that accepts listeners.
/// Handles adding and removing listeners as well as injecting them to the DOM.
pub trait ListenerHandler: private::ListenerGetterSetter {
    fn add_listener(&mut self, key: String, event_type: EventType) {
        self.get_listeners_to_add().insert(key, event_type);
    }

    // consider removing this; devs can simply remove the item from the EventListern hashmap
    // without going through this structure
    fn remove_listener(&mut self, key: String) {
        self.get_listeners_to_remove().push(key);
    }

    fn inject(&mut self, node_ref: NodeRef) -> Option<HashMap<String, EventListener>> {
        if let Some(elem) = node_ref.cast::<Element>() {
            let mut listeners = HashMap::new();
            for (key, listener) in self.get_listeners_to_add().drain() {
                let listener = match listener {
                    EventType::MouseEvent(ev) => private::build_mouse_event(&elem, ev),
                    EventType::Event(ev) => private::build_generic_event(&elem, ev),
                    EventType::TouchEvent(ev) => private::build_touch_event(&elem, ev),
                };

                listeners.insert(key, listener);
            }
            return Some(listeners);
        }
        None
    }
}
