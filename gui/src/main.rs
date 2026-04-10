mod application;
mod config;
mod objects;
mod oui;
mod scan_file;
mod vendor_cache;
mod widgets;
mod window;

use application::KawaiiFiApplication;
use gtk::prelude::*;
use gtk::{gio, glib};

fn main() -> glib::ExitCode {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // Load resources from installed location
    let res = gio::Resource::load(config::resources_file()).expect("Could not load gresource file");
    gio::resources_register(&res);

    // Create the application
    let app = KawaiiFiApplication::new(config::app_id());

    // Run the application
    app.run()
}
