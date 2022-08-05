use gloo_events::EventListener;
use std::{collections::HashMap, rc::Rc};
use yew::{Callback, Properties};

use crate::{
    attribute_holder::AttributeHolder,
    attribute_injector::AttributeInjector,
    attributes::{
        aria_attributes::{AriaAttributeReceiver, AriaAttributes},
        button_html_attributes::ButtonHtmlAttributes,
        html_attributes::{HtmlAttributeReceiver, HtmlAttributes},
    },
    callback_holder::CallbackHolder,
    events::events::{
        AnimationEvents, CustomEvent, DragEvents, EventPropsReceiver, EventType, FocusEvents,
        GenericEvents, InputEvents, KeyboardEvents, MouseEvents, PointerEvents, ProgressEvents,
        TouchEvents, TransitionEvents, WheelEvents,
    },
    listener_injector::{AddListenerError, ListenerInjector},
};

use super::{
    aria_props::AriaPropsHandler,
    custom_attributes::{CustomAttributeReceiver, CustomAttrs, CustomPropsHandler},
    html_element_props::HtmlElementPropsHandler,
    DomInjector,
};

/// Second iteration of button props. Streamline down to simply 4 data structures.
/// To maintain type safety for attributes, input is handled by functions which take a specific type.
/// Since any attribute can have any type of listener, any event can be provided.
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ButtonProps2 {
    attributes_to_add: HashMap<String, Option<String>>,
    attributes_to_remove: Vec<String>,
    listeners_to_add: HashMap<String, EventType>,
    listeners_to_remove: Vec<String>,
    on_props_update: Callback<Rc<ButtonProps2>>,
}

impl ButtonProps2 {
    pub fn new(on_props_update: Callback<Rc<Self>>) -> Self {
        Self {
            attributes_to_add: HashMap::new(),
            attributes_to_remove: Vec::new(),
            listeners_to_add: HashMap::new(),
            listeners_to_remove: Vec::new(),
            on_props_update,
        }
    }
}

impl super::private::PropsGetterSetter for ButtonProps2 {
    fn get_props_to_add(&mut self) -> &mut HashMap<String, Option<String>> {
        &mut self.attributes_to_add
    }

    fn get_props_to_remove(&mut self) -> &mut Vec<String> {
        &mut self.attributes_to_remove
    }
}

impl super::private::ListenerGetterSetter for ButtonProps2 {
    fn get_listeners_to_add(&mut self) -> &mut HashMap<String, EventType> {
        &mut self.listeners_to_add
    }

    fn get_listeners_to_remove(&mut self) -> &mut Vec<String> {
        &mut self.listeners_to_remove
    }
}

/// A trait to be implemented by any type that accepts button props.
pub trait ButtonPropsHandler: super::private::PropsGetterSetter {
    fn add_button_prop(&mut self, prop: ButtonHtmlAttributes) {
        let key = prop.to_string();
        let val = match &prop {
            ButtonHtmlAttributes::AutoFocus => None,
            ButtonHtmlAttributes::Disabled => None,
            ButtonHtmlAttributes::Form(val) => Some(val.to_owned()),
            ButtonHtmlAttributes::FormAction(val) => Some(val.to_owned()),
            ButtonHtmlAttributes::FormEncType(val) => Some(val.to_owned()),
            ButtonHtmlAttributes::FormMethod(val) => Some(val.to_owned()),
            ButtonHtmlAttributes::FormNoValidate => None,
            ButtonHtmlAttributes::FormTarget(val) => Some(val.to_owned()),
            ButtonHtmlAttributes::Name(val) => Some(val.to_owned()),
            ButtonHtmlAttributes::Type(val) => Some(val.to_string()),
            ButtonHtmlAttributes::Value(val) => Some(val.to_string()),
        };
        self.get_props_to_add().insert(key, val);
    }

    fn remove_button_prop(&mut self, prop: ButtonHtmlAttributes) {
        self.get_props_to_remove().push(prop.to_string())
    }
}

impl ButtonPropsHandler for ButtonProps2 {}
impl AriaPropsHandler for ButtonProps2 {}
impl HtmlElementPropsHandler for ButtonProps2 {}
impl CustomPropsHandler for ButtonProps2 {}
impl DomInjector for ButtonProps2 {
    fn get_props_update_callback(&self) -> &Callback<Rc<Self>> {
        &self.on_props_update
    }
}

