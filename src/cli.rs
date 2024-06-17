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
    #[arg(short, long, group = "template_specifier")]
    pub alias: Option<String>,

    /// Specify a template by its index
    #[arg(short, long, group = "template_specifier")]
    pub index: Option<usize>,
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
use crate::config::{ Template, TemplateScripts, LoginDetails };

pub fn prompt_new_template() -> Template {
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
        alias: alias,
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

pub fn prompt_login_details() -> LoginDetails {
    println!("{}", "Login".blue().bold());
    let handle_or_email = rprompt::prompt_reply("handle/email: ").unwrap();
    let password = rpassword::prompt_password("password: ").unwrap();
    
    LoginDetails {
        handle: handle_or_email,
        password: password,
    }
}
