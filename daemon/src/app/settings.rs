use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Settings {
    pub pipeline: Option<Vec<Proc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "name", rename_all = "lowercase")]
pub enum Proc {
    Blur { k: i32 },
    Pixelate { k: i32 },
    Sepia,
    Edges { t1: f64, t2: f64 },
}

impl Settings {
    pub fn new(settings: &str) -> Result<Settings> {
        serde_json::from_str(settings)
    }

    pub fn to_string(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_empty() {
        let s = Settings::new(r#"{}"#).unwrap();

        assert!(s.pipeline.is_none());
    }

    #[test]
    fn decodes_empty_list() {
        let s = Settings::new(r#"{"pipeline": []}"#).unwrap();

        assert_eq!(s.pipeline.unwrap().len(), 0);
    }

    #[test]
    fn decodes_blur() {
        let json = r#"
        {"pipeline": [
            {"name": "blur", "k": 10}
        ]}
        "#;

        let s = Settings::new(json).unwrap();

        assert_eq!(s.pipeline.unwrap().get(0).unwrap(), &Proc::Blur { k: 10 });
    }

    #[test]
    fn decodes_pixelate() {
        let json = r#"
        {"pipeline": [
            {"name": "pixelate", "k": 20}
        ]}
        "#;

        let s = Settings::new(json).unwrap();

        assert_eq!(
            s.pipeline.unwrap().get(0).unwrap(),
            &Proc::Pixelate { k: 20 }
        );
    }

    #[test]
    fn decodes_sepia() {
        let json = r#"
        {"pipeline": [
            {"name": "sepia"}
        ]}
        "#;

        let s = Settings::new(json).unwrap();

        assert_eq!(s.pipeline.unwrap().get(0).unwrap(), &Proc::Sepia);
    }
}
