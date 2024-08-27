use std::{collections::BTreeMap, fs, path::Path};
use serde::Deserialize;
use log::error;

use crate::components::{
    BoxComponent,
    Component,
    DateTimeComponent,
    PasswordComponent,
    RunnerComponent,
    UsernameComponent
};

#[derive(Deserialize, Debug)]
pub struct Config {
    /// Named configuration for each monitor
    pub monitors: BTreeMap<String, Monitor>,
    /// Name of the monitor which should contain the login form
    pub main_monitor: String,
    /// The default username to prefill the username field if provided
    #[serde(default)]
    pub username: Option<String>,
    /// All paths from where stylesheets should be loaded
    #[serde(default)]
    pub styles: Vec<String>,
    /// Css classes which are applied to different nodes and on specific events
    #[serde(default)]
    pub classes: Classes,
    /// Named runner options
    pub runners: BTreeMap<String, Runner>,
    /// Optional name of a runner which is preselected
    #[serde(default)]
    pub default_runner: Option<String>,
    /// Layout of the main monitor
    #[serde(default = "default_layout")]
    pub layout: Component,
}

impl Config {
    pub fn new(path_str: &String) -> Self {
        let path = Path::new(path_str);
        if path.exists() {
            let str = fs::read_to_string(path).unwrap_or_default();
            match serde_yaml::from_str(str.as_str()) {
                Ok(config) => config,
                Err(err) => {
                    error!("invalid config file: {err}");
                    std::process::exit(1)
                }
            }
        } else {
            error!("missing config file at {}", path.to_str().unwrap_or_default());
            std::process::exit(1)
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            monitors: BTreeMap::new(),
            main_monitor: String::new(),
            username: None,
            styles: Vec::new(),
            runners: BTreeMap::new(),
            default_runner: None,
            classes: Classes::default(),
            layout: default_layout()
        }
    }
}

fn default_layout() -> Component {
    Component::Box(BoxComponent {
        children: vec![
            Component::DateTime(DateTimeComponent::default()),
            Component::Username(UsernameComponent::default()),
            Component::Password(PasswordComponent::default()),
            Component::Runner(RunnerComponent::default())
        ],
        ..Default::default()
    })
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum MonitorBackground {
    /// Rgb value of a color
    Rgb(u8, u8, u8),
    /// Path to the background image
    Image(String)
}

#[derive(Deserialize, Debug)]
pub struct Monitor {
    /// Hardware output of the monitor (e.g. DP-1)
    pub output: String,
    /// Path to the background image of the monitor
    pub background: Option<MonitorBackground>
}

#[derive(Deserialize, Debug)]
pub struct Runner {
    /// Name which should be displayed when the runner is selected
    pub display_name: String,
    /// Commands to run when the login succeeds
    pub run: Vec<String>,
    /// Environment variables to set when the login succeeds
    #[serde(default)]
    pub env: Vec<String>
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Classes {
    /// Css classes which are applied to the background windows
    pub background: Vec<String>,
    /// Css classes which are applied to the main window holding the layout tree
    pub window: Vec<String>,
    /// Css class which is applied to the username/password fields when a login error occurs
    pub field_error: String,
    /// Css class which is applied to the username/password fields when they are empty
    pub field_empty: String,
}

impl Default for Classes {
    fn default() -> Self {
        Self {
            background: vec![String::from("background")],
            window: vec![String::from("window")],
            field_error: String::from("error"),
            field_empty: String::from("empty"),
        }
    }
}