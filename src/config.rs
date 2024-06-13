use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};
use crate::template::Template;
use crate::cli::config::ConfigArgs;
use std::path::{Path, PathBuf};
use colored::Colorize;
use std::fmt::Debug;

#[derive(Serialize, Deserialize)]
pub struct FolderNames {
    pub acmsguru: String,
    pub contest: String,
    pub group: String,
    pub gym: String,
    pub root: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub templates: Vec<Template>,
    pub default: isize,

    pub gen_after_parse: bool,
    pub host: String,
    pub proxy: Option<String>,
    pub folder_name: FolderNames,
}

impl Config {
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),
            default: -1,
            gen_after_parse: false,
            host: "https://codeforces.com".to_string(),
            proxy: None,
            folder_name: FolderNames {
                acmsguru: "acmsguru".to_string(),
                contest: "contest".to_string(),
                group: "group".to_string(),
                gym: "gym".to_string(),
                root: "cf".to_string(),
            },
        }
    }
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let path = dirs::config_dir().unwrap().join("cf_tool").join("config.json");
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
    pub fn load_or_new() -> Self {
        Self::load().unwrap_or_else(|_| Self::new())
    }
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = dirs::config_dir().unwrap().join("cf_tool").join("config.json");
        let parent = path.parent().unwrap();
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }
    pub fn get_default_template(&self) -> Option<&Template> {
        if self.default < 0 {
            None
        } else {
            self.templates.get(self.default as usize)
        }
    }
    pub fn get_template(&self, alias: &str) -> Option<&Template> {
        self.templates.iter().find(|t| t.alias == alias)
    }
    pub fn get_template_from_suffix(&self, suffix: &str) -> Option<&Template> {
        self.templates.iter().find(|t| t.suffix.contains(&suffix.to_string()))
    }
    pub fn get_folder_name(&self, folder: &str) -> &str {
        match folder {
            "acmsguru" => &self.folder_name.acmsguru,
            "contest" => &self.folder_name.contest,
            "group" => &self.folder_name.group,
            "gym" => &self.folder_name.gym,
            "root" => &self.folder_name.root,
            _ => panic!("Invalid folder name: {}", folder),
        }
    }
    pub fn set_folder_name(&mut self, folder: &str, name: &str) {
        match folder {
            "acmsguru" => self.folder_name.acmsguru = name.to_string(),
            "contest" => self.folder_name.contest = name.to_string(),
            "group" => self.folder_name.group = name.to_string(),
            "gym" => self.folder_name.gym = name.to_string(),
            "root" => self.folder_name.root = name.to_string(),
            _ => panic!("Invalid folder name: {}", folder),
        }
    }
    pub fn delete_template(&mut self, index: usize) {
        self.templates.remove(index);
        if self.default as usize == index {
            self.default = -1;
        }
    }
}

fn prompt_until_valid<T: std::str::FromStr + Debug, F: Fn(&T) -> bool>(name: &str, valid: F) -> T {
    let mut user_input = rprompt::prompt_reply(format!("{}: ", name)).unwrap();
    let mut index = user_input.parse::<T>();
    loop {
        if let Ok(result) = index {
            if valid(&result) {
                return result;
            }
        }
        user_input = rprompt::prompt_reply(format!("invalid {}. please try again: ", name)).unwrap();
        index = user_input.parse::<T>();
    }
}

fn print_templates(config: &Config) {
    println!("{}", "You have the following templates:".blue().bold());
    for (i, template) in config.templates.iter().enumerate() {
        if i as isize == config.default {
            println!("{}{} {}", format!("{}", i).green().bold(), ".".green().bold(), template.alias.green().bold());
        } else {
            println!("{}. {}", i, template.alias);
        }
    }
}

pub fn configure(args: &ConfigArgs) {
    // println!("{}", "Configure cf_tool.".blue().bold());
    // println!("{} Login", "0.".bold());
    // println!("{} Add template", "1.".bold());
    // println!("{} Delete template", "2.".bold());
    // println!("{} Set default template", "3.".bold());
    // println!("{} Generate after parse", "4.".bold());
    // println!("{} Set host", "5.".bold());
    // println!("{} Set proxy", "6.".bold());
    // println!("{} Set folder names", "7.".bold());

    // println!("{}", "Please choose one of the options above.".blue().bold());
    // let index = prompt_until_valid("index", |x: &isize| 0 <= *x && *x <= 7);
    // println!("");

    // let changes_made = match index {
    //     0 => {
    //         configure_login();
    //     }
    //     1 => {
    //         configure_add_template();
    //     }
    //     2 => {
    //         configure_delete_template();
    //     }
    //     3 => {
    //         configure_set_default_template();
    //     }
    //     4 => {
    //         configure_generate_after_parse();
    //     }
    //     5 => {
    //         configure_set_host();
    //     }
    //     6 => {
    //         configure_set_proxy();
    //     }
    //     7 => {
    //         configure_set_folder_names();
    //     }
    //     _ => {
    //         panic!("Invalid index: {}", index);
    //     }
    // };

    // println!("{}", serde_json::to_string_pretty(&config).unwrap());
}

fn configure_login() {
    println!("{}", "Enter your login details.".blue().bold());
    let username = rprompt::prompt_reply("handle/email: ").unwrap();
    let password = rpassword::prompt_password("password: ").unwrap();
}

