use crate::cli;
use crate::config;
use crate::files;
use colored::Colorize;
use anyhow::{Context, Result};

pub fn delete(args: cli::TemplateArgs) -> Result<()> {

    let mut conf = config::Config::load_or_new(&files::config_file_path())
        .with_context(|| "Failed to load config.")?;

    if conf.templates.len() == 0 {
        println!("{}", "You don't have any templates yet!".red().bold());
        println!("Consider adding one by running {}.", "\"cf template add\""
            .underline());
        return Ok(());
    } 

    let aliases = match args.alias {
        Some(alias) => {
            if !conf.templates.contains_key(&alias) {
                println!("{}", format!(
                    "There are no templates with the name {}.", alias
                    .underline()).red().bold());
                std::process::exit(0); // Vec::new()
            }
            vec![alias]
        },
        None => {
            let aliases = conf.templates.keys()
                .map(|k| k.clone())
                .collect::<Vec<_>>();
            inquire::MultiSelect::new("Which templates should be the deleted?", 
                aliases).prompt().with_context(|| "Failed to get input from user.")?
        },
    };

    if aliases.len() == 0 {
        println!("No templates were deleted.");
        return Ok(());
    }

    // println!("{} {}", aliases.join(", "), );
    for alias in aliases {
        println!("{}", format!("{} has been deleted.", alias.underline())
            .bold().green());
        conf.templates.remove(&alias);
    }

    if let Some(default) = &conf.default.clone() {
        if !conf.templates.contains_key(default) {
            conf.default = None;
        }
    }

    conf.write(&files::config_file_path())
        .with_context(|| "Failed to save config to file.")?;

    Ok(())
}
