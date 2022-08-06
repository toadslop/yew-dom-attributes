use std::{collections::HashMap, rc::Rc};

use yew::{Callback, Properties};

use crate::{
    attributes::{aria_attributes::AriaAttributes, html_attributes::HtmlAttributes},
    events::events::EventType,
};

use super::{aria_props::AriaPropsHandler, custom_attributes::CustomPropsHandler, DomInjector};

/// Properties for a generic Yew components.
#[derive(Debug, Properties, PartialEq, Clone)]
pub struct HtmlElementProps {
    attributes_to_add: HashMap<String, Option<String>>,
    attributes_to_remove: Vec<String>,
    listeners_to_add: HashMap<String, EventType>,
    listeners_to_remove: Vec<String>,
    /// A callback used to pass changes to button props from the child back up to the parent.
    /// This is necessary to inform the parent that attributes and listeners were either
    /// added or removed from the DOM. If this is not used properly, your component will
    /// not know that it happened and will try again on the next rerender.
    on_props_update: Callback<Rc<HtmlElementProps>>,
}

/// A trait to be implemented by any type that accepts button props.
pub trait HtmlElementPropsHandler: super::private::PropsGetterSetter {
    fn add_html_prop(&mut self, prop: HtmlAttributes) {
        let key = prop.to_string();
        let val = match &prop {
            HtmlAttributes::AccessKey(val) => Some(val.to_string()),
            HtmlAttributes::ContentEditable(val) => Some(val.to_string()),
            HtmlAttributes::ContextMenu(val) => Some(val.to_string()),
            HtmlAttributes::Dir(val) => Some(val.to_string()),
            HtmlAttributes::Draggable(val) => Some(val.to_string()),
            HtmlAttributes::Hidden(val) => Some(val.to_string()),
            HtmlAttributes::Id(val) => Some(val.to_string()),
            HtmlAttributes::Lang(val) => Some(val.to_string()),
            HtmlAttributes::Placeholder(val) => Some(val.to_string()),
            HtmlAttributes::Slot(val) => Some(val.to_string()),
            HtmlAttributes::SpellCheck(val) => Some(val.to_string()),
            HtmlAttributes::TabIndex(val) => Some(val.to_string()),
            HtmlAttributes::Title(val) => Some(val.to_string()),
            HtmlAttributes::Translate(val) => Some(val.to_string()),
            HtmlAttributes::Role(val) => Some(val.to_string()),
            HtmlAttributes::About(val) => Some(val.to_string()),
            HtmlAttributes::Datatype(val) => Some(val.to_string()),
            HtmlAttributes::Inlist(val) => Some(val.to_string()),
            HtmlAttributes::Prefix(val) => Some(val.to_string()),
            HtmlAttributes::Property(val) => Some(val.to_string()),
            HtmlAttributes::Resource(val) => Some(val.to_string()),
            HtmlAttributes::Typeof(val) => Some(val.to_string()),
            HtmlAttributes::Vocab(val) => Some(val.to_string()),
            HtmlAttributes::AutoCapitalize(val) => Some(val.to_string()),
            HtmlAttributes::AutoCorrect(val) => Some(val.to_string()),
            HtmlAttributes::AutoSave(val) => Some(val.to_string()),
            HtmlAttributes::Color(val) => Some(val.to_string()),
            HtmlAttributes::ItemProp(val) => Some(val.to_string()),
            HtmlAttributes::ItemScope(val) => Some(val.to_string()),
            HtmlAttributes::ItemType(val) => Some(val.to_string()),
            HtmlAttributes::ItemID(val) => Some(val.to_string()),
            HtmlAttributes::ItemRef(val) => Some(val.to_string()),
            HtmlAttributes::Results(val) => Some(val.to_string()),
            HtmlAttributes::Security(val) => Some(val.to_string()),
            HtmlAttributes::Unselectable(val) => Some(val.to_string()),
            HtmlAttributes::InputMode(val) => Some(val.to_string()),
            HtmlAttributes::Is(val) => Some(val.to_string()),
        };
        self.get_props_to_add().insert(key, val);
    }

    fn remove_html_prop(&mut self, prop: AriaAttributes) {
        self.get_props_to_remove().push(prop.to_string())
    }
}

impl super::private::PropsGetterSetter for HtmlElementProps {
    fn get_props_to_add(&mut self) -> &mut HashMap<String, Option<String>> {
        &mut self.attributes_to_add
    }

    fn get_props_to_remove(&mut self) -> &mut Vec<String> {
        &mut self.attributes_to_remove
    }
}

impl super::private::ListenerGetterSetter for HtmlElementProps {
    fn get_listeners_to_add(&mut self) -> &mut HashMap<String, EventType> {
        &mut self.listeners_to_add
    }

    fn get_listeners_to_remove(&mut self) -> &mut Vec<String> {
        &mut self.listeners_to_remove
    }
}

impl AriaPropsHandler for HtmlElementProps {}
impl HtmlElementPropsHandler for HtmlElementProps {}
impl CustomPropsHandler for HtmlElementProps {}
impl DomInjector for HtmlElementProps {
    fn new(on_props_update: Callback<Rc<Self>>) -> Self {
        Self {
            attributes_to_add: HashMap::new(),
            attributes_to_remove: Vec::new(),
            listeners_to_add: HashMap::new(),
            listeners_to_remove: Vec::new(),
            on_props_update,
        }
    }

    fn get_props_update_callback(&self) -> &Callback<Rc<Self>> {
        &self.on_props_update
    }
}