fn configure_add_template() -> bool {
    let mut config = Config::load_or_new();

    println!("{}", "List of available languages:".blue().bold());
    println!(" {} GNU G++20 11.2.0", " 54.".bold());
    println!(" {} Python 3.8.10", "101.".bold());
    println!(" {} Java 17 64bit", "93.".bold());
    println!(" {} Kotlin 1.7.20", "80.".bold());
    println!("{}", "Choose a language for your new template.".blue().bold());
    let lang = prompt_until_valid("lang", |x: &usize| 0 <= *x && *x <= 100);
    println!("I CHOSE C++");
    println!("");

    println!("Alias:");
    println!("  The alias is what you will use to refer to your template in commands like");
    println!("  {}", "cf gen <alias>".bold());
    println!("{}", "Choose an alias for your new template.".blue().bold());
    let alias = prompt_until_valid("alias", |x: &String| x.len() > 0 && x.chars().all(char::is_alphanumeric));
    let alias = rprompt::prompt_reply("alias: ").unwrap();
    println!();

    if (config.templates.iter().find(|t| t.alias == alias)).is_some() {
        println!("{} {}", "Template already exists: ".bold().blue(), alias);
        return false;
    }
    println!();

    println!("Template file:");
    println!("  You can insert placeholders in your template code. When the file is generated,");
    println!("  these placeholders will be replaced with the appropriate values.");
    println!("");
    println!("  $%U%$   Handle (e.g. kriepiekrollie)");
    println!("  $%Y%$   Year   (e.g. 2024)");
    println!("  $%M%$   Month  (e.g. 03)");
    println!("  $%D%$   Day    (e.g. 14)");
    println!("  $%h%$   Hour   (e.g. 08)");
    println!("  $%m%$   Minute (e.g. 05)");
    println!("  $%s%$   Second (e.g. 00)");
    println!("");

    println!("{}", "Provide absolute path of your new template's file.".blue().bold());
    let path = prompt_until_valid("filepath", |x: &String| {
        let p = PathBuf::from(x);
        p.exists() && p.is_file()
    });
    println!();

    // Check if file exists
    let p = Path::new(&path);
    if !p.exists() {
        println!("File does not exist: {}", p.display());
        return false;
    }

    println!("{}", "Choose suffixes to associate with this template. (space-separated)".blue().bold());
    let suffix = rprompt::prompt_reply("suffixes: ").unwrap().split_whitespace().map(|s| s.to_string()).collect();
    println!();

    println!("{}", "Script".blue().bold());
    // Describe format of scripts.
    
    println!("{}", "Provide a script to run before your code is executed.".blue().bold());
    let before_script = rprompt::prompt_reply("before: ").ok();
    println!();

    println!("{}", "Provide a script to execute your code.".blue().bold());
    let script = rprompt::prompt_reply("script: ").ok();
    println!();

    println!("{}", "Provide a script to run after your code has been executed.".blue().bold());
    let after_script = rprompt::prompt_reply("after: ").ok();
    println!();

    let template = Template {
        alias,
        lang,
        path,
        suffix,
        before_script,
        script,
        after_script,
    };

    config.templates.push(template);
    return true;
}

fn configure_delete_template() {

    let mut config = Config::load_or_new();

    // If there are no templates, inform user.
    if config.templates.is_empty() {
        println!("{}", "You don't have any templates to delete.".blue().bold());
        return;
    }

    print_templates(&config);
    println!("{}", "Choose a template to delete.".blue().bold());
    let index = prompt_until_valid("index", |x: &usize| 0 <= *x && *x < config.templates.len());
    println!("");

    config.delete_template(index);
    config.save();

    println!("{}", "Template successfully deleted.");
}

fn configure_set_default_template() {
    let mut config = Config::load_or_new();

    print_templates(&config);
    println!("{}", "Choose a new default template.".blue().bold());
    let index = prompt_until_valid("index", |x: &isize| 0 <= *x && (*x as usize) < config.templates.len());
    println!("");

    config.default = index;
    config.save();
    println!("New default successfully saved.");
}

fn configure_generate_after_parse() {
    let mut config = Config::load_or_new();
    let gen_after_parse = rprompt::prompt_reply("gen_after_parse: ").unwrap().parse::<bool>().unwrap();
    config.gen_after_parse = gen_after_parse;
}

fn configure_set_host() {
    let mut config = Config::load_or_new();
    let host = rprompt::prompt_reply("host: ").unwrap();
    config.host = host;
}

fn configure_set_proxy() {
    let mut config = Config::load_or_new();
    let proxy = rprompt::prompt_reply("proxy: ").ok();
    config.proxy = proxy;
}

fn configure_set_folder_names() {
    let mut config = Config::load_or_new();
    println!("{} acmsguru", "0.".bold());
    println!("{} contest", "1.".bold());
    println!("{} group", "2.".bold());
    println!("{} gym", "3.".bold());
    println!("{} root", "4.".bold());
    println!("");

    let folder = rprompt::prompt_reply("folder: ").unwrap();
    let name = rprompt::prompt_reply("name: ").unwrap();
    config.set_folder_name(&folder, &name);
}


