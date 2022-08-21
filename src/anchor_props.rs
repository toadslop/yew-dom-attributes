use super::{prop_handler, DomInjector, ProcessAction};
use domatt::attributes::anchor::AnchorAttribute;
use domatt::events::Event;
use std::rc::Rc;
use yew::{Callback, Context, Properties};

prop_handler!(AnchorProps, AnchorAttribute);
