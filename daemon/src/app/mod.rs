mod settings;

use std::sync::{Arc, Mutex};

use clap::Clap;
use crossbeam_channel::Sender;

use crate::opts;
use settings::Settings;

pub struct AppStruct {
    settings: Settings,
    opts: opts::Opts,
    subscribers: Vec<Sender<String>>,
}

pub type App = Arc<Mutex<AppStruct>>;

pub fn new() -> Arc<Mutex<AppStruct>> {
    let app = AppStruct {
        opts: opts::Opts::parse(),
        settings: Settings::new(r#"{"foo": "bar"}"#).unwrap(),
        subscribers: Vec::new(),
    };

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

        println!("{:?}", self.settings);

        Ok(())
    }

    pub fn subscribe(&mut self, sender: Sender<String>) {
        self.subscribers.push(sender);
    }
}
