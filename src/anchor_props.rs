use super::{from_global, prop_handler, DomInjector};
use domatt::attributes::anchor::AnchorAttribute;

prop_handler!(AnchorProps, AnchorAttribute);
from_global!(AnchorProps);
