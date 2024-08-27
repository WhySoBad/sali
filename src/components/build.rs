use std::{cell::RefCell, rc::Rc, sync::Arc, time::Duration};

use chrono::Local;
use glib::{timeout_add_local, ControlFlow};
use gtk4 as gtk;
use gtk::{*, prelude::*};
use log::warn;

use crate::config::Config;

use super::Component;

pub type Wrapped<T> = Rc<RefCell<T>>;

pub fn build_component_tree(
    component: Component,
    username: &mut Option<Wrapped<Widget>>,
    password: &mut Option<Wrapped<Widget>>,
    runner: &mut Option<Wrapped<Widget>>,
    config: &Arc<Config>
) -> Option<Wrapped<Widget>> {
    match component {
        super::Component::Username(field) => {
            match username {
                Some(_) => {
                    warn!("received other username node, ignoring lower level node");
                    None
                },
                None => {
                    let entry = build_username_field(field, config.username.clone());
                    let widget = Rc::new(RefCell::new(entry.upcast::<Widget>()));
                    *username = Some(widget.clone());
                    return Some(widget)
                }
            }
        },
        super::Component::Password(field) => {
            match password {
                Some(_) => {
                    warn!("received other password node, ignoring lower level node");
                    None
                },
                None => {
                    let entry = Rc::new(RefCell::new(build_password_field(field).upcast::<Widget>()));
                    *password = Some(entry.clone());
                    return Some(entry)
                }
            }
        },
        super::Component::Runner(field) => {
            match runner {
                Some(_) => {
                    warn!("received other runner node, ignoring lower level node");
                    None
                },
                None => {
                    let options = config.runners.values().map(|r| r.display_name.as_str()).collect::<Vec<_>>();
                    let dropdown = build_runner_field(field, options, config.default_runner.clone());
                    let widget = Rc::new(RefCell::new(dropdown.upcast::<Widget>()));
                    *runner = Some(widget.clone());
                    return Some(widget)
                }
            }
        },
        super::Component::Box(bx) => {
            let children = bx.children.clone().into_iter()
                .filter_map(|c| build_component_tree(c, username, password, runner, config))
                .collect::<Vec<_>>();

            let built = build_box(bx);
            children.iter().for_each(|child| {
                let widget = child.as_ref().borrow();
                built.append(widget.as_ref() as &Widget)
            });

            Some(Rc::new(RefCell::new(built.upcast::<Widget>())))
        },
        super::Component::DateTime(datetime) => {
            let widget = build_datetime(datetime).upcast::<Widget>();
            Some(Rc::new(RefCell::new(widget)))
        },
        super::Component::Label(label) => {
            let widget = build_label(label).upcast::<Widget>();
            Some(Rc::new(RefCell::new(widget)))
        },
    }
}

fn build_username_field(field: super::UsernameComponent, default_username: Option<String>) -> Entry {
    Entry::builder()
        .css_classes(field.classes)
        .placeholder_text(field.placeholder)
        .text(default_username.unwrap_or_default())
        .build()
}

fn build_password_field(field: super::PasswordComponent) -> Entry {
    Entry::builder()
        .css_classes(field.classes)
        .placeholder_text(field.placeholder)
        .visibility(false)
        .build()
}

fn build_runner_field(field: super::RunnerComponent, runners: Vec<&str>, default_runner: Option<String>) -> DropDown {
    let model = StringList::new(runners.as_slice());
    let mut selected = 0;
    if let Some(default_runner) = &default_runner {
        if let Some((index, _)) = runners.iter().enumerate().find(|(_, &val)| val == default_runner) {
            selected = index as u32;
        }
    }

    DropDown::builder()
        .css_classes(field.classes)
        .selected(selected)
        .model(&model)
        .selected(selected)
        .build()
}

fn build_box(bx: super::BoxComponent) -> Box {
    Box::builder()
        .css_classes(bx.classes)
        .orientation(bx.orientation.into())
        .halign(bx.halign.into())
        .valign(bx.valign.into())
        .hexpand(bx.hexpand)
        .vexpand(bx.vexpand)
        .width_request(bx.width)
        .spacing(bx.spacing)
        .build()
}

fn build_datetime(datetime: super::DateTimeComponent) -> Label {
    let current = format!("{}", Local::now().format(datetime.format.as_str()));
    let label = Label::builder()
        .css_classes(datetime.classes)
        .label(current)
        .build();

    let cloned_label = label.clone();
    timeout_add_local(Duration::from_millis(datetime.interval), move || {
        let formatted = format!("{}", Local::now().format(datetime.format.as_str()));
        cloned_label.set_label(formatted.as_str());
        ControlFlow::Continue
    });
    label
}

fn build_label(label: super::LabelComponent) -> Label {
    Label::builder()
        .css_classes(label.classes)
        .label(label.label)
        .build()
}