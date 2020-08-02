use std::sync::{Arc, Mutex};

use clap::Clap;
use crossbeam_channel::Sender;

use crate::opts;

pub struct AppStruct {
    settings: String,
    opts: opts::Opts,
    subscribers: Vec<Sender<String>>,
}

pub type App = Arc<Mutex<AppStruct>>;

pub fn new() -> Arc<Mutex<AppStruct>> {
    let app = AppStruct {
        opts: opts::Opts::parse(),
        settings: r#"{"foo": "bar"}"#.into(),
        subscribers: Vec::new(),
    };

    Arc::new(Mutex::new(app))
}

impl AppStruct {
    pub fn opts(&self) -> opts::Opts {
        self.opts.clone()
    }

    pub fn get_settings(&self) -> String {
        self.settings.clone()
    }

    pub fn set_settings(&mut self, settings: String) {
        self.settings = settings;
    }

    pub fn subscribe(&mut self, sender: Sender<String>) {
        self.subscribers.push(sender);
    }
}
