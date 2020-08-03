use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pipeline: Option<Vec<Proc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Proc {
    Blur(i32),
    Pixelate(i32),
}

impl Settings {
    pub fn new(settings: &str) -> Result<Settings> {
        serde_json::from_str(settings)
    }

    pub fn to_string(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}
