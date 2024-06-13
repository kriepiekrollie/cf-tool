use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use serde::{ Serialize, Deserialize };
use crate::cli::{ TemplateArgs, ConfigArgs };
use std::path::{ Path, PathBuf };
use colored::Colorize;
use std::fmt::Debug;

// Maybe i move this out to a module called session.rs?
pub struct LoginDetails {
    pub username: String,
    pub password_hash: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Template {
    pub alias: String,
    pub lang: usize,
    pub path: String,
    pub suffix: Vec<String>,
    
    pub scripts: TemplateScripts,
}

#[derive(Default, Serialize, Deserialize)]
pub struct TemplateScripts {
    pub before: Option<String>,
    pub execute: Option<String>,
    pub after: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub templates: Vec<Template>,
    pub default: isize,
}

impl Config {
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),
            default: -1,
        }
    }
    // pub fn load() -> Result<Self, Box<dyn Error>> {
    // pub fn load_or_new() -> Result<Self, Box<dyn Error>> {
}

fn get_template(config: &Config, args: &TemplateArgs) -> Template {
    if let Some(alias) = &args.alias {
    } else if let Some(index) = &args.index {
    } else {
    }
    Template {
        alias: "sussy".to_string(),
        lang: 0,
        path: "amogus".to_string(),
        suffix: Vec::new(),
        scripts: TemplateScripts {
            before: None, execute: None, after: None,
        },
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

pub fn login(details: &LoginDetails) {
}
