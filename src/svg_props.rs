use super::{prop_handler, DomInjector};
use domatt::attributes::svg::SvgAttribute;
use domatt::events::Event;
use std::collections::HashMap;
use std::rc::Rc;
use yew::{Callback, Properties};

prop_handler!(SvgProps, SvgAttribute);
