use std::fmt::Display;

use bevy_reflect::{Reflect, ValueInfo};
use yew::NodeRef;

/// This trait is used to define how a struct should be converted into html attributes and injected to the DOM.
pub trait AttributeCollection {
    /// The inject function takes a NodeRef in which to inject the fields of the struct as html attributes.
    /// You'll need to convert the NodeRef to an Element and use set_attribute.
    fn inject(&self, node_ref: &NodeRef);
}

pub fn default_to_html_val_string(value_info: &ValueInfo, value: &dyn Reflect) -> Option<String> {
    let type_name = value_info.type_name();
    let result = match type_name {
        "core::option::Option<alloc::string::String>" => {
            let downcast = value.downcast_ref::<Option<String>>().unwrap();
            convert_to_string(downcast)
        }
        "core::option::Option<bool>" => {
            let downcast = value.downcast_ref::<Option<bool>>().unwrap();
            convert_to_string(downcast)
        }
        "core::option::Option<u64>" => {
            let downcast = value.downcast_ref::<Option<u64>>().unwrap();
            convert_to_string(downcast)
        }
        _ => None,
    };

    result
}

pub fn convert_to_string(option: &Option<impl Display>) -> Option<String> {
    if let Some(val) = option {
        Some(val.to_string())
    } else {
        None
    }
}
