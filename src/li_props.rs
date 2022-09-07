use super::{from_global, prop_handler, DomInjector};
use domatt::attributes::li::LiAttribute;

prop_handler!(LiProps, LiAttribute);
from_global!(LiProps);
