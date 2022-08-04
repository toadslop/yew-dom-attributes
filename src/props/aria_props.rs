use crate::attributes::aria_attributes::AriaAttributes;

pub trait AriaPropsHandler: super::private::PropsGetterSetter {
    fn add_aria_prop(&mut self, prop: AriaAttributes) {
        let key = prop.to_string();
        let val = match &prop {
            AriaAttributes::AriaActivedescendant(val) => Some(val.to_owned()),
            AriaAttributes::AriaAtomic(val) => Some(val.to_string()),
            AriaAttributes::AriaAutocomplete(val) => Some(val.to_string()),
            AriaAttributes::AriaBusy(val) => Some(val.to_string()),
            AriaAttributes::AriaChecked(val) => Some(val.to_string()),
            AriaAttributes::AriaColcount(val) => Some(val.to_string()),
            AriaAttributes::AriaColindex(val) => Some(val.to_string()),
            AriaAttributes::AriaColspan(val) => Some(val.to_string()),
            AriaAttributes::AriaControls(val) => Some(val.to_owned()),
            AriaAttributes::AriaCurrent(val) => Some(val.to_string()),
            AriaAttributes::AriaDescribedby(val) => Some(val.to_owned()),
            AriaAttributes::AriaDetails(val) => Some(val.to_owned()),
            AriaAttributes::AriaDisabled(val) => Some(val.to_string()),
            AriaAttributes::AriaDropeffect(val) => Some(val.to_string()),
            AriaAttributes::AriaErrormessage(val) => Some(val.to_owned()),
            AriaAttributes::AriaExpanded(val) => Some(val.to_string()),
            AriaAttributes::AriaFlowto(val) => Some(val.to_owned()),
            AriaAttributes::AriaGrabbed(val) => Some(val.to_string()),
            AriaAttributes::AriaHaspopup(val) => Some(val.to_string()),
            AriaAttributes::AriaHidden(val) => Some(val.to_string()),
            AriaAttributes::AriaInvalid(val) => Some(val.to_string()),
            AriaAttributes::AriaKeyshortcuts(val) => Some(val.to_owned()),
            AriaAttributes::AriaLabel(val) => Some(val.to_owned()),
            AriaAttributes::AriaLabelledby(val) => Some(val.to_owned()),
            AriaAttributes::AriaLevel(val) => Some(val.to_string()),
            AriaAttributes::AriaLive(val) => Some(val.to_string()),
            AriaAttributes::AriaModal(val) => Some(val.to_string()),
            AriaAttributes::AriaMultiline(val) => Some(val.to_string()),
            AriaAttributes::AriaMultiselectable(val) => Some(val.to_string()),
            AriaAttributes::AriaOrientation(val) => Some(val.to_string()),
            AriaAttributes::AriaOwns(val) => Some(val.to_owned()),
            AriaAttributes::AriaPlaceholder(val) => Some(val.to_owned()),
            AriaAttributes::AriaPosinset(val) => Some(val.to_string()),
            AriaAttributes::AriaPressed(val) => Some(val.to_string()),
            AriaAttributes::AriaReadonly(val) => Some(val.to_string()),
            AriaAttributes::AriaRelevant(val) => Some(val.to_string()),
            AriaAttributes::AriaRequired(val) => Some(val.to_string()),
            AriaAttributes::AriaRoledescription(val) => Some(val.to_owned()),
            AriaAttributes::AriaRowcount(val) => Some(val.to_string()),
            AriaAttributes::AriaRowindex(val) => Some(val.to_string()),
            AriaAttributes::AriaRowspan(val) => Some(val.to_string()),
            AriaAttributes::AriaSelected(val) => Some(val.to_string()),
            AriaAttributes::AriaSetsize(val) => Some(val.to_string()),
            AriaAttributes::AriaSort(val) => Some(val.to_string()),
            AriaAttributes::AriaValuemax(val) => Some(val.to_string()),
            AriaAttributes::AriaValuemin(val) => Some(val.to_string()),
            AriaAttributes::AriaValuenow(val) => Some(val.to_string()),
            AriaAttributes::AriaValuetext(val) => Some(val.to_owned()),
        };
        self.get_props_to_add().insert(key, val);
    }

    fn remove_aria_prop(&mut self, prop: AriaAttributes) {
        self.get_props_to_remove().push(prop.to_string())
    }
}
