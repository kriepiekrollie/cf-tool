use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use colored::Colorize;
use crate::cli::TemplateArgs;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginDetails {
    pub handle: String,
    pub password: String,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct Template {
    pub lang: usize,
    pub path: PathBuf,
    pub suffix: Vec<String>,
    
    pub scripts: TemplateScripts,
}

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
pub struct TemplateScripts {
    pub before: String,
    pub execute: String,
    pub after: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub templates: HashMap<String, Template>,
    pub default: isize,
    pub cf_root: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Self {
            templates: HashMap::new(),
            default: -1,
            cf_root: std::env::home_dir().unwrap().join("cf")
        }
    }
    pub fn load(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config: Self = serde_json::from_reader(reader)?;
        Ok(config)
    }
    pub fn load_or_new(path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        if path.exists() {
            Self::load(path)
        } else {
            let config = Self::new();
            config.save(path)?;
            Ok(config)
        }
    }
    pub fn save(&self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
    pub fn add_template(&mut self, template: Template) {
        // self.templates.push(template);
    }
    pub fn delete_template(&mut self, alias: &String) {
        // self.templates.retain(|t| t.alias != alias);
    }
}
