use clap::{ Args, Parser, Subcommand };

/*
 * This module declares how the user can interact with the tool.
 * I am using the clap library and at some points rprompt and rpassword.
 */

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/*
 * Currently, cf_tool supports the following subcommands:
 *   config
 *   parse
 *   gen
 *   submit
 *   test
 *   source
 * 
 * Perhaps, maybe in future implement the stuff from original cftool?
 *   contest list
 *   problem list (with colors and symbols indicating submission results?)
 *   register
 *   open (shouldn't be too hard)
 */

#[derive(Subcommand)]
pub enum Commands {
    /// Change configuration
    Config(ConfigArgs),

    /// Fetch samples from contest
    Parse(ContestArgs),

    /// Generate file from template
    Gen(TemplateArgs),

    /// Submit code for problem
    Submit,

    /// Test code locally
    Test,

    /// Open the Github page of cf_tool
    Source,
}

/*
 * Declare how a user can specify which contest a command should use.
 */

#[derive(Args)]
pub struct ContestArgs {
    /// Specify a contest (default)
    #[arg(short, long, group = "contest_type")]
    contest: bool,

    /// Specify a gym
    #[arg(short, long, group = "contest_type")]
    gym: bool,

    /// The contest ID
    pub contest_id: String,
}
impl ContestArgs {
    pub fn get_contest_type(&self) -> ContestType {
        if self.gym {
            return ContestType::Gym;
        } else if self.contest {
            return ContestType::Contest;
        }
        ContestType::default()
    }
}

#[derive(Default,Clone)]
pub enum ContestType {
    #[default]
    Contest,
    Gym,
}
impl std::fmt::Display for ContestType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ContestType::Contest => write!(f, "contest"),
            ContestType::Gym => write!(f, "gym"),
        }
    }
}

/*
 * Declare how a user can specify which template a command should use.
 * This struct will be used in utils to fetch the template from config.
 */

#[derive(Args)]
pub struct TemplateArgs {
    /// Specify a template by its alias
    #[arg(short, long, group = "template_specifier")]
    pub alias: Option<String>,

    /// Specify a template by its index
    #[arg(short, long, group = "template_specifier")]
    pub index: Option<usize>,
}

/*
 * This module declares how the config subcommand works.
 * Unlike xalanq's cftool, I'm mostly using command-line arguments, 
 * but sometimes still stdin and stdout.
 */

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

/*
 * For now i'm only implementing the following config options:
 *
 *   login
 *   template add
 *            delete
 *            set-default
 *              
 * I might add some of the original options later:
 *
 *   gen-after-parse
 *   set-host
 *   set-proxy
 *   set-folder root
 *              contest
 *              gym
 *              group
 *              acmsguru
 * 
 * I never really found any of these useful, other than gen-after-parse
 * so I'll just enable it by default.
 *
 * I can see how host and proxy could be useful for some...
 *
 * I really don't get the point of set-folder though.
 * If you want your own directory structure just fork this project lol
 */

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Login
    Login,

    /// Configure templates
    Template(TemplateConfigArgs)
}

/* 
 * Declare commands relating to templates.
 */

#[derive(Args)]
pub struct TemplateConfigArgs {
    #[command(subcommand)]
    pub command: TemplateConfigCommands,
}

#[derive(Subcommand)]
pub enum TemplateConfigCommands {
    /// Add a template
    Add,

    /// Set a default template
    SetDefault(TemplateArgs),

    /// Delete a template
    Delete(TemplateArgs)
}

fn prompt_until_valid<T: std::str::FromStr + std::fmt::Debug, F: Fn(&T) -> bool>(name: &str, valid: F) -> T {
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

use crate::config::{ Template, TemplateScripts, LoginDetails };

pub fn prompt_new_template() -> Template {
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

pub fn prompt_login_details() -> LoginDetails {
    LoginDetails {
        username: "bruh".to_string(),
        password_hash: "amogus".to_string(),
    }
}
