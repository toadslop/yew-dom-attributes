use super::{prop_handler, DomInjector};
use domatt::attributes::button::ButtonAttribute;
use domatt::events::Event;
use std::collections::HashMap;
use std::rc::Rc;
use yew::{Callback, Context, Properties};

prop_handler!(ButtonProps, ButtonAttribute);
