use super::{from_global, prop_handler, DomInjector};
use domatt::attributes::button::ButtonAttribute;

prop_handler!(ButtonProps, ButtonAttribute);
from_global!(ButtonProps);
