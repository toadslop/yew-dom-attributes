use bevy_reflect::{FromReflect, Reflect, Struct, ValueInfo};
use strum::Display;
use web_sys::{console, Element};
use yew::{NodeRef, Properties};

use crate::attribute_collection::{
    convert_to_string, default_to_html_val_string, AttributeCollection,
};

#[derive(Debug, PartialEq, Clone, Reflect, Display, FromReflect)]
#[strum(serialize_all = "kebab-case")]
pub enum AriaAutocomplete {
    None,
    Inline,
    List,
    Both,
}

#[derive(Debug, PartialEq, Clone, Reflect, Display, FromReflect)]
#[strum(serialize_all = "kebab-case")]
pub enum AriaChecked {
    False,
    Mixed,
    True,
}

#[derive(Debug, PartialEq, Clone, Reflect, Display, FromReflect)]
#[strum(serialize_all = "kebab-case")]
pub enum AriaCurrent {
    False,
    True,
    Page,
    Step,
    Location,
    Date,
    Time,
}

#[derive(Debug, PartialEq, Clone, Reflect, Display, FromReflect)]
#[strum(serialize_all = "kebab-case")]
pub enum AriaDropeffect {
    None,
    Copy,
    Execute,
    Link,
    Move,
    Popup,
}

#[derive(Debug, PartialEq, Clone, Reflect, Display, FromReflect)]
#[strum(serialize_all = "kebab-case")]
pub enum AriaHasPopup {
    False,
    True,
    Menu,
    Listbox,
    Tree,
    Grid,
    Dialog,
}

#[derive(Debug, PartialEq, Clone, Reflect, Display, FromReflect)]
#[strum(serialize_all = "kebab-case")]
pub enum AriaInvalid {
    False,
    True,
    Grammar,
    Spelling,
}

#[derive(Debug, PartialEq, Clone, Reflect, Display, FromReflect)]
#[strum(serialize_all = "kebab-case")]
pub enum AriaLive {
    Off,
    Assertive,
    Polite,
}

#[derive(Debug, PartialEq, Clone, Reflect, Display, FromReflect)]
#[strum(serialize_all = "kebab-case")]
pub enum AriaOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Properties, PartialEq, Clone, Reflect, FromReflect, Default)]
pub struct AriaAttributes {
    /// Identifies the currently active element when DOM focus is on a composite widget, textbox, group, or application.
    #[prop_or_default]
    pub aria_activedescendant: Option<String>,

    /// Indicates whether assistive technologies will present all, or only parts of, the changed region based on the
    /// change notifications defined by the aria-relevant attribute.
    #[prop_or_default]
    pub aria_atomic: Option<bool>,

    ///  Indicates whether inputting text could trigger display of one or more predictions of the user's intended value
    /// for an input and specifies how predictions would be presented if they are made.
    #[prop_or_default]
    pub aria_autocomplete: Option<AriaAutocomplete>,

    /// Indicates an element is being modified and that assistive technologies MAY want to wait until the modifications
    /// are complete before exposing them to the user.
    #[prop_or_default]
    pub aria_busy: Option<bool>,

    /// Indicates the current "checked" state of checkboxes, radio buttons, and other widgets.
    ///
    /// see [aria_pressed](`AriaAttributes::aria_pressed`) see [aria_selected](`AriaAttributes::aria_selected`).
    #[prop_or_default]
    pub aria_checked: Option<AriaChecked>,

    /// Defines the total number of columns in a table, grid, or treegrid.
    ///
    /// see [aria_colindex](`AriaAttributes::aria_colindex`).
    #[prop_or_default]
    pub aria_colcount: Option<u64>,

    /// Defines an element's column index or position with respect to the total number of columns within a table, grid, or treegrid.
    ///
    /// see [aria_colcount](`AriaAttributes::aria_colcount`) see [aria_colspan](`AriaAttributes::aria_colspan`).
    #[prop_or_default]
    pub aria_colindex: Option<u64>,

