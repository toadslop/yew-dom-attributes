use strum::Display;

use crate::attribute_holder::Attribute;

/// [https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-autocomplete]
#[derive(Debug, PartialEq, Clone, Display, Hash, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaAutocomplete {
    None,
    Inline,
    List,
    Both,
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaChecked {
    False,
    Mixed,
    True,
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaCurrent {
    False,
    True,
    Page,
    Step,
    Location,
    Date,
    Time,
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaDropeffect {
    None,
    Copy,
    Execute,
    Link,
    Move,
    Popup,
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaHasPopup {
    False,
    True,
    Menu,
    Listbox,
    Tree,
    Grid,
    Dialog,
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaInvalid {
    False,
    True,
    Grammar,
    Spelling,
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaLive {
    Off,
    Assertive,
    Polite,
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaOrientation {
    Horizontal,
    Vertical,
}

type AriaPressed = AriaChecked;

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaRelevant {
    Additions,
    #[strum(serialize = "additions removals")]
    AdditionsRemovals,
    #[strum(serialize = "additions text")]
    AdditionsText,
    All,
    Removals,
    #[strum(serialize = "removals additions")]
    RemovalsAdditions,
    #[strum(serialize = "removals text")]
    RemovalsText,
    Text,
    #[strum(serialize = "text additions")]
    TextAdditions,
    #[strum(serialize = "text removals")]
    TextRemovals,
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaSort {
    None,
    Ascending,
    Descending,
    Other,
}

/// An enum defining the different Aria attribute values. Each variant takes a tuple that represents the valid values
/// for the attributes.
#[derive(Debug, Clone, Display, Eq)]
#[strum(serialize_all = "kebab-case")]
pub enum AriaAttributes {
    /// Identifies the currently active element when DOM focus is on a composite widget, textbox, group, or application.
    AriaActivedescendant(String),

    /// Indicates whether assistive technologies will present all, or only parts of, the changed region based on the
    /// change notifications defined by the aria-relevant attribute.
    AriaAtomic(bool),

    /// Indicates whether inputting text could trigger display of one or more predictions of the user's intended value
    /// for an input and specifies how predictions would be presented if they are made.
    AriaAutocomplete(AriaAutocomplete),

    /// Indicates an element is being modified and that assistive technologies MAY want to wait until the modifications
    /// are complete before exposing them to the user.
    AriaBusy(bool),

    /// Indicates the current "checked" state of checkboxes, radio buttons, and other widgets.
    ///
    /// see [aria_pressed](`AriaAttributes::aria_pressed`) see [aria_selected](`AriaAttributes::aria_selected`).
    AriaChecked(AriaChecked),

    /// Defines the total number of columns in a table, grid, or treegrid.
    ///
    /// see [aria_colindex](`AriaAttributes::aria_colindex`).
    AriaColcount(i64),

    /// Defines an element's column index or position with respect to the total number of columns within a table, grid, or treegrid.
    ///
    /// see [aria_colcount](`AriaAttributes::aria_colcount`) see [aria_colspan](`AriaAttributes::aria_colspan`).
    AriaColindex(i64),

    /// Defines the number of columns spanned by a cell or gridcell within a table, grid, or treegrid.
    ///
    /// see[aria_colindex](`AriaAttributes::aria_colindex`) see [aria_rowspan](`AriaAttributes::aria_rowspan`).
    AriaColspan(i64),

    /// Identifies the element (or elements) whose contents or presence are controlled by the current element.
    ///
    /// see [aria_owns](`AriaAttributes::aria_owns`).
    AriaControls(String),

    /// Indicates the element that represents the current item within a container or set of related elements.
    AriaCurrent(AriaCurrent),

    /// Identifies the element (or elements) that describes the object.
    ///
    /// see [aria_labelledby](`AriaAttributes::aria_labelledby`)
    AriaDescribedby(String),

    /// Identifies the element that provides a detailed, extended description for the object.
    ///
    /// see [aria_describedby](`AriaAttributes::aria_describedby`).
    AriaDetails(String),

    /// Indicates that the element is perceivable but disabled, so it is not editable or otherwise operable.
    ///
    /// see [aria_hidden](`AriaAttributes::aria_hidden`) see [aria_readonly](`AriaAttributes::aria_readonly`).
    AriaDisabled(bool),

    /// Indicates what functions can be performed when a dragged object is released on the drop target.
    AriaDropeffect(AriaDropeffect),

    /// Identifies the element that provides an error message for the object.
    ///
    /// see [aria_invalid](`AriaAttributes::aria_invalid`) see [aria_describedby](`AriaAttributes::aria_describedby`)).
    AriaErrormessage(String),

    /// Indicates whether the element, or another grouping element it controls, is currently expanded or collapsed.
    AriaExpanded(bool),

    /// Identifies the next element (or elements) in an alternate reading order of content which, at the user's discretion,
    /// allows assistive technology to override the general default of reading in document source order.
    AriaFlowto(String),

    /// Indicates an element's "grabbed" state in a drag-and-drop operation.
    AriaGrabbed(bool),

    /// Indicates the availability and type of interactive popup element, such as menu or dialog, that can
    /// be triggered by an element.
    AriaHaspopup(AriaHasPopup),

    /// Indicates whether the element is exposed to an accessibility API.
    ///
    /// see [aria_disabled](`AriaAttributes::aria_disabled`).
    AriaHidden(bool),

    /// Indicates the entered value does not conform to the format expected by the application.
    ///
    /// see [aria_errormessage](`AriaAttributes::aria_errormessage`).
    AriaInvalid(AriaInvalid),

    /// Indicates keyboard shortcuts that an author has implemented to activate or give focus to an element.
    AriaKeyshortcuts(String),

    /// Defines a string value that labels the current element.
    ///
    /// see [aria_labelledby](`AriaAttributes::aria_labelledby`)
    AriaLabel(String),

    /// Identifies the element (or elements) that labels the current element.
    ///
    /// see [aria_describedby](`AriaAttributes::aria_describedby`).
    AriaLabelledby(String),

    /// Defines the hierarchical level of an element within a structure.
    AriaLevel(i64),

    /// Indicates that an element will be updated, and describes the types of updates the user agents, assistive
    /// technologies, and user can expect from the live region.
    AriaLive(AriaLive),

    /// Indicates whether an element is modal when displayed.
    AriaModal(bool),

    /// Indicates whether a text box accepts multiple lines of input or only a single line.
    AriaMultiline(bool),

    /// Indicates that the user may select more than one item from the current selectable descendants.
    AriaMultiselectable(bool),

    /// Indicates whether the element's orientation is horizontal, vertical, or unknown/ambiguous.
    AriaOrientation(AriaOrientation),

    /// Identifies an element (or elements) in order to define a visual, functional, or contextual parent/child relationship
    /// between DOM elements where the DOM hierarchy cannot be used to represent the relationship.
    ///
    /// see [aria_controls](`AriaAttributes::aria_controls`).
    AriaOwns(String),

    /// Defines a short hint (a word or short phrase) intended to aid the user with data entry when the control has no value.
    /// A hint could be a sample value or a brief description of the expected format.
    AriaPlaceholder(String),

    /// Defines an element's number or position in the current set of listitems or treeitems. Not required if all elements
    /// in the set are present in the DOM.
    ///
    /// see [aria_setsize](`AriaAttributes::aria_setsize`)
    AriaPosinset(i64),

    /// Indicates the current "pressed" state of toggle buttons.
    ///
    /// [aria_checked](`AriaAttributes::aria_checked`) see [aria_selected](`AriaAttributes::aria_selected`).
    AriaPressed(AriaPressed),

    /// Indicates that the element is not editable, but is otherwise operable.
    /// see [aria_disabled](`AriaAttributes::aria_disabled`).
    AriaReadonly(bool),

    /// Indicates what notifications the user agent will trigger when the accessibility tree within a live region is modified.
    ///
    /// see [aria_atomic](`AriaAttributes::aria_atomic`).
    AriaRelevant(AriaRelevant),

    /// Indicates that user input is required on the element before a form may be submitted.
    AriaRequired(bool),

    /// Defines a human-readable, author-localized description for the role of an element.
    AriaRoledescription(String),

    /// Defines the total number of rows in a table, grid, or treegrid.
    ///
    /// see [aria_rowindex](`AriaAttributes::aria_rowindex`).
    AriaRowcount(i64),

    /// Defines an element's row index or position with respect to the total number of rows within a table, grid, or treegrid.
    ///
    /// see [aria_rowcount](`AriaAttributes::aria_rowcount`) see [aria_rowspan](`AriaAttributes::aria_rowspan`).
    AriaRowindex(i64),

    /// Defines the number of rows spanned by a cell or gridcell within a table, grid, or treegrid.
    ///
    /// see [aria_rowindex](`AriaAttributes::aria_rowindex`) see [aria_colspan](`AriaAttributes::aria_colspan`).
    AriaRowspan(i64),

    /// Indicates the current "selected" state of various widgets.
    ///
    /// see [aria_checked](`AriaAttributes::aria_checked`) see [aria_pressed](`AriaAttributes::aria_pressed`).
    AriaSelected(bool),

    /// Defines the number of items in the current set of listitems or treeitems. Not required if all
    /// elements in the set are present in the DOM.
    ///
    /// see [aria_posinset](`AriaAttributes::aria_posinset`).
    AriaSetsize(i64),

    /// Indicates if items in a table or grid are sorted in ascending or descending order.
    AriaSort(AriaSort),

    /// Defines the maximum allowed value for a range widget.
    AriaValuemax(i64),

    /// Defines the minimum allowed value for a range widget.
    AriaValuemin(i64),

    /// Defines the current value for a range widget.
    ///
    /// see [aria_valuetext](`AriaAttributes::aria_valuetext`).
    AriaValuenow(i64),

    /// Defines the human readable text alternative of aria-valuenow for a range widget.
    AriaValuetext(String),
}

impl PartialEq for AriaAttributes {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl std::hash::Hash for AriaAttributes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

impl Attribute for AriaAttributes {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match self {
            AriaAttributes::AriaActivedescendant(val) => Some(val.to_string()),
            AriaAttributes::AriaAtomic(val) => Some(val.to_string()),
            AriaAttributes::AriaAutocomplete(val) => Some(val.to_string()),
            AriaAttributes::AriaBusy(val) => Some(val.to_string()),
            AriaAttributes::AriaChecked(val) => Some(val.to_string()),
            AriaAttributes::AriaColcount(val) => Some(val.to_string()),
            AriaAttributes::AriaColindex(val) => Some(val.to_string()),
            AriaAttributes::AriaColspan(val) => Some(val.to_string()),
            AriaAttributes::AriaControls(val) => Some(val.to_string()),
            AriaAttributes::AriaCurrent(val) => Some(val.to_string()),
            AriaAttributes::AriaDescribedby(val) => Some(val.to_string()),
            AriaAttributes::AriaDetails(val) => Some(val.to_string()),
            AriaAttributes::AriaDisabled(val) => Some(val.to_string()),
            AriaAttributes::AriaDropeffect(val) => Some(val.to_string()),
            AriaAttributes::AriaErrormessage(val) => Some(val.to_string()),
            AriaAttributes::AriaExpanded(val) => Some(val.to_string()),
            AriaAttributes::AriaFlowto(val) => Some(val.to_string()),
            AriaAttributes::AriaGrabbed(val) => Some(val.to_string()),
            AriaAttributes::AriaHaspopup(val) => Some(val.to_string()),
            AriaAttributes::AriaHidden(val) => Some(val.to_string()),
            AriaAttributes::AriaInvalid(val) => Some(val.to_string()),
            AriaAttributes::AriaKeyshortcuts(val) => Some(val.to_string()),
            AriaAttributes::AriaLabel(val) => Some(val.to_string()),
            AriaAttributes::AriaLabelledby(val) => Some(val.to_string()),
            AriaAttributes::AriaLevel(val) => Some(val.to_string()),
            AriaAttributes::AriaLive(val) => Some(val.to_string()),
            AriaAttributes::AriaModal(val) => Some(val.to_string()),
            AriaAttributes::AriaMultiline(val) => Some(val.to_string()),
            AriaAttributes::AriaMultiselectable(val) => Some(val.to_string()),
            AriaAttributes::AriaOrientation(val) => Some(val.to_string()),
            AriaAttributes::AriaOwns(val) => Some(val.to_string()),
            AriaAttributes::AriaPlaceholder(val) => Some(val.to_string()),
            AriaAttributes::AriaPosinset(val) => Some(val.to_string()),
            AriaAttributes::AriaPressed(val) => Some(val.to_string()),
            AriaAttributes::AriaReadonly(val) => Some(val.to_string()),
            AriaAttributes::AriaRelevant(val) => Some(val.to_string()),
            AriaAttributes::AriaRequired(val) => Some(val.to_string()),
            AriaAttributes::AriaRoledescription(val) => Some(val.to_string()),
            AriaAttributes::AriaRowcount(val) => Some(val.to_string()),
            AriaAttributes::AriaRowindex(val) => Some(val.to_string()),
            AriaAttributes::AriaRowspan(val) => Some(val.to_string()),
            AriaAttributes::AriaSelected(val) => Some(val.to_string()),
            AriaAttributes::AriaSetsize(val) => Some(val.to_string()),
            AriaAttributes::AriaSort(val) => Some(val.to_string()),
            AriaAttributes::AriaValuemax(val) => Some(val.to_string()),
            AriaAttributes::AriaValuemin(val) => Some(val.to_string()),
            AriaAttributes::AriaValuenow(val) => Some(val.to_string()),
            AriaAttributes::AriaValuetext(val) => Some(val.to_string()),
        }
    }
}

pub trait AriaAttributeReceiver {
    fn add_aria_attribute(&mut self, attribute: AriaAttributes) -> bool;

    fn remove_aria_attribute(&mut self, attribute: AriaAttributes) -> bool;
}
