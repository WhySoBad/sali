use components::build::{build_component_tree, Wrapped};
use gtk4 as gtk;
use login::handle_login;
use std::process;
use std::{fs, sync::Arc};

use clap::Parser;
use cli::Cli;
use config::{Classes, Config};
use gtk::gdk::{*, prelude::*};
use gtk::{*, prelude::*};
use gtk4_layer_shell::*;
use log::{error, info, warn};
use rsass::{compile_scss, output};

mod config;
mod cli;
mod components;
mod login;

// TODO: Maybe cache the compiled styles using a checksum to prevent it from compiling in every run
//       This could potentially mean a faster greeter 👀

const APP_ID: &str = "ch.wysbd.sali";

fn main() {
       env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
       let cli = Cli::parse();
       let config = Arc::new(Config::new(&cli.config));

      let app = Application::builder()
        .application_id(APP_ID)
        .build();

      let cloned_config = config.clone();
      app.connect_startup(move |_| {
          load_stylesheets(&cloned_config);
      });

      let cloned_config = config.clone();
      let cloned_app = app.clone();
      app.connect_activate(move |_| {
          cloned_config.monitors.iter().for_each(|(name, mon)| {
              build_background_window(&cloned_app, mon, cloned_config.clone());
              if *name == config.main_monitor {
                  build_form_window(&cloned_app, mon, cloned_config.clone());
              }
          })
      });

      app.run();
}

fn load_stylesheets(config: &Arc<Config>) {
    let provider = CssProvider::new();
    let format = output::Format {
        style: output::Style::Expanded,
        ..Default::default()
    };

    config.styles.iter().for_each(|path_str| {
        let path = &config.resolve_path(path_str);
        if path.exists() {
            match fs::read(path) {
                Ok(content) => {
                    let css = if path.extension().is_some_and(|ext| ext == "scss") {
                        match compile_scss(content.as_slice(), format) {
                            Ok(css) => css,
                            Err(err) => {
                                error!("unable to compile stylesheet {path_str}: {err}");
                                Vec::new()
                            }
                        }
                    } else {
                        content
                    };
                    let str = std::str::from_utf8(css.as_slice()).expect("should be valid utf-8");
                    provider.load_from_data(str);
                },
                Err(err) => error!("unable to read stylesheet from {path_str}: {err}")
            }
        } else {
            warn!("style path {path_str} does not exist");
        }
    });

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("should have display"),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION
    )
}

fn get_gdk_monitor(monitor: &config::Monitor) -> Option<gtk::gdk::Monitor> {
    let display = Display::default().expect("should have display");
    (0..display.monitors().n_items())
        .filter_map(|i| {
            let obj = display.monitors().item(i)?;
            obj.downcast::<gtk::gdk::Monitor>().ok()
        })
        .find(|m| m.connector().unwrap_or_default() == monitor.output)
}

fn build_background_window(app: &Application, monitor: &config::Monitor, config: Arc<Config>) {
    let Some(background) = &monitor.background else {
        return;
    };

    let Some(gdk_monitor) = get_gdk_monitor(monitor) else {
        warn!("found no monitor with output {} on default display to build background window", monitor.output);
        return;
    };

    let provider = CssProvider::new();
    let geometry = gdk_monitor.geometry();
    let window = ApplicationWindow::builder()
        .application(app)
        .css_classes(config.classes.background.clone())
        .destroy_with_parent(true)
        .default_width(geometry.width())
        .default_height(geometry.height())
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Background);
    window.set_monitor(&gdk_monitor);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Left, true);
    window.set_exclusive_zone(-1);

    let display = Display::default().expect("should have display");
    let class_name = format!("{}-{}", APP_ID.replace(".", "-"), monitor.output);
    let class_content = match background {
        config::MonitorBackground::Rgb(r, g, b) => format!("background-color: rgb({r},{g},{b})"),
        config::MonitorBackground::Image(path) => format!("background: url(\"file://{}\")", config.resolve_path(path).to_string_lossy()),
    };
    let css_str = format!(r".{class_name} {{ {class_content}; background-size: cover; background-position: center; }}");
    provider.load_from_data(&css_str);
    window.add_css_class(&class_name);

    gtk::style_context_add_provider_for_display(
        &display,
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION
    );

    window.present();
}

