use std::fmt::{Display, Formatter};
use yew::NodeRef;

/// This trait is used to define how a struct containing attributes should be converted into
/// html attributes and injected to the DOM.
pub trait AttributeInjector {
    /// The inject function takes a NodeRef in which to inject the fields of the struct as html attributes.
    /// You'll need to convert the NodeRef to an Element and use set_attribute.
    fn inject(&self, node_ref: &NodeRef) -> Result<(), SetAttributeError>;
}

/// This error indicates that an injection of an attribute to the DOM failed.
#[derive(Debug, Clone)]
pub struct SetAttributeError {
    key: String,
    value: Option<String>,
}

impl SetAttributeError {
    pub fn new(key: String, value: Option<String>) -> Self {
        Self { key, value }
    }
}

impl Display for SetAttributeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = if let Some(value) = &self.value {
            value.to_owned()
        } else {
            "None".into()
        };
        write!(
            f,
            "Failed to insert attribute to the DOM. Key: {} Value: {}",
            self.key, value
        )
    }
}

// pub fn default_to_html_val_string(value_info: &ValueInfo, value: &dyn Reflect) -> Option<String> {
//     let type_name = value_info.type_name();
//     let result = match type_name {
//         "core::option::Option<alloc::string::String>" => {
//             let downcast = value.downcast_ref::<Option<String>>().unwrap();
//             convert_to_string(downcast)
//         }
//         "core::option::Option<bool>" => {
//             let downcast = value.downcast_ref::<Option<bool>>().unwrap();
//             convert_to_string(downcast)
//         }
//         "core::option::Option<u64>" => {
//             let downcast = value.downcast_ref::<Option<u64>>().unwrap();
//             convert_to_string(downcast)
//         }
//         _ => None,
//     };

//     result
// }

// pub fn convert_to_string(option: &Option<impl Display>) -> Option<String> {
//     if let Some(val) = option {
//         Some(val.to_string())
//     } else {
//         None
//     }
// }
