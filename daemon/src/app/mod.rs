pub mod settings;

use std::sync::{Arc, Mutex};

use clap::Clap;
use crossbeam_channel::Sender;

use crate::opts;
use settings::Settings;

pub struct AppStruct {
    settings: Settings,
    opts: opts::Opts,
    #[allow(dead_code)]
    subscribers: Vec<Sender<String>>,
}

pub type App = Arc<Mutex<AppStruct>>;

const SETTINGS: &str = "/home/naps62/.config/instacam/config.json";

pub fn new() -> Arc<Mutex<AppStruct>> {
    let app = AppStruct {
        opts: opts::Opts::parse(),
        settings: load_settings(SETTINGS),
        subscribers: Vec::new(),
    };

    println!("{:?}", app.settings);
    Arc::new(Mutex::new(app))
}

impl AppStruct {
    pub fn opts(&self) -> opts::Opts {
        self.opts.clone()
    }

    pub fn get_settings(&self) -> Settings {
        self.settings.clone()
    }

    pub fn set_settings(&mut self, settings: String) -> Result<(), serde_json::Error> {
        self.settings = Settings::new(settings.as_str())?;

        save_settings(&self.settings, SETTINGS);

        Ok(())
    }

    pub fn subscribe(&mut self, sender: Sender<String>) {
        self.subscribers.push(sender);
    }
}

fn load_settings(file: &str) -> Settings {
    let json = match std::fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(_) => String::from("{}"),
    };
    println!("{:?}", json);

    Settings::new(json.as_str()).unwrap()
}

fn save_settings(settings: &Settings, file: &str) {
    let json = settings.to_string().unwrap();

    std::fs::write(file, json.as_str()).unwrap();
}