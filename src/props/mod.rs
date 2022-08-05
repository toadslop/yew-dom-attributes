use std::{collections::HashMap, rc::Rc};

use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::{Callback, NodeRef};

use crate::events::events::{
    AnimationEvents, CustomEvent, DragEvents, EventType, FocusEvents, GenericEvents, InputEvents,
    KeyboardEvents, MouseEvents, PointerEvents, ProgressEvents, TouchEvents, TransitionEvents,
    WheelEvents,
};

pub mod aria_props;
pub mod button_props;
pub mod custom_attributes;
pub mod html_element_props;
pub mod svg_props;
// TODO: create builder classes that allow users to build props, step by step specifying the
// capacity for each prop holder. That way they can prevent allocation for props that they
// will never use.

mod private {
    use crate::events::events::EventType;
    use std::collections::HashMap;

    pub trait PropsGetterSetter {
        fn get_props_to_add(&mut self) -> &mut HashMap<String, Option<String>>;

        fn get_props_to_remove(&mut self) -> &mut Vec<String>;
    }

    pub trait ListenerGetterSetter {
        fn get_listeners_to_add(&mut self) -> &mut HashMap<String, EventType>;

        fn get_listeners_to_remove(&mut self) -> &mut Vec<String>;
    }
}

pub trait DynamicListenerComponent {
    fn get_listeners(&self) -> &mut HashMap<String, EventListener>;
}

/// A trait to be implemented by any element that accepts listeners.
/// Handles adding and removing listeners as well as injecting them to the DOM.
pub trait DomInjector: private::ListenerGetterSetter + private::PropsGetterSetter {
    fn add_listener(&mut self, key: String, event_type: EventType) {
        gloo_console::log!("ADDING A LISTNER NOE");
        self.get_listeners_to_add().insert(key, event_type);
    }

    // update how we handle this: this structure will own the event listener
    // this structure will destroy the listener
    // possible problem: we want the listener to go when the element its attached
    // to goes out of scope. but this will be owned by the parent.
    fn remove_listener(&mut self, key: String) {
        gloo_console::log!("ADDING A LISTNER TO REMOVE");
        self.get_listeners_to_remove().push(key);
    }

    fn get_listener_to_add_count(&mut self) -> usize {
        self.get_listeners_to_add().len()
    }

    fn get_props_update_callback(&self) -> &Callback<Rc<Self>>;

    /// The active_listeners parameter should be stored in the host Component so the listeners it contained will be
    /// dropped when that Component is destroyed.
    fn inject(
        &mut self,
        node_ref: &NodeRef,
        active_listeners: &mut HashMap<String, EventListener>,
    ) {
        if let Some(elem) = node_ref.cast::<Element>() {
            let listeners_to_remove = self.get_listeners_to_remove();
            gloo_console::log!("listeners to remove length before");
            gloo_console::log!(listeners_to_remove.len());
            remove_listeners(active_listeners, listeners_to_remove);
            gloo_console::log!("listeners to remove length after");
            gloo_console::log!(listeners_to_remove.len());

            let listeners_to_add = self.get_listeners_to_add();
            gloo_console::log!("listenrs to add length before");
            gloo_console::log!(listeners_to_add.len());
            listeners_to_add
                .iter()
                .for_each(|(key, _)| gloo_console::log!(key));
            inject_listeners(&elem, active_listeners, listeners_to_add);
            gloo_console::log!("listenrs to add length after");
            gloo_console::log!(listeners_to_add.len());
            gloo_console::log!("");

            let attributes_to_remove = self.get_props_to_remove();
            remove_attributes(&elem, attributes_to_remove);
            gloo_console::log!("attributes to remove length");
            gloo_console::log!(attributes_to_remove.len());
            let attributes_to_add = self.get_props_to_add();
            gloo_console::log!("attributes to add length");
            gloo_console::log!(attributes_to_add.len());
            inject_attributes(&elem, attributes_to_add);
        }
    }
}

fn inject_attributes(elem: &Element, attributes_to_add: &mut HashMap<String, Option<String>>) {
    for (key, value) in attributes_to_add.drain() {
        elem.set_attribute(&key, &value.unwrap_or_default())
            .unwrap();
    }
}

fn remove_attributes(elem: &Element, attributes_to_remove: &mut Vec<String>) {
    while let Some(key) = attributes_to_remove.pop() {
        elem.remove_attribute(&key).unwrap();
    }
}

fn remove_listeners(
    active_listeners: &mut HashMap<String, EventListener>,
    listeners_to_remove: &mut Vec<String>,
) {
    while let Some(listener_key) = listeners_to_remove.pop() {
        gloo_console::log!("Inside remove listener");
        gloo_console::log!(listener_key.clone());
        match active_listeners.remove(&listener_key) {
            Some(listener) => drop(listener),
            None => (),
        }
    }
}

fn build_event_listener<TEvent: JsCast + 'static>(
    elem: &Element,
    cb: Callback<TEvent>,
    event_type: String,
) -> EventListener {
    EventListener::new(&elem, event_type, move |e| {
        let event = e.clone();
        cb.emit(event.dyn_into::<TEvent>().unwrap())
    })
}