    /// Defines the number of columns spanned by a cell or gridcell within a table, grid, or treegrid.
    ///
    /// see[aria_colindex](`AriaAttributes::aria_colindex`) see [aria_rowspan](`AriaAttributes::aria_rowspan`).
    #[prop_or_default]
    pub aria_colspan: Option<u64>,

    /// Identifies the element (or elements) whose contents or presence are controlled by the current element.
    ///
    /// see [aria_owns](`AriaAttributes::aria_owns`).
    #[prop_or_default]
    pub aria_controls: Option<String>,

    /// Indicates the element that represents the current item within a container or set of related elements.
    #[prop_or_default]
    pub aria_current: Option<AriaCurrent>,

    /// Identifies the element (or elements) that describes the object.
    ///
    /// see [aria_labelledby](`AriaAttributes::aria_labelledby`)
    #[prop_or_default]
    pub aria_describedby: Option<String>,

    // Identifies the element that provides a detailed, extended description for the object.

    // see [aria_describedby](`AriaAttributes::aria_describedby`).
    #[prop_or_default]
    pub aria_details: Option<String>,

    /// Indicates that the element is perceivable but disabled, so it is not editable or otherwise operable.
    ///
    /// see [aria_hidden](`AriaAttributes::aria_hidden`) see [aria_readonly](`AriaAttributes::aria_readonly`).
    #[prop_or_default]
    pub aria_disabled: Option<bool>,

    /// Indicates what functions can be performed when a dragged object is released on the drop target.
    #[deprecated(since = "ARIA 1.1")]
    #[prop_or_default]
    pub aria_dropeffect: Option<AriaDropeffect>,

    /// Identifies the element that provides an error message for the object.
    ///
    /// see [aria_invalid](`AriaAttributes::aria_invalid`) see [aria_describedby](`AriaAttributes::aria_describedby`)).
    #[prop_or_default]
    pub aria_errormessage: Option<String>,

    /// Indicates whether the element, or another grouping element it controls, is currently expanded or collapsed. */
    #[prop_or_default]
    pub aria_expanded: Option<bool>,

    /// Identifies the next element (or elements) in an alternate reading order of content which, at the user's discretion,
    /// allows assistive technology to override the general default of reading in document source order.
    #[prop_or_default]
    pub aria_flowto: Option<String>,

    /// Indicates an element's "grabbed" state in a drag-and-drop operation.
    #[deprecated(since = "ARIA 1.1")]
    pub aria_grabbed: Option<bool>,

    /// Indicates the availability and type of interactive popup element, such as menu or dialog, that can be triggered by an element. */
    #[prop_or_default]
    pub aria_haspopup: Option<AriaHasPopup>,

    /// Indicates whether the element is exposed to an accessibility API.
    /// see [aria_disabled](`AriaAttributes::aria_disabled`).
    #[prop_or_default]
    pub aria_hidden: Option<bool>,

    /// Indicates the entered value does not conform to the format expected by the application.
    /// see [aria_errormessage](`AriaAttributes::aria_errormessage`).
    #[prop_or_default]
    pub aria_invalid: Option<AriaInvalid>,

    /// Indicates keyboard shortcuts that an author has implemented to activate or give focus to an element.
    #[prop_or_default]
    pub aria_keyshortcuts: Option<String>,

    /// Defines a string value that labels the current element.
    /// see [aria_labelledby](`AriaAttributes::aria_labelledby`)
    #[prop_or_default]
    pub aria_label: Option<String>,

    /// Identifies the element (or elements) that labels the current element.
    /// @see [aria_describedby](`AriaAttributes::aria_describedby`).
    #[prop_or_default]
    pub aria_labelledby: Option<String>,

    /// Defines the hierarchical level of an element within a structure.
    #[prop_or_default]
    pub aria_level: Option<u64>,

    /// Indicates that an element will be updated, and describes the types of updates the user agents, assistive
    /// technologies, and user can expect from the live region.
    #[prop_or_default]
    pub aria_live: Option<AriaLive>,

