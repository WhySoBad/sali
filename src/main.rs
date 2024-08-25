use gtk4::{self as gtk, prelude::Cast};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use std::{fs, path::Path, sync::Arc};

use clap::Parser;
use cli::Cli;
use config::Config;
use gtk::{gdk::Display, prelude::{ApplicationExt, ApplicationExtManual, DisplayExt, GtkWindowExt, ListModelExt, MonitorExt, WidgetExt}, Application, ApplicationWindow, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
use log::{error, warn};
use rsass::{compile_scss, output};

mod config;
mod cli;
mod components;

// TODO: Maybe cache the compiled styles using a checksum to prevent it from compiling in every run
//       This could potentially mean a faster greeter 👀
//
// TODO: Create custom css strings containing the backgrounds for the monitors and add them to the application
//       => DON'T USE SCSS FOR THIS!

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
              build_background_window(&cloned_app, mon);
              if *name == config.main_monitor {
                  build_form_window(&cloned_app, mon);
              }
          })
      });

      app.run();
}

fn load_stylesheets(config: &Arc<Config>) {
    let provider = CssProvider::new();
    let format = output::Format {
        style: output::Style::Compressed,
        ..Default::default()
    };

    config.styles.iter().for_each(|path_str| {
        let path = Path::new(path_str);
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

fn build_background_window(app: &Application, monitor: &config::Monitor) {
    let display = Display::default().expect("should have display");
    let gdk_monitor = (0..display.monitors().n_items())
        .filter_map(|i| {
            let obj = display.monitors().item(i)?;
            obj.downcast::<gtk::gdk::Monitor>().ok()
        })
        .find(|m| m.connector().unwrap_or_default() == monitor.output);

    let Some(gdk_monitor) = gdk_monitor else {
        warn!("found no monitor with output {} on default display", monitor.output);
        return;
    };

    let provider = CssProvider::new();
    let geometry = gdk_monitor.geometry();
    let window = ApplicationWindow::builder()
        .application(app)
        .css_classes(vec![String::from("background"), String::from("window")])
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

    if let Some(background) = &monitor.background {
        let class_name = format!("{}-{}", APP_ID.replace(".", "-"), monitor.output);
        let class_content = match background {
            config::MonitorBackground::Rgb(r, g, b) => format!("background-color: rgb({r},{g},{b})"),
            config::MonitorBackground::Image(path) => format!("background: url(\"file://{path}\")"),
        };
        provider.load_from_data(format!(r".{class_name} {{ {class_content}; background-size: cover; background-position: center; }}").as_str());
        window.add_css_class(&class_name);

        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION
        )
    }

    window.present();
}

fn build_form_window(app: &Application, monitor: &config::Monitor) {
    // FIXME: Find display for monitor (e.g. for DP-1)
    let window = ApplicationWindow::builder()
        .application(app)
        .css_classes(vec![String::from("window")])
        .destroy_with_parent(true)
        .fullscreened(true)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Background);

    // TODO

    window.present();
}