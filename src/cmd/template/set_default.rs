use crate::cli;
use crate::config;
use crate::files;
use colored::Colorize;
use anyhow::{Context, Result};

pub fn set_default(args: cli::TemplateArgs) -> Result<()> {

    let mut conf = config::Config::load_or_new(&files::config_file_path())
        .with_context(|| "Failed to load config.")?;

    if conf.templates.len() == 0 {
        println!("{}", "You don't have any templates yet!".red().bold());
        println!("Consider adding one by running {}.", "\"cf template add\""
            .underline());
        return Ok(());
    } 

    let alias = match args.alias {
        Some(alias) => alias,
        None => {
            if let Some(default) = conf.default {
                println!("{}", format!("Current default: {}", 
                    default.underline()).blue().bold());
            }
            let aliases = conf.templates.keys()
                .map(|k| k.clone())
                .collect::<Vec<_>>();
            inquire::Select::new("Which template should be the default?", aliases)
                .prompt().with_context(|| "Failed to get input from user.")?
        },
    };

    if conf.templates.contains_key(&alias) {
        conf.default = Some(alias.clone());
        conf.write(&files::config_file_path())
            .with_context(|| "Failed to save config to file.")?;
        println!("{}", format!("{} is now the default template.", 
            alias.underline()).green().bold());
    } else {
        println!("{}", format!("There are no templates with the name {}.", 
            alias.underline()).red().bold());
    };

    Ok(())
}
