use yew::Properties;

use crate::{
    aria_attributes::AriaAttributes,
    attribute_holder::AttributeHolder,
    attribute_injector::AttributeInjector,
    button_html_attributes::ButtonHtmlAttributes,
    callback_holder::CallbackHolder,
    events::{
        AnimationEvents, CustomEvent, DragEvents, EventPropsReceiver, FocusEvents, GenericEvents,
        InputEvents, KeyboardEvents, MouseEvents, PointerEvents, ProgressEvents, TouchEvents,
        TransitionEvents, WheelEvents,
    },
    html_attributes::HtmlAttributes,
    listener_injector::ListenerInjector,
    misc_attributes::MiscAttrs,
};

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ButtonProps {
    // attributes
    button_attributes: AttributeHolder<ButtonHtmlAttributes>,
    aria_attributes: AttributeHolder<AriaAttributes>,
    html_attributes: AttributeHolder<HtmlAttributes>,
    custom_attributes: MiscAttrs,
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
            custom_attributes: MiscAttrs::new(),
        }
    }

    pub fn add_btn_attribute(&mut self, attribute: ButtonHtmlAttributes) -> bool {
        self.button_attributes.add_attribute(attribute)
    }

    pub fn remove_btn_attribute(&mut self, attribute: ButtonHtmlAttributes) -> bool {
        self.button_attributes.remove_attribute(attribute)
    }

    pub fn add_aria_attribute(&mut self, attribute: AriaAttributes) -> bool {
        self.aria_attributes.add_attribute(attribute)
    }

    pub fn remove_aria_attribute(&mut self, attribute: AriaAttributes) -> bool {
        self.aria_attributes.remove_attribute(attribute)
    }

    pub fn add_html_attribute(&mut self, attribute: HtmlAttributes) -> bool {
        self.html_attributes.add_attribute(attribute)
    }

    pub fn remove_html_attribute(&mut self, attribute: HtmlAttributes) -> bool {
        self.html_attributes.remove_attribute(attribute)
    }

    pub fn add_html_custom_attribute(&mut self, key: String, value: String) -> bool {
        self.custom_attributes.add_attribute(key, value);
        true
    }

    pub fn remove_html_custome_attribute(&mut self, key: String) -> bool {
        self.custom_attributes.remove_attribute(key);
        true
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
    ) -> Result<Option<Vec<gloo_events::EventListener>>, crate::listener_injector::AddListenerError>
    {
        self.mouse_event_listeners.inject_listeners(node_ref)
    }
}

impl EventPropsReceiver for ButtonProps {
    fn add_generic_listener(&mut self, callback: GenericEvents) {
        self.generic_listeners.add_callback(callback);
    }

    fn add_mouse_event_listener(self: &mut ButtonProps, callback: MouseEvents) {
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
