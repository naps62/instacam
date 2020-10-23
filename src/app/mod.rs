pub mod settings;

use std::sync::{Arc, Mutex};

use clap::Clap;

use crate::args::Args;
use settings::Settings;

pub struct AppStruct {
    settings: Settings,
    args: Args,
}

pub type App = Arc<Mutex<AppStruct>>;

const SETTINGS: &str = "/home/naps62/.config/instacam/config.json";

pub fn new() -> Arc<Mutex<AppStruct>> {
    let app = AppStruct {
        args: Args::parse(),
        settings: load_settings(SETTINGS),
    };

    Arc::new(Mutex::new(app))
}

impl AppStruct {
    pub fn args(&self) -> Args {
        self.args.clone()
    }

    pub fn get_settings(&self) -> Settings {
        self.settings.clone()
    }
}

fn load_settings(file: &str) -> Settings {
    let json = match std::fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(_) => String::from("{}"),
    };

    Settings::new(json.as_str()).unwrap()
}
