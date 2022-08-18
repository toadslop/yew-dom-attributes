use crate::events::events::{
    AnimationEvents, CustomEvent, DragEvents, EventType, FocusEvents, GenericEvents, InputEvents,
    KeyboardEvents, MouseEvents, PointerEvents, ProgressEvents, TouchEvents, TransitionEvents,
    WheelEvents,
};

use gloo_events::EventListener;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::Element;

use yew::{Callback, Component, Context, NodeRef};
pub mod button_props;
pub mod global_props;
pub mod svg_props;

// TODO: create builder classes that allow users to build props, step by step specifying the
// capacity for each prop holder. That way they can prevent allocation for props that they
// will never use.

// Note to self: the details element has a toggle event. A special "add toggle event"
// method needs to be added to it.

mod private {
    use crate::events::events::EventType;
    use std::collections::HashMap;

    use super::ProcessAction;

    pub trait PropsGetterSetter {
        fn get_attributes(&mut self) -> &mut Vec<(ProcessAction, String, Option<String>)>;
    }

    pub trait ListenerGetterSetter {
        fn get_listeners_to_add(&mut self) -> &mut HashMap<String, EventType>;

        fn get_listeners_to_remove(&mut self) -> &mut Vec<String>;
    }
}

pub trait DynamicListenerComponent {
    fn get_listeners(&self) -> &mut HashMap<String, Rc<EventListener>>;
}

/// A trait to be implemented by any element that accepts listeners.
/// Handles adding and removing listeners as well as injecting them to the DOM.
pub trait DomInjector: private::ListenerGetterSetter + private::PropsGetterSetter {
    fn new<T: Component, F, R>(ctx: &Context<T>, func: F) -> Self
    where
        F: Fn(Rc<Self>) -> R + 'static,
        T: yew::Component,
        <T as yew::Component>::Message: std::convert::From<R>;

    fn add_listener(&mut self, key: String, event_type: EventType) {
        self.get_listeners_to_add().insert(key, event_type);
    }

    fn remove_listener(&mut self, key: String) {
        self.get_listeners_to_remove().push(key);
    }

    /// This function returns a callback that takes the props struct itelf. This is used
    /// to pass changes to props struct from the child back up to the parent.
    /// This is necessary to inform the parent that attributes and listeners were either
    /// added or removed from the DOM. If this is not used properly, your component will
    /// not know that it happened and will try again on the next rerender.
    fn get_props_update_callback(&self) -> &Callback<Rc<Self>>;

    /// The active_listeners parameter should be stored in the host Component so the listeners it contained will be
    /// dropped when that Component is destroyed.
    fn inject(
        &mut self,
        node_ref: &NodeRef,
        active_listeners: &mut HashMap<String, Rc<EventListener>>,
    ) {
        if let Some(elem) = node_ref.cast::<Element>() {
            let listeners_to_remove = self.get_listeners_to_remove();
            remove_listeners(active_listeners, listeners_to_remove);

            let listeners_to_add = self.get_listeners_to_add();
            inject_listeners(&elem, active_listeners, listeners_to_add);

            let attributes = self.get_attributes();
            inject_attributes(&elem, attributes)
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
                .unwrap(),
            ProcessAction::Remove => elem.remove_attribute(&key).unwrap(),
        }
    }
}

fn remove_listeners(
    active_listeners: &mut HashMap<String, Rc<EventListener>>,
    listeners_to_remove: &mut Vec<String>,
) {
    while let Some(listener_key) = listeners_to_remove.pop() {
        match active_listeners.remove(&listener_key) {
            Some(listener) => drop(listener),
            None => (),
        }
    }
}

fn build_event_listener<TEvent: JsCast + 'static>(
    elem: &Element,
    cb: Callback<TEvent>,
    event_type: &'static str,
) -> EventListener {
    let cb = cb.clone();
    EventListener::new(&elem, event_type, move |e| {
        let event = e.clone();
        cb.emit(event.dyn_into::<TEvent>().unwrap())
    })
}

fn build_mouse_event(elem: &Element, ev: MouseEvents) -> EventListener {
    let cb = match &ev {
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
    }
    .clone();
    let event_type: &'static str = ev.into();

    build_event_listener(elem, cb, event_type)
}

fn build_generic_event(elem: &Element, ev: GenericEvents) -> EventListener {
    let cb = match &ev {
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
    }
    .clone();
    let event_type = ev.into();

    build_event_listener(elem, cb, event_type)
}

fn build_touch_event(elem: &Element, ev: TouchEvents) -> EventListener {
    let cb = match &ev {
        TouchEvents::TouchCancel(cb) => cb,
        TouchEvents::TouchEnd(cb) => cb,
        TouchEvents::TouchMove(cb) => cb,
        TouchEvents::TouchStart(cb) => cb,
    }
    .clone();
    let event_type = ev.into();

    build_event_listener(elem, cb, event_type)
}

fn build_focus_event(elem: &Element, ev: FocusEvents) -> EventListener {
    let cb = match &ev {
        FocusEvents::Blur(cb) => cb,
        FocusEvents::Focus(cb) => cb,
        FocusEvents::FocusIn(cb) => cb,
        FocusEvents::FocusOut(cb) => cb,
        FocusEvents::Submit(cb) => cb,
    }
    .clone();
    let event_type = ev.into();

    build_event_listener(elem, cb, event_type)
}

