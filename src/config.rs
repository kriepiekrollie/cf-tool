use std::path::PathBuf;
use std::fs;
use std::io::BufReader;
use std::fmt::Debug;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use anyhow::{Context, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginDetails {
    pub handle_or_email: String,
    pub password: String,
    pub remember: bool,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct TemplateScripts {
    pub before: Option<String>,
    pub execute: String,
    pub after: Option<String>,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Template {
    pub lang_id: u8,
    pub path: PathBuf,
    pub suffix: Vec<String>,

    pub scripts: TemplateScripts,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub templates: HashMap<String, Template>,
    pub default: Option<String>,
    pub cf_root: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            default: None,
            cf_root: dirs::home_dir()
                .expect("Failed to get home directory.").join("cf")
        }
    }

    pub fn load(path: &PathBuf) -> Result<Self> {
        let file = fs::File::open(path).with_context(||
            format!("Failed to open file for reading: {:?}", path))?;
        let reader = BufReader::new(file);
        let config: Self = serde_json::from_reader(reader).with_context(||
            format!("Failed to read config to file: {:?}", path))?;
        Ok(config)
    }

    pub fn load_or_new(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            Self::load(path)
        } else {
            let config = Self::new();
            config.write(path).with_context(|| 
                format!("Failed to create config file: {:?}", path))?;
            Ok(config)
        }
    }

    pub fn write(&self, path: &PathBuf) -> Result<()> {
        fs::create_dir_all(&path).with_context(|| 
            format!("Failed to create config directory: {:?}", path))?;
        let file = fs::File::create(path).with_context(||
            format!("Failed to open file for writing: {:?}", path))?;
        serde_json::to_writer_pretty(file, self).with_context(||
            format!("Failed to write config to file: {:?}", path))?;
        Ok(())
    }
}
