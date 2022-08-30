use super::{prop_handler, DomInjector};
use domatt::attributes::anchor::AnchorAttribute;
use domatt::events::Event;
use std::collections::HashMap;
use std::rc::Rc;
use yew::Properties;

prop_handler!(AnchorProps, AnchorAttribute);