fn build_drag_event(elem: &Element, ev: DragEvents) -> EventListener {
    let cb = match &ev {
        DragEvents::Drag(cb) => cb,
        DragEvents::DragEnd(cb) => cb,
        DragEvents::DragEnter(cb) => cb,
        DragEvents::DragExit(cb) => cb,
        DragEvents::DragLeave(cb) => cb,
        DragEvents::DragOver(cb) => cb,
        DragEvents::DragStart(cb) => cb,
        DragEvents::Drop(cb) => cb,
    }
    .clone();
    let event_type = ev.into();

    build_event_listener(elem, cb, event_type)
}

fn build_input_event(elem: &Element, ev: InputEvents) -> EventListener {
    let cb = match &ev {
        InputEvents::Input(cb) => cb,
    }
    .clone();
    let event_type = ev.into();

    build_event_listener(elem, cb, event_type)
}

fn build_keyboard_event(elem: &Element, ev: KeyboardEvents) -> EventListener {
    let cb = match &ev {
        KeyboardEvents::Keydown(cb) => cb,
        KeyboardEvents::KeyPress(cb) => cb,
        KeyboardEvents::KeyUp(cb) => cb,
    }
    .clone();
    let event_type = ev.into();

    build_event_listener(elem, cb, event_type)
}

fn build_progress_event(elem: &Element, ev: ProgressEvents) -> EventListener {
    let cb = match &ev {
        ProgressEvents::LoadStart(cb) => cb,
        ProgressEvents::Progress(cb) => cb,
        ProgressEvents::Loadend(cb) => cb,
    }
    .clone();
    let event_type = ev.into();

    build_event_listener(elem, cb, event_type)
}

fn build_wheel_event(elem: &Element, ev: WheelEvents) -> EventListener {
    let cb = match &ev {
        WheelEvents::Wheel(cb) => cb,
    }
    .clone();
    let event_type = ev.into();

    build_event_listener(elem, cb, event_type)
}

fn build_animation_event(elem: &Element, ev: AnimationEvents) -> EventListener {
    let cb = match &ev {
        AnimationEvents::AnimationCancel(cb) => cb,
        AnimationEvents::AnimationEnd(cb) => cb,
        AnimationEvents::AnimationIteration(cb) => cb,
        AnimationEvents::AnimationStart(cb) => cb,
    }
    .clone();
    let event_type = ev.into();

    build_event_listener(elem, cb, event_type)
}

fn build_pointer_event(elem: &Element, ev: PointerEvents) -> EventListener {
    let cb = match &ev {
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
    }
    .clone();
    let event_type = ev.into();

    build_event_listener(elem, cb, event_type)
}

fn build_transition_event(elem: &Element, ev: TransitionEvents) -> EventListener {
    let cb = match &ev {
        TransitionEvents::TransitionCancel(cb) => cb,
        TransitionEvents::TransitionEnd(cb) => cb,
        TransitionEvents::TransitionRun(cb) => cb,
        TransitionEvents::TransitionStart(cb) => cb,
    }
    .clone();
    let event_type = ev.into();

    build_event_listener(elem, cb, event_type)
}

fn build_custom_event(elem: &Element, ev: CustomEvent) -> EventListener {
    let event_type = ev.get_event_type();
    let cb = ev.get_callback().clone();
    build_event_listener(elem, cb, event_type)
}

fn inject_listeners(
    elem: &Element,
    active_listeners: &mut HashMap<String, Rc<EventListener>>,
    listeners_to_add: &mut HashMap<String, EventType>,
) {
    let mut listener_holder = HashMap::new();
    for (key, listener) in listeners_to_add.drain() {
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
        listener_holder.insert(key, Rc::new(listener));
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
            listeners_to_add: HashMap<String, EventType>,
            listeners_to_remove: Vec<String>,
            /// A callback used to pass changes to button props from the child back up to the parent.
            /// This is necessary to inform the parent that attributes and listeners were either
            /// added or removed from the DOM. If this is not used properly, your component will
            /// not know that it happened and will try again on the next rerender.
            on_props_update: Callback<Rc<$name>>,
        }

        impl $name {
            pub fn add_attribute(&mut self, attribute: Box<dyn $attr_type>) {
                let action = ProcessAction::Add;
                let key = String::from(attribute.get_key());
                let val = attribute.get_val().map(String::from);
                self.attributes.push((action, key, val))
            }

            pub fn remove_attribute(&mut self, attribute: Box<dyn $attr_type>) {
                let action = ProcessAction::Remove;
                let key = String::from(attribute.get_key());
                let val = attribute.get_val().map(String::from);
                self.attributes.push((action, key, val))
            }
        }

        impl super::private::PropsGetterSetter for $name {
            fn get_attributes(&mut self) -> &mut Vec<(ProcessAction, String, Option<String>)> {
                &mut self.attributes
            }
        }

        impl super::private::ListenerGetterSetter for $name {
            fn get_listeners_to_add(&mut self) -> &mut HashMap<String, EventType> {
                &mut self.listeners_to_add
            }

            fn get_listeners_to_remove(&mut self) -> &mut Vec<String> {
                &mut self.listeners_to_remove
            }
        }

        impl DomInjector for $name {
            fn new<T, F, R>(ctx: &Context<T>, func: F) -> Self
            where
                F: Fn(Rc<Self>) -> R + 'static,
                T: yew::Component,
                <T as yew::Component>::Message: std::convert::From<R>,
            {
                let on_props_update = ctx.link().callback(func);

                Self {
                    attributes: Vec::new(),
                    listeners_to_add: HashMap::new(),
                    listeners_to_remove: Vec::new(),
                    on_props_update,
                }
            }

            fn get_props_update_callback(&self) -> &Callback<Rc<Self>> {
                &self.on_props_update
            }
        }
    };
}

pub(crate) use prop_handler;
