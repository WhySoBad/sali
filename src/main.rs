use std::{fs, path::Path, sync::Arc};

use clap::Parser;
use cli::Cli;
use config::{Config, Monitor};
use gtk::{gdk::Display, prelude::{ApplicationExt, GtkWindowExt}, Application, ApplicationWindow, CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION};
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
      println!("Hello, world! {config:?}");

      let app = Application::builder()
        .application_id(APP_ID)
        .build();

      let cloned_config = config.clone();
      app.connect_startup(move |_| {
          load_stylesheets(&cloned_config);
          load_background_classes(&cloned_config)
      });
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
                    provider.load_from_data(css.as_slice());
                },
                Err(err) => error!("unable to read stylesheet from {path_str}: {err}")
            }
        } else {
            warn!("style path {path_str} does not exist");
        }
    });

    StyleContext::add_provider_for_display(
        &Display::default().expect("should have display"),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION
    )
}

fn load_background_classes(config: &Arc<Config>) {
    // TODO
}

fn build_monitor_window(app: &Application, monitor: &Monitor, main: bool) {
    // FIXME: Find display for monitor (e.g. for DP-1)
    let window = ApplicationWindow::builder()
        .application(app)
        .css_classes(vec![String::from("window")])
        .destroy_with_parent(true)
        .fullscreened(true)
        .build();

    // TODO

    window.present();
}