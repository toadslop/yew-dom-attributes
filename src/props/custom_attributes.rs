pub trait CustomPropsHandler: super::private::PropsGetterSetter {
    fn add_custom_prop(&mut self, prop: CustomAttribute) {
        let key = prop.get_key();
        let val = prop.get_value();
        self.get_props_to_add().insert(key, val);
    }

    fn remove_custom_prop(&mut self, prop: CustomAttribute) {
        self.get_props_to_remove().push(prop.get_key())
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct CustomAttribute {
    key: String,
    value: Option<String>,
}

impl CustomAttribute {
    pub fn new_key_value_attribute(key: String, value: String) -> Self {
        Self {
            key,
            value: Some(value),
        }
    }

    pub fn new_boolean_attribute(key: String) -> Self {
        Self { key, value: None }
    }

    fn get_key(&self) -> String {
        self.key.clone()
    }

    fn get_value(&self) -> Option<String> {
        self.value.clone()
    }
}
