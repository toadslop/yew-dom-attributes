use super::{prop_handler, DomInjector, ProcessAction};
use domatt::attributes::svg::SvgAttribute;
use domatt::events::Event;
use std::rc::Rc;
use yew::{Callback, Context, Properties};

prop_handler!(SvgProps, SvgAttribute);