fn build_form_window(app: &Application, monitor: &config::Monitor, config: Arc<Config>) {
    let Some(gdk_monitor) = get_gdk_monitor(monitor) else {
        warn!("found no monitor with output {} on default display to build form window", monitor.output);
        return;
    };

    let geometry = gdk_monitor.geometry();
    let window = ApplicationWindow::builder()
        .application(app)
        .css_classes(config.classes.window.clone())
        .destroy_with_parent(true)
        .fullscreened(true)
        .focusable(true)
        .decorated(false)
        .hexpand(true)
        .vexpand(true)
        .resizable(false)
        .width_request(geometry.width())
        .height_request(geometry.height())
        .build();

    window.init_layer_shell();
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Top, true);
    window.set_exclusive_zone(-1);
    window.set_layer(Layer::Overlay);
    window.set_monitor(&gdk_monitor);
    window.set_keyboard_mode(KeyboardMode::OnDemand);

    let (mut username, mut password, mut runner) = (None, None, None);

    let tree = build_component_tree(
        config.layout.clone(),
        &mut username,
        &mut password,
        &mut runner,
        &config
    );

    let Some(password) = password else {
        error!("no password component is specified");
        std::process::exit(1);
    };

    if runner.is_none() && config.default_runner.is_none() {
        error!("neither a runner component nor a default runner is specified");
        std::process::exit(1);
    } else if username.is_none() && config.username.is_none() {
        error!("neither a username component nor a default username is specified");
        std::process::exit(1)
    }

    let add_empty_class = move |entry: &Entry, classes: &Classes| {
        if entry.text().is_empty() {
            entry.add_css_class(&classes.field_empty)
        } else {
            entry.remove_css_class(&classes.field_empty)
        }
    };

    let (cu, cr, cp, cc) = (username.clone(), runner.clone(), password.clone(), config.clone());
    let tmp = password.as_ref().borrow();
    let entry = tmp.downcast_ref::<Entry>().expect("should be entry");
    entry.connect_text_notify(move |entry| add_empty_class(entry, &cc.classes));
    let cc = config.clone();
    entry.connect_activate(move |_| {
        handle_submit(cu.clone(), cp.clone(), cr.clone(), cc.clone());
    });

    let (cu, cr, cp, cc) = (username.clone(), runner.clone(), password.clone(), config.clone());
    if let Some(usr) = username.clone() {
        let tmp = usr.as_ref().borrow();
        let entry = tmp.downcast_ref::<Entry>().expect("should be entry");
        entry.connect_text_notify(move |entry| add_empty_class(entry, &cc.classes));
        let cc = config.clone();
        entry.connect_activate(move |_| {
            handle_submit(cu.clone(), cp.clone(), cr.clone(), cc.clone());
        });
    }

    match tree {
        Some(child) => {
            let widget = child.as_ref().borrow();
            window.set_child(Some(widget.as_ref() as &Widget));
            window.present();
            info!("opened login form");
        },
        None => {
            error!("component tree is empty which makes login impossible");
            process::exit(1);
        }
    }
}

fn handle_submit(username: Option<Wrapped<Widget>>, password: Wrapped<Widget>, runner: Option<Wrapped<Widget>>, config: Arc<Config>) {
    let runner_opt = if let Some(runner) = runner {
        let tmp = runner.as_ref().borrow();
        let entry = tmp.downcast_ref::<DropDown>().expect("should be dropdown");

        entry.selected_item().and_downcast::<StringObject>().and_then(|selected| {
            config.runners.values().find(|r| r.display_name == selected.string())
        })
    } else {
        let name = config.default_runner.clone().expect("should have default runner");
        config.runners.get(&name)
    };

    let Some(runner) = runner_opt else {
        warn!("no runner found for submission");
        return;
    };

    let tmp = password.as_ref().borrow();
    let password_entry = tmp.clone().downcast::<Entry>().expect("should be entry");
    let username_entry = username.map(|usr| {
        let tmp = usr.as_ref().borrow();
        tmp.clone().downcast::<Entry>().expect("should be entry")
    });

    let password_str = password_entry.text().to_string();
    let username_str = match &username_entry {
        Some(entry) => entry.text().to_string(),
        None => config.username.clone().expect("should have default username")
    };

    match handle_login(username_str, password_str, runner) {
        login::LoginResult::Failure(failure) => {
            match failure {
                login::LoginFailure::MissingFields => {
                    if password_entry.text().is_empty() {
                        password_entry.add_css_class(&config.classes.field_error)
                    } else if let Some(entry) = &username_entry {
                        entry.add_css_class(&config.classes.field_error)
                    }
                },
                login::LoginFailure::AuthError |
                login::LoginFailure::Error => {
                    password_entry.add_css_class(&config.classes.field_error);
                    if let Some(entry) = &username_entry { entry.add_css_class(&config.classes.field_error) };
                },
            }
        },
        login::LoginResult::Success => {
            info!("login attempt succeeded");
            std::process::exit(0);
        },
    }
}