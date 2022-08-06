use domatt::{AriaAttributes, Attribute};

pub trait AriaPropsHandler: super::private::PropsGetterSetter {
    fn add_aria_prop(&mut self, prop: AriaAttributes) {
        let key = prop.get_key();
        let val = prop.get_val();
        self.get_props_to_add().insert(key, val);
    }

    fn remove_aria_prop(&mut self, prop: AriaAttributes) {
        self.get_props_to_remove().push(prop.to_string())
    }
}
