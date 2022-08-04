use yew::Properties;

use crate::{
    attribute_holder::AttributeHolder,
    attribute_injector::AttributeInjector,
    attributes::{
        aria_attributes::{AriaAttributeReceiver, AriaAttributes},
        html_attributes::{HtmlAttributeReceiver, HtmlAttributes},
    },
    callback_holder::CallbackHolder,
    events::events::{
        AnimationEvents, CustomEvent, DragEvents, EventPropsReceiver, FocusEvents, GenericEvents,
        InputEvents, KeyboardEvents, MouseEvents, PointerEvents, ProgressEvents, TouchEvents,
        TransitionEvents, WheelEvents,
    },
    listener_injector::ListenerInjector,
    misc_attributes::{CustomAttributeReceiver, CustomAttrs},
};

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct HtmlElementProps {
    // attributes
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

impl HtmlElementProps {}

impl HtmlElementProps {
    pub fn new() -> Self {
        Self {
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

    pub fn add_html_attribute(&mut self, attribute: HtmlAttributes) -> bool {
        self.html_attributes.add_attribute(attribute)
    }

    pub fn remove_html_attribute(&mut self, attribute: HtmlAttributes) -> bool {
        self.html_attributes.remove_attribute(attribute)
    }
}

impl AriaAttributeReceiver for HtmlElementProps {
    fn add_aria_attribute(&mut self, attribute: AriaAttributes) -> bool {
        self.aria_attributes.add_attribute(attribute)
    }

    fn remove_aria_attribute(&mut self, attribute: AriaAttributes) -> bool {
        self.aria_attributes.remove_attribute(attribute)
    }
}

impl HtmlAttributeReceiver for HtmlElementProps {
    fn add_html_attribute(&mut self, attribute: HtmlAttributes) -> bool {
        self.html_attributes.add_attribute(attribute)
    }

    fn remove_html_attribute(&mut self, attribute: HtmlAttributes) -> bool {
        self.html_attributes.remove_attribute(attribute)
    }
}

impl CustomAttributeReceiver for HtmlElementProps {
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

impl EventPropsReceiver for HtmlElementProps {
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

impl AttributeInjector for HtmlElementProps {
    fn inject(
        &mut self,
        node_ref: &yew::NodeRef,
    ) -> Result<(), crate::attribute_injector::SetAttributeError> {
        self.aria_attributes.inject(node_ref)?;
        self.html_attributes.inject(node_ref)?;
        self.custom_attributes.inject(node_ref)?;
        Ok(())
    }
}

impl ListenerInjector for HtmlElementProps {
    fn inject_listeners(
        &mut self,
        node_ref: &yew::NodeRef,
    ) -> Result<Vec<gloo_events::EventListener>, crate::listener_injector::AddListenerError> {
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