//
//
// The original implementation

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ButtonProps {
    // attributes
    button_attributes: AttributeHolder<ButtonHtmlAttributes>,
    aria_attributes: AttributeHolder<AriaAttributes>,
    html_attributes: AttributeHolder<HtmlAttributes>,
    custom_attributes: CustomAttrs,
    // event listeners
    generic_listeners: CallbackHolder<GenericEvents>,
    mouse_event_listeners: CallbackHolder<MouseEvents>,
    transition_event_listeners: CallbackHolder<TransitionEvents>,
    touch_event_listeners: CallbackHolder<TouchEvents>,
    pointer_event_listeners: CallbackHolder<PointerEvents>,
    animation_event_listeners: CallbackHolder<AnimationEvents>,
    wheel_event_listeners: CallbackHolder<WheelEvents>,
    progress_event_listeners: CallbackHolder<ProgressEvents>,
    keyboard_event_listeners: CallbackHolder<KeyboardEvents>,
    input_event_listeners: CallbackHolder<InputEvents>,
    drag_event_listeners: CallbackHolder<DragEvents>,
    focuse_event_listeners: CallbackHolder<FocusEvents>,
    custom_listeners: CallbackHolder<CustomEvent>,
}

impl ButtonProps {
    pub fn new() -> Self {
        Self {
            button_attributes: AttributeHolder::new(),
            aria_attributes: AttributeHolder::new(),
            html_attributes: AttributeHolder::new(),
            generic_listeners: CallbackHolder::new(),
            mouse_event_listeners: CallbackHolder::new(),
            custom_listeners: CallbackHolder::new(),
            transition_event_listeners: CallbackHolder::new(),
            touch_event_listeners: CallbackHolder::new(),
            animation_event_listeners: CallbackHolder::new(),
            pointer_event_listeners: CallbackHolder::new(),
            wheel_event_listeners: CallbackHolder::new(),
            progress_event_listeners: CallbackHolder::new(),
            keyboard_event_listeners: CallbackHolder::new(),
            drag_event_listeners: CallbackHolder::new(),
            input_event_listeners: CallbackHolder::new(),
            focuse_event_listeners: CallbackHolder::new(),
            custom_attributes: CustomAttrs::new(),
        }
    }

    pub fn add_btn_attribute(&mut self, attribute: ButtonHtmlAttributes) -> bool {
        self.button_attributes.add_attribute(attribute)
    }

    pub fn remove_btn_attribute(&mut self, attribute: ButtonHtmlAttributes) -> bool {
        self.button_attributes.remove_attribute(attribute)
    }
}

impl AttributeInjector for ButtonProps {
    fn inject(
        &mut self,
        node_ref: &yew::NodeRef,
    ) -> Result<(), crate::attribute_injector::SetAttributeError> {
        self.button_attributes.inject(node_ref)?;
        self.aria_attributes.inject(node_ref)?;
        self.html_attributes.inject(node_ref)?;
        self.custom_attributes.inject(node_ref)?;
        Ok(())
    }
}

