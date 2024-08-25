use gtk4 as gtk;
use serde::Deserialize;

pub mod build;

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum Component {
    /// Password form field component
    Password(FieldComponent),
    /// Username form field component
    Username(FieldComponent),
    /// Runner select form field component
    Runner(ComponentWithClasses),
    /// Box component
    Box(BoxComponent),
    /// Label component displaying the current date time
    DateTime(DateTimeComponent),
    /// Label component displaying any text
    Label(LabelComponent)
}

#[derive(Deserialize, Clone, Debug)]
pub struct ComponentWithClasses {
    /// Css classes which are applied to the component
    pub classes: Vec<String>
}

impl Default for ComponentWithClasses {
    fn default() -> Self {
        Self {
            classes: Vec::new()
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct DateTimeComponent {
    /// Css classes which are applied to the datetime label
    pub classes: Vec<String>,
    /// Format string used for the date
    ///
    /// Reference: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    pub format: String,
    /// Milliseconds after which the date time should be updated
    pub interval: u32,
}

impl Default for DateTimeComponent {
    fn default() -> Self {
        Self {
            classes: vec![String::from("label"), String::from("datetime")],
            format: String::from("%H:%M"),
            interval: 1000
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct LabelComponent {
    /// Css classes which are applied to the label
    #[serde(default)]
    pub classes: Vec<String>,
    /// Static text value of the label
    pub label: String
}

impl Default for LabelComponent {
    fn default() -> Self {
        Self {
            classes: vec![String::from("label")],
            label: String::new()
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct FieldComponent {
    /// Css classes which are applied to the form field
    pub classes: Vec<String>,
    /// Placeholder text for the form field
    pub placeholder: String,
}

impl Default for FieldComponent {
    fn default() -> Self {
        Self {
            classes: vec![String::from("field")],
            placeholder: String::new()
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct BoxComponent {
    /// Css classes which are applied to the box
    pub classes: Vec<String>,
    /// Horizontal alignment of children in the box
    pub halign: Align,
    /// Vertical alignment of children in the box
    pub valign: Align,
    /// Boolean whether the box should expand horizontally
    pub hexpand: bool,
    /// Boolean whether the box should expand vertically
    pub vexpand: bool,
    /// Requested width of the box
    pub width: i32,
    /// Spacing between the children of the box
    pub spacing: i32,
    /// Orientation of the children of the box
    pub orientation: Orientation,
    /// Children of the box component
    pub children: Vec<Component>
}

impl Default for BoxComponent {
    fn default() -> Self {
        Self {
            spacing: 12,
            orientation: Orientation::Vertical,
            children: Vec::new(),
            classes: vec![String::from("box")],
            halign: Align::Center,
            valign: Align::Center,
            hexpand: true,
            vexpand: true,
            width: 500
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum Align {
    Fill,
    Start,
    End,
    Center,
    Baseline
}

impl From<Align> for gtk::Align {
    fn from(value: Align) -> Self {
        match value {
            Align::Fill => gtk::Align::Fill,
            Align::Start => gtk::Align::Start,
            Align::End => gtk::Align::End,
            Align::Center => gtk::Align::Center,
            Align::Baseline => gtk::Align::Baseline,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum Orientation {
    Horizontal,
    Vertical
}

impl From<Orientation> for gtk::Orientation {
    fn from(value: Orientation) -> Self {
        match value {
            Orientation::Horizontal => gtk::Orientation::Horizontal,
            Orientation::Vertical => gtk::Orientation::Vertical,
        }
    }
}