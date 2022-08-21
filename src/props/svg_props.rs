use super::{prop_handler, DomInjector, ProcessAction};
use crate::events::events::EventType;
use domatt::attributes::svg::SvgAttribute;
use std::{collections::HashMap, rc::Rc};
use yew::{Callback, Context, Properties};

prop_handler!(SvgProps, SvgAttribute);