fn build_mouse_event(elem: &Element, ev: MouseEvents) -> EventListener {
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

fn build_generic_event(elem: &Element, ev: GenericEvents) -> EventListener {
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

fn build_touch_event(elem: &Element, ev: TouchEvents) -> EventListener {
    let event_type = ev.to_string();
    let cb = match ev {
        TouchEvents::TouchCancel(cb) => cb,
        TouchEvents::TouchEnd(cb) => cb,
        TouchEvents::TouchMove(cb) => cb,
        TouchEvents::TouchStart(cb) => cb,
    };
    build_event_listener(elem, cb, event_type)
}

fn build_focus_event(elem: &Element, ev: FocusEvents) -> EventListener {
    let event_type = ev.to_string();
    let cb = match ev {
        FocusEvents::Blur(cb) => cb,
        FocusEvents::Focus(cb) => cb,
        FocusEvents::FocusIn(cb) => cb,
        FocusEvents::FocusOut(cb) => cb,
        FocusEvents::Submit(cb) => cb,
    };
    build_event_listener(elem, cb, event_type)
}

fn build_drag_event(elem: &Element, ev: DragEvents) -> EventListener {
    let event_type = ev.to_string();
    let cb = match ev {
        DragEvents::Drag(cb) => cb,
        DragEvents::DragEnd(cb) => cb,
        DragEvents::DragEnter(cb) => cb,
        DragEvents::DragExit(cb) => cb,
        DragEvents::DragLeave(cb) => cb,
        DragEvents::DragOver(cb) => cb,
        DragEvents::DragStart(cb) => cb,
        DragEvents::Drop(cb) => cb,
    };
    build_event_listener(elem, cb, event_type)
}

fn build_input_event(elem: &Element, ev: InputEvents) -> EventListener {
    let event_type = ev.to_string();
    let cb = match ev {
        InputEvents::Input(cb) => cb,
    };
    build_event_listener(elem, cb, event_type)
}

fn build_keyboard_event(elem: &Element, ev: KeyboardEvents) -> EventListener {
    let event_type = ev.to_string();
    let cb = match ev {
        KeyboardEvents::Keydown(cb) => cb,
        KeyboardEvents::KeyPress(cb) => cb,
        KeyboardEvents::KeyUp(cb) => cb,
    };
    build_event_listener(elem, cb, event_type)
}

fn build_progress_event(elem: &Element, ev: ProgressEvents) -> EventListener {
    let event_type = ev.to_string();
    let cb = match ev {
        ProgressEvents::LoadStart(cb) => cb,
        ProgressEvents::Progress(cb) => cb,
        ProgressEvents::Loadend(cb) => cb,
    };
    build_event_listener(elem, cb, event_type)
}

fn build_wheel_event(elem: &Element, ev: WheelEvents) -> EventListener {
    let event_type = ev.to_string();
    let cb = match ev {
        WheelEvents::Wheel(cb) => cb,
    };
    build_event_listener(elem, cb, event_type)
}

fn build_animation_event(elem: &Element, ev: AnimationEvents) -> EventListener {
    let event_type = ev.to_string();
    let cb = match ev {
        AnimationEvents::AnimationCancel(cb) => cb,
        AnimationEvents::AnimationEnd(cb) => cb,
        AnimationEvents::AnimationIteration(cb) => cb,
        AnimationEvents::AnimationStart(cb) => cb,
    };
    build_event_listener(elem, cb, event_type)
}

fn build_pointer_event(elem: &Element, ev: PointerEvents) -> EventListener {
    let event_type = ev.to_string();
    let cb = match ev {
        PointerEvents::GotPointerCapture(cb) => cb,
        PointerEvents::LostPointerCapture(cb) => cb,
        PointerEvents::PointerCancel(cb) => cb,
        PointerEvents::PointerDown(cb) => cb,
        PointerEvents::PointerEnter(cb) => cb,
        PointerEvents::PointerLeave(cb) => cb,
        PointerEvents::PointerMove(cb) => cb,
        PointerEvents::PointerOut(cb) => cb,
        PointerEvents::PointerOver(cb) => cb,
        PointerEvents::PointerUp(cb) => cb,
    };
    build_event_listener(elem, cb, event_type)
}

fn build_transition_event(elem: &Element, ev: TransitionEvents) -> EventListener {
    let event_type = ev.to_string();
    let cb = match ev {
        TransitionEvents::TransitionCancel(cb) => cb,
        TransitionEvents::TransitionEnd(cb) => cb,
        TransitionEvents::TransitionRun(cb) => cb,
        TransitionEvents::TransitionStart(cb) => cb,
    };
    build_event_listener(elem, cb, event_type)
}

fn build_custom_event(elem: &Element, ev: CustomEvent) -> EventListener {
    let event_type = ev.get_event_type();
    let cb = ev.get_callback();
    build_event_listener(elem, cb, event_type)
}

fn inject_listeners(
    elem: &Element,
    active_listeners: &mut HashMap<String, EventListener>,
    listeners_to_add: &mut HashMap<String, EventType>,
) {
    let mut listener_holder = HashMap::new();
    for (key, listener) in listeners_to_add.drain() {
        gloo_console::log!(key.clone());
        let listener = match listener {
            EventType::MouseEvent(ev) => build_mouse_event(&elem, ev),
            EventType::Event(ev) => build_generic_event(&elem, ev),
            EventType::TouchEvent(ev) => build_touch_event(&elem, ev),
            EventType::FocusEvent(ev) => build_focus_event(&elem, ev),
            EventType::DragEvent(ev) => build_drag_event(&elem, ev),
            EventType::InputEvent(ev) => build_input_event(&elem, ev),
            EventType::KeyboardEvent(ev) => build_keyboard_event(&elem, ev),
            EventType::ProgressEvent(ev) => build_progress_event(&elem, ev),
            EventType::WheelEvent(ev) => build_wheel_event(&elem, ev),
            EventType::AnimationEvent(ev) => build_animation_event(&elem, ev),
            EventType::PointerEvent(ev) => build_pointer_event(&elem, ev),
            EventType::TransitionEvent(ev) => build_transition_event(&elem, ev),
            EventType::CustomEvent(ev) => build_custom_event(&elem, ev),
        };
        listener_holder.insert(key, listener);
    }
    active_listeners.extend(listener_holder);
}