    /// /** Indicates whether an element is modal when displayed. */
    #[prop_or_default]
    pub aria_modal: Option<bool>,

    /// Indicates whether a text box accepts multiple lines of input or only a single line.
    #[prop_or_default]
    pub aria_multiline: Option<bool>,

    /// Indicates that the user may select more than one item from the current selectable descendants.
    #[prop_or_default]
    pub aria_multiselectable: Option<bool>,

    /// Indicates whether the element's orientation is horizontal, vertical, or unknown/ambiguous.
    pub aria_orientation: Option<AriaOrientation>,

    /// Identifies an element (or elements) in order to define a visual, functional, or contextual parent/child relationship
    /// between DOM elements where the DOM hierarchy cannot be used to represent the relationship.
    /// see [aria_controls](`AriaAttributes::aria_controls`).
    pub aria_owns: Option<String>, // /**
                                   //  * Defines a short hint (a word or short phrase) intended to aid the user with data entry when the control has no value.
                                   //  * A hint could be a sample value or a brief description of the expected format.
                                   //  */
                                   // 'aria-placeholder'?: string | undefined;
                                   // /**
                                   //  * Defines an element's number or position in the current set of listitems or treeitems. Not required if all elements in the set are present in the DOM.
                                   //  * @see aria-setsize.
                                   //  */
                                   // 'aria-posinset'?: number | undefined;
                                   // /**
                                   //  * Indicates the current "pressed" state of toggle buttons.
                                   //  * @see aria-checked @see aria-selected.
                                   //  */
                                   // 'aria-pressed'?: boolean | 'false' | 'mixed' | 'true' | undefined;
                                   // /**
                                   //  * Indicates that the element is not editable, but is otherwise operable.
                                   //  * @see aria-disabled.
                                   //  */
                                   // 'aria-readonly'?: Booleanish | undefined;
                                   // /**
                                   //  * Indicates what notifications the user agent will trigger when the accessibility tree within a live region is modified.
                                   //  * @see aria-atomic.
                                   //  */
                                   // 'aria-relevant'?: 'additions' | 'additions removals' | 'additions text' | 'all' | 'removals' | 'removals additions' | 'removals text' | 'text' | 'text additions' | 'text removals' | undefined;
                                   // /** Indicates that user input is required on the element before a form may be submitted. */
                                   // 'aria-required'?: Booleanish | undefined;
                                   // /** Defines a human-readable, author-localized description for the role of an element. */
                                   // 'aria-roledescription'?: string | undefined;
                                   // /**
                                   //  * Defines the total number of rows in a table, grid, or treegrid.
                                   //  * @see aria-rowindex.
                                   //  */
                                   // 'aria-rowcount'?: number | undefined;
                                   // /**
                                   //  * Defines an element's row index or position with respect to the total number of rows within a table, grid, or treegrid.
                                   //  * @see aria-rowcount @see aria-rowspan.
                                   //  */
                                   // 'aria-rowindex'?: number | undefined;
                                   // /**
                                   //  * Defines the number of rows spanned by a cell or gridcell within a table, grid, or treegrid.
                                   //  * @see aria-rowindex @see aria-colspan.
                                   //  */
                                   // 'aria-rowspan'?: number | undefined;
                                   // /**
                                   //  * Indicates the current "selected" state of various widgets.
                                   //  * @see aria-checked @see aria-pressed.
                                   //  */
                                   // 'aria-selected'?: Booleanish | undefined;
                                   // /**
                                   //  * Defines the number of items in the current set of listitems or treeitems. Not required if all elements in the set are present in the DOM.
                                   //  * @see aria-posinset.
                                   //  */
                                   // 'aria-setsize'?: number | undefined;
                                   // /** Indicates if items in a table or grid are sorted in ascending or descending order. */
                                   // 'aria-sort'?: 'none' | 'ascending' | 'descending' | 'other' | undefined;
                                   // /** Defines the maximum allowed value for a range widget. */
                                   // 'aria-valuemax'?: number | undefined;
                                   // /** Defines the minimum allowed value for a range widget. */
                                   // 'aria-valuemin'?: number | undefined;
                                   // /**
                                   //  * Defines the current value for a range widget.
                                   //  * @see aria-valuetext.
                                   //  */
                                   // 'aria-valuenow'?: number | undefined;
                                   // /** Defines the human readable text alternative of aria-valuenow for a range widget. */
                                   // 'aria-valuetext'?: string | undefined;
}

