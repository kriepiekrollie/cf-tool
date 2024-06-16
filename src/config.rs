use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use colored::Colorize;
use crate::cli::{TemplateArgs, ConfigArgs};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginDetails {
    pub handle: String,
    pub password: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Template {
    pub alias: String,
    pub lang: usize,
    pub path: PathBuf,
    pub suffix: Vec<String>,
    
    pub scripts: TemplateScripts,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct TemplateScripts {
    pub before: String,
    pub execute: String,
    pub after: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub templates: Vec<Template>,
    pub default: isize,
    pub cf_root: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),
            default: -1,
            cf_root: std::env::current_dir().unwrap().join("cf")
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
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
}

fn get_template(config: &Config, args: &TemplateArgs) -> Template {
    if let Some(alias) = &args.alias {
        config.templates.iter().find(|t| t.alias == *alias).unwrap().clone()
    } else if let Some(index) = &args.index {
        config.templates.get(*index as usize)
            .unwrap_or_else(|| panic!("Invalid index."))
            .clone()
    } else if config.default >= 0 {
        config.templates.get(config.default as usize).unwrap().clone()
    } else {
        // You don't have a template yet...
        Template::default()
    }
}

pub fn add_template(template: &Template) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn delete_template(args: &TemplateArgs) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn set_default_template(args: &TemplateArgs) -> Result<(), Box<dyn Error>> {
    Ok(())
}
