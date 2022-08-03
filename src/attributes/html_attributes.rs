use std::hash::Hash;

use strum::Display;

use crate::attribute_holder::Attribute;

#[derive(Debug, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum HtmlAttributes {
    // Standard HTML Attributes
    AccessKey(String),
    // Note: class is already handled well by Yew, so we are disabling it here.
    // Class(String),
    ContentEditable(ContentEditable),
    ContextMenu(String),
    Dir(String),
    Draggable(bool),
    Hidden(bool),
    Id(String),
    Lang(String),
    Placeholder(String),
    Slot(String),
    SpellCheck(bool),
    // Style(CSSProperties), TODO: make a css properties handler
    TabIndex(u64),
    Title(String),
    Translate(Translate),

    // WAI-ARIA
    Role(AriaRole),

    // RDFa Attributes
    About(String),
    Datatype(String),
    Inlist(String),
    Prefix(String),
    Property(String),
    Resource(String),
    Typeof(String),
    Vocab(String),

    // Non-standard Attributes
    AutoCapitalize(String),
    AutoCorrect(String),
    AutoSave(String),
    Color(String),
    ItemProp(String),
    ItemScope(bool),
    ItemType(String),
    ItemID(String),
    ItemRef(String),
    Results(u64),
    Security(String),
    Unselectable(Unselectable),

    // Living Standard
    /// Hints at the type of data that might be entered by the user while editing the element or its contents
    /// see [https://html.spec.whatwg.org/multipage/interaction.html#input-modalities:-the-inputmode-attribute]
    InputMode(InputMode),

    /// Specify that a standard HTML element should behave like a defined custom built-in element
    /// see [https://html.spec.whatwg.org/multipage/custom-elements.html#attr-is]
    Is(String),
}

#[derive(Debug, Clone, Display, Eq, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum AriaRole {
    Alert,
    AlertDialog,
    Application,
    Article,
    Banner,
    Button,
    Cell,
    Checkbox,
    ColumnHeader,
    Combobox,
    Complementary,
    ContentInfo,
    Definition,
    Dialog,
    Directory,
    Document,
    Feed,
    Figure,
    Form,
    Grid,
    GridCell,
    Group,
    Heading,
    Img,
    Link,
    List,
    ListBox,
    ListItem,
    Log,
    Main,
    Marquee,
    Math,
    Menu,
    Menubar,
    MenuItem,
    MenuItemCheckbox,
    MenuItemRadio,
    Navigation,
    None,
    Note,
    Option,
    Presentation,
    ProgressBar,
    Radio,
    RadioGroup,
    Region,
    Row,
    RowGroup,
    RowHeader,
    ScrollBar,
    Search,
    SearchBox,
    Separator,
    Slider,
    SpinButton,
    Status,
    Switch,
    Tab,
    Table,
    TabLList,
    TabPanel,
    Term,
    TextNox,
    Timer,
    Toolbar,
    Tooltip,
    Tree,
    TreeGrid,
    TreeItem,
    Custom(String),
}

impl Attribute for HtmlAttributes {
    fn get_key(&self) -> String {
        self.to_string()
    }

    fn get_val(&self) -> Option<String> {
        match self {
            HtmlAttributes::AccessKey(val) => Some(val.to_string()),
            HtmlAttributes::ContentEditable(val) => Some(val.to_string()),
            HtmlAttributes::ContextMenu(val) => Some(val.to_string()),
            HtmlAttributes::Dir(val) => Some(val.to_string()),
            HtmlAttributes::Draggable(val) => Some(val.to_string()),
            HtmlAttributes::Hidden(val) => Some(val.to_string()),
            HtmlAttributes::Id(val) => Some(val.to_string()),
            HtmlAttributes::Lang(val) => Some(val.to_string()),
            HtmlAttributes::Placeholder(val) => Some(val.to_string()),
            HtmlAttributes::Slot(val) => Some(val.to_string()),
            HtmlAttributes::SpellCheck(val) => Some(val.to_string()),
            HtmlAttributes::TabIndex(val) => Some(val.to_string()),
            HtmlAttributes::Title(val) => Some(val.to_string()),
            HtmlAttributes::Translate(val) => Some(val.to_string()),
            HtmlAttributes::About(val) => Some(val.to_string()),
            HtmlAttributes::Datatype(val) => Some(val.to_string()),
            HtmlAttributes::Inlist(val) => Some(val.to_string()),
            HtmlAttributes::Prefix(val) => Some(val.to_string()),
            HtmlAttributes::Property(val) => Some(val.to_string()),
            HtmlAttributes::Resource(val) => Some(val.to_string()),
            HtmlAttributes::Typeof(val) => Some(val.to_string()),
            HtmlAttributes::Vocab(val) => Some(val.to_string()),
            HtmlAttributes::AutoCapitalize(val) => Some(val.to_string()),
            HtmlAttributes::AutoCorrect(val) => Some(val.to_string()),
            HtmlAttributes::AutoSave(val) => Some(val.to_string()),
            HtmlAttributes::Color(val) => Some(val.to_string()),
            HtmlAttributes::ItemProp(val) => Some(val.to_string()),
            HtmlAttributes::ItemScope(val) => Some(val.to_string()),
            HtmlAttributes::ItemType(val) => Some(val.to_string()),
            HtmlAttributes::ItemID(val) => Some(val.to_string()),
            HtmlAttributes::ItemRef(val) => Some(val.to_string()),
            HtmlAttributes::Results(val) => Some(val.to_string()),
            HtmlAttributes::Security(val) => Some(val.to_string()),
            HtmlAttributes::Unselectable(val) => Some(val.to_string()),
            HtmlAttributes::InputMode(val) => Some(val.to_string()),
            HtmlAttributes::Is(val) => Some(val.to_string()),
            HtmlAttributes::Role(val) => match val {
                AriaRole::Custom(custom_val) => Some(custom_val.to_string()),
                _ => Some(val.to_string()),
            },
        }
    }
}

impl PartialEq for HtmlAttributes {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Hash for HtmlAttributes {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum ContentEditable {
    True,
    False,
    Inherit,
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Translate {
    Yes,
    No,
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum Unselectable {
    On,
    Off,
}

#[derive(Debug, PartialEq, Clone, Display, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum InputMode {
    None,
    Text,
    Tel,
    Url,
    Email,
    Numeric,
    Decimal,
    Search,
}

pub trait HtmlAttributeReceiver {
    fn add_html_attribute(&mut self, attribute: HtmlAttributes) -> bool;

    fn remove_html_attribute(&mut self, attribute: HtmlAttributes) -> bool;
}