impl ListenerInjector for ButtonProps {
    fn inject_listeners(
        &mut self,
        node_ref: &yew::NodeRef,
    ) -> Result<Vec<EventListener>, AddListenerError> {
        let mut listeners = Vec::new();

        let mouse_listeners = &mut self.mouse_event_listeners.inject_listeners(node_ref)?;
        listeners.append(mouse_listeners);

        let generic_listeners = &mut self.generic_listeners.inject_listeners(node_ref)?;
        listeners.append(generic_listeners);

        let custom_listeners = &mut self.custom_listeners.inject_listeners(node_ref)?;
        listeners.append(custom_listeners);

        let transition_event_listeners =
            &mut self.transition_event_listeners.inject_listeners(node_ref)?;
        listeners.append(transition_event_listeners);

        let touch_event_listeners = &mut self.touch_event_listeners.inject_listeners(node_ref)?;
        listeners.append(touch_event_listeners);

        let animation_event_listeners =
            &mut self.animation_event_listeners.inject_listeners(node_ref)?;
        listeners.append(animation_event_listeners);

        let pointer_event_listeners =
            &mut self.pointer_event_listeners.inject_listeners(node_ref)?;
        listeners.append(pointer_event_listeners);

        let wheel_event_listeners = &mut self.wheel_event_listeners.inject_listeners(node_ref)?;
        listeners.append(wheel_event_listeners);

        let progress_event_listeners =
            &mut self.progress_event_listeners.inject_listeners(node_ref)?;
        listeners.append(progress_event_listeners);

        let keyboard_event_listeners =
            &mut self.keyboard_event_listeners.inject_listeners(node_ref)?;
        listeners.append(keyboard_event_listeners);

        let drag_event_listeners = &mut self.drag_event_listeners.inject_listeners(node_ref)?;
        listeners.append(drag_event_listeners);

        let input_event_listeners = &mut self.input_event_listeners.inject_listeners(node_ref)?;
        listeners.append(input_event_listeners);

        let focuse_event_listeners = &mut self.focuse_event_listeners.inject_listeners(node_ref)?;
        listeners.append(focuse_event_listeners);

        Ok(listeners)
    }
}

impl EventPropsReceiver for ButtonProps {
    fn add_generic_listener(&mut self, callback: GenericEvents) {
        self.generic_listeners.add_callback(callback);
    }

    fn add_mouse_event_listener(&mut self, callback: MouseEvents) {
        self.mouse_event_listeners.add_callback(callback)
    }

    fn add_custom_listener(&mut self, callback: CustomEvent) {
        self.custom_listeners.add_callback(callback);
    }

    fn add_transition_event_listener(&mut self, callback: TransitionEvents) {
        self.transition_event_listeners.add_callback(callback);
    }

    fn add_touch_event_listener(&mut self, callback: TouchEvents) {
        self.touch_event_listeners.add_callback(callback);
    }

    fn add_animation_event_listener(&mut self, callback: AnimationEvents) {
        self.animation_event_listeners.add_callback(callback);
    }

    fn add_pointer_event_listener(&mut self, callback: PointerEvents) {
        self.pointer_event_listeners.add_callback(callback);
    }

    fn add_wheel_event_listener(&mut self, callback: WheelEvents) {
        self.wheel_event_listeners.add_callback(callback);
    }

    fn add_progress_event_listener(&mut self, callback: ProgressEvents) {
        self.progress_event_listeners.add_callback(callback);
    }

    fn add_keyboard_event_listener(&mut self, callback: KeyboardEvents) {
        self.keyboard_event_listeners.add_callback(callback);
    }

    fn add_drag_event_listener(&mut self, callback: DragEvents) {
        self.drag_event_listeners.add_callback(callback);
    }

    fn add_input_event_listener(&mut self, callback: InputEvents) {
        self.input_event_listeners.add_callback(callback);
    }

    fn add_focus_event_listeners(&mut self, callback: FocusEvents) {
        self.focuse_event_listeners.add_callback(callback);
    }
}

impl AriaAttributeReceiver for ButtonProps {
    fn add_aria_attribute(&mut self, attribute: AriaAttributes) -> bool {
        self.aria_attributes.add_attribute(attribute)
    }

    fn remove_aria_attribute(&mut self, attribute: AriaAttributes) -> bool {
        self.aria_attributes.remove_attribute(attribute)
    }
}

impl HtmlAttributeReceiver for ButtonProps {
    fn add_html_attribute(&mut self, attribute: HtmlAttributes) -> bool {
        self.html_attributes.add_attribute(attribute)
    }

    fn remove_html_attribute(&mut self, attribute: HtmlAttributes) -> bool {
        self.html_attributes.remove_attribute(attribute)
    }
}

impl CustomAttributeReceiver for ButtonProps {
    fn add_attribute(&mut self, key: String, value: String) -> bool {
        self.custom_attributes.add_attribute(key, value)
    }

    fn add_boolean_attribute(&mut self, key: String) -> bool {
        self.custom_attributes.add_boolean_attribute(key)
    }

    fn remove_attribute(&mut self, key: String) -> bool {
        self.custom_attributes.remove_attribute(key)
    }
}
