pub mod settings;

use std::sync::{Arc, Mutex};
use std::{fs, path};

use settings::Settings;

pub struct AppStruct {
    settings: Settings,
}

pub type App = Arc<Mutex<AppStruct>>;

const LOCAL_SETTINGS: &str = "./config.json";
const SETTINGS: &str = "/home/naps62/.config/instacam/config.json";

pub fn new() -> Arc<Mutex<AppStruct>> {
    let app = AppStruct {
        settings: load_settings(),
    };

    Arc::new(Mutex::new(app))
}

impl AppStruct {
    pub fn get_settings(&self) -> Settings {
        self.settings.clone()
    }
}

fn load_settings() -> Settings {
    let json = read_file_if_exists(LOCAL_SETTINGS)
        .or_else(|_| read_file_if_exists(SETTINGS))
        .unwrap();

    Settings::new(json.as_str()).unwrap()
}

fn read_file_if_exists(file: &str) -> Result<String, ()> {
    if path::Path::new(file).exists() {
        fs::read_to_string(file).map_err(|_| ())
    } else {
        Err(())
    }
}
