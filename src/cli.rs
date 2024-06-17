use clap::{Args, Parser, Subcommand};
use crate::cf::ContestType;

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
 * Currently, cf plans to support the following subcommands:
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
    /// Login
    Login,

    /// Configure templates
    Template(TemplateCommandArgs),

    /// Fetch samples from contest
    Parse(ContestArgs),

    /// Generate file from template
    Gen(TemplateArgs),

    /// Submit code for problem
    Submit,

    /// Test code locally
    Test,

    /// Open the Github page of cf-tool
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
    pub fn contest_type(&self) -> ContestType {
        if self.gym {
            return ContestType::Gym;
        } else if self.contest {
            return ContestType::Contest;
        }
        ContestType::default()
    }
}

/*
 * Declare how a user can specify which template a command should use.
 * This struct will be used in utils to fetch the template from config.
 */

#[derive(Args)]
pub struct TemplateArgs {
    /// Specify a template by its alias
    pub alias: Option<String>,
}

/*
 * This part declares how the config subcommand works.
 * Unlike xalanq's cftool, I'm mostly using command-line arguments, 
 * but sometimes still stdin and stdout.
 */

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

/* 
 * Declare commands relating to templates.
 */

#[derive(Args)]
pub struct TemplateCommandArgs {
    #[command(subcommand)]
    pub command: TemplateCommands,
}

#[derive(Subcommand)]
pub enum TemplateCommands {
    /// Add a template
    Add,

    /// Set a default template
    Set(TemplateArgs),

    /// Delete a template
    Delete(TemplateArgs)
}

fn prompt_until_valid(prompt: &str, invalid_message: &dyn Fn(&String) -> Option<&str>) -> String {
    let mut user_input = rprompt::prompt_reply(prompt).unwrap();
    loop {
        match invalid_message(&user_input) {
            None => {
                return user_input;
            }
            Some(msg) => {
                println!("{}", msg.red().bold());
                user_input = rprompt::prompt_reply(prompt).unwrap();
            }
        }
    }
}

use colored::Colorize;
use crate::config::{ Config, Template, TemplateScripts, LoginDetails };

pub fn prompt_new_template(conf: &Config) -> Template {
    println!("{}", "Add Template".blue().bold());
    println!("");

    println!("{}", "Choose an alias (alphanumeric characters)".blue().bold());
    let alias: String = prompt_until_valid("alias: ", &|user_input: &String| {
        if user_input.len() == 0 {
            Some("The alias can't be empty.")
        } else if !user_input.chars().all(|c| c.is_alphanumeric()) {
            Some("The alias must consist of only alphanumeric characters.")
        } else {
            None
        }
    });
    println!("");

    println!("{}", "Choose a language (index)".blue().bold());
    let lang: usize = prompt_until_valid("lang: ", &|user_input: &String| {
        match user_input.parse::<usize>() {
            Ok(x) => 
                if 100 < x {
                    Some("Language must be an integer between 0 and 100.")
                } else {
                    None
                }
            Err(e) => Some("Language must be a valid integer.")
        }
    }).parse::<usize>().unwrap();
    println!("");

    println!("{}", "Input the path to the template code.".blue().bold());
    let path: String = prompt_until_valid("path: ", &|user_input: &String| {
        None
    });
    println!("");

    println!("{}", "Write a script to run before executing the code.".blue().bold());
    let before_script = rprompt::prompt_reply("before: ").unwrap();
    println!("");

    println!("{}", "Write a script to execute the code.".blue().bold());
    let execute_script = rprompt::prompt_reply("execute: ").unwrap();
    println!("");

    println!("{}", "Write a script to run after executing the code.".blue().bold());
    let after_script = rprompt::prompt_reply("after: ").unwrap();
    println!("");

    println!("{}", "Template successfully added!".green().bold());
    println!("{}", "Failed to add template!".red().bold());

    Template {
        lang: lang,
        path: path.into(),
        suffix: Vec::new(),
        scripts: TemplateScripts {
            before: before_script,
            execute: execute_script,
            after: after_script,
        },
    }
}

use regex::Regex;
use inquire::validator::{StringValidator, Validation};
use email_address::EmailAddress;

pub fn prompt_login_details() -> LoginDetails {
    // This is what codeforces.com/register says
    let handle_re = Regex::new(r"^[\w-]+$").unwrap();
    let handle_or_email_validator = move |input: &str| 
        if EmailAddress::is_valid(&input) {
            Ok(Validation::Valid)
        } else if input.len() == 0 {
            Ok(Validation::Invalid("Field should not be empty.".into()))
        } else if handle_re.is_match(&input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Field should contain only Latin letters, digits, underscore or dash characters.".into()))
        };
    let password_validator = |input: &str|
        if input.len() < 5 {
            Ok(Validation::Invalid("Field should contain at least 5 characters.".into()))
        } else {
            Ok(Validation::Valid)
        };

    let handle_or_email = inquire::Text::new("Handle/Email:")
        .with_validator(handle_or_email_validator)
        .prompt().unwrap();

    let password = inquire::Password::new("Password:")
        .with_validator(password_validator)
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt().unwrap();

    let remember = inquire::Confirm::new("Stay signed in?")
        .with_default(true)
        .with_help_message("only for a month")
        .prompt().unwrap();

    // TODO: Actually use the "remember" variable.
    
    LoginDetails {
        handle: handle_or_email,
        password: password,
    }
}

pub fn prompt_delete_template(conf: &Config) -> Vec<&String> {
    let aliases = conf.templates.keys()
        .collect::<Vec<_>>();
    let selection: Vec<&String> = inquire::MultiSelect::new("Which templates do you wish to delete?", aliases)
        .prompt().unwrap();
    selection
}