impl AttributeCollection for AriaAttributes {
    fn inject(&self, node_ref: &NodeRef) {
        if let Some(elem) = node_ref.cast::<Element>() {
            for (i, value) in self.iter_fields().enumerate() {
                let field_name = self.name_at(i).unwrap().replace("_", "-");
                console::log_1(&field_name.clone().into());
                match value.get_type_info() {
                    bevy_reflect::TypeInfo::Struct(_) => todo!(),
                    bevy_reflect::TypeInfo::TupleStruct(_) => todo!(),
                    bevy_reflect::TypeInfo::Tuple(_) => todo!(),
                    bevy_reflect::TypeInfo::List(_) => todo!(),
                    bevy_reflect::TypeInfo::Array(_) => todo!(),
                    bevy_reflect::TypeInfo::Map(_) => todo!(),
                    bevy_reflect::TypeInfo::Value(value_info) => {
                        let name = value_info.type_name();
                        let html_attr_val = aria_to_html_val_string(value_info, value);
                        if let Some(attr_val) = html_attr_val {
                            elem.set_attribute(field_name.as_str(), attr_val.as_str())
                                .unwrap();
                        }

                        console::log_1(&name.into());
                    }
                    bevy_reflect::TypeInfo::Dynamic(_) => todo!(),
                }
            }
        }
    }
}

pub fn aria_to_html_val_string(value_info: &ValueInfo, value: &dyn Reflect) -> Option<String> {
    let type_name = value_info.type_name();
    let result = match type_name {
        "core::option::Option<yew_dom_attributes::aria_attributes::AriaAutocomplete>" => {
            let downcast = value.downcast_ref::<Option<AriaAutocomplete>>().unwrap();
            convert_to_string(downcast)
        }
        "core::option::Option<yew_dom_attributes::aria_attributes::AriaChecked>" => {
            let downcast = value.downcast_ref::<Option<AriaChecked>>().unwrap();
            convert_to_string(downcast)
        }
        "core::option::Option<yew_dom_attributes::aria_attributes::AriaHasPopup>" => {
            let downcast = value.downcast_ref::<Option<AriaHasPopup>>().unwrap();
            convert_to_string(downcast)
        }
        "core::option::Option<yew_dom_attributes::aria_attributes::AriaCurrent>" => {
            let downcast = value.downcast_ref::<Option<AriaCurrent>>().unwrap();
            convert_to_string(downcast)
        }
        "core::option::Option<yew_dom_attributes::aria_attributes::AriaDropeffect>" => {
            let downcast = value.downcast_ref::<Option<AriaDropeffect>>().unwrap();
            convert_to_string(downcast)
        }
        "core::option::Option<yew_dom_attributes::aria_attributes::AriaInvalid>" => {
            let downcast = value.downcast_ref::<Option<AriaInvalid>>().unwrap();
            convert_to_string(downcast)
        }
        "core::option::Option<yew_dom_attributes::aria_attributes::AriaLive>" => {
            let downcast = value.downcast_ref::<Option<AriaLive>>().unwrap();
            convert_to_string(downcast)
        }
        "core::option::Option<yew_dom_attributes::aria_attributes::AriaOrientation>" => {
            let downcast = value.downcast_ref::<Option<AriaOrientation>>().unwrap();
            convert_to_string(downcast)
        }
        _ => default_to_html_val_string(value_info, value),
    };

    result
}
