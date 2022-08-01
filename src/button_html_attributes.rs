use crate::{
    aria_attributes::AriaAttributes,
    attribute_holder::{Attribute, AttributeHolder},
    attribute_injector::AttributeInjector,
};
use strum::Display;
use yew::Properties;

#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "camelCase")]
pub enum ButtonOwnHtmlAttributes {
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

impl std::hash::Hash for ButtonOwnHtmlAttributes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_key().hash(state)
    }
}

impl PartialEq for ButtonOwnHtmlAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.get_key() == other.get_key()
    }
}

impl Eq for ButtonOwnHtmlAttributes {}

impl Attribute for ButtonOwnHtmlAttributes {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match self {
            ButtonOwnHtmlAttributes::AutoFocus => None,
            ButtonOwnHtmlAttributes::Disabled => None,
            ButtonOwnHtmlAttributes::Form(val) => Some(val.to_string()),
            ButtonOwnHtmlAttributes::FormAction(val) => Some(val.to_string()),
            ButtonOwnHtmlAttributes::FormEncType(val) => Some(val.to_string()),
            ButtonOwnHtmlAttributes::FormMethod(val) => Some(val.to_string()),
            ButtonOwnHtmlAttributes::FormNoValidate => None,
            ButtonOwnHtmlAttributes::FormTarget(val) => Some(val.to_string()),
            ButtonOwnHtmlAttributes::Name(val) => Some(val.to_string()),
            ButtonOwnHtmlAttributes::Type(val) => Some(val.to_string()),
            ButtonOwnHtmlAttributes::Value(val) => Some(val.to_string()),
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

#[derive(Debug, Properties, PartialEq, Clone)]
pub struct ButtonHtmlAttributes {
    button_attributes: AttributeHolder<ButtonOwnHtmlAttributes>,
    aria_attributes: AttributeHolder<AriaAttributes>,
}

impl ButtonHtmlAttributes {
    pub fn new() -> Self {
        Self {
            button_attributes: AttributeHolder::new(),
            aria_attributes: AttributeHolder::new(),
        }
    }

    pub fn add_btn_attribute(&mut self, attribute: ButtonOwnHtmlAttributes) {
        self.button_attributes.add_attribute(attribute);
    }

    pub fn add_aria_attribute(&mut self, attribute: AriaAttributes) {
        self.aria_attributes.add_attribute(attribute);
    }
}

impl AttributeInjector for ButtonHtmlAttributes {
    fn inject(
        &self,
        node_ref: &yew::NodeRef,
    ) -> Result<(), crate::attribute_injector::SetAttributeError> {
        self.button_attributes.inject(node_ref)?;
        self.aria_attributes.inject(node_ref)?;
        Ok(())
    }
}
