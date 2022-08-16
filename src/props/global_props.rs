use super::{prop_handler, DomInjector, ProcessAction};
use crate::events::events::EventType;
use domatt::global::GlobalAttribute;
use std::{collections::HashMap, rc::Rc};
use yew::{Callback, Context, Properties};

prop_handler!(GlobalProps, GlobalAttribute);
