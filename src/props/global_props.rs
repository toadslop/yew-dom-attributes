use super::{prop_handler, DomInjector, ProcessAction};
use domatt::attributes::global::GlobalAttribute;
use domatt::events::Event;
use std::rc::Rc;
use yew::{Callback, Context, Properties};

prop_handler!(GlobalProps, GlobalAttribute);
