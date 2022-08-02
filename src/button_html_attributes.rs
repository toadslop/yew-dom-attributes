use crate::{
    aria_attributes::AriaAttributes,
    attribute_holder::{Attribute, AttributeHolder},
    attribute_injector::AttributeInjector,
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
use strum::Display;

use yew::Properties;

#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "camelCase")]
pub enum ButtonHtmlAttributes {
    AutoFocus,
    Disabled,
    Form(String),
    FormAction(String),
    FormEncType(String),
    FormMethod(String),
    FormNoValidate,
    FormTarget(String),
    Name(String),
    Type(ButtonType),
    Value(ButtonValue),
}

impl std::hash::Hash for ButtonHtmlAttributes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_key().hash(state)
    }
}

impl PartialEq for ButtonHtmlAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.get_key() == other.get_key()
    }
}

impl Eq for ButtonHtmlAttributes {}

impl Attribute for ButtonHtmlAttributes {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match self {
            ButtonHtmlAttributes::AutoFocus => None,
            ButtonHtmlAttributes::Disabled => None,
            ButtonHtmlAttributes::Form(val) => Some(val.to_string()),
            ButtonHtmlAttributes::FormAction(val) => Some(val.to_string()),
            ButtonHtmlAttributes::FormEncType(val) => Some(val.to_string()),
            ButtonHtmlAttributes::FormMethod(val) => Some(val.to_string()),
            ButtonHtmlAttributes::FormNoValidate => None,
            ButtonHtmlAttributes::FormTarget(val) => Some(val.to_string()),
            ButtonHtmlAttributes::Name(val) => Some(val.to_string()),
            ButtonHtmlAttributes::Type(val) => Some(val.to_string()),
            ButtonHtmlAttributes::Value(val) => Some(val.to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum ButtonType {
    Submit,
    Reset,
    Button,
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum ButtonValue {
    String(String),
    StringVec(Vec<String>),
    Number(u64),
}
