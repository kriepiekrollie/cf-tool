use crate::cli;
use crate::config;
use crate::files;
use crate::cf;
use std::path::Path;
use itertools::Itertools;
use regex::Regex;
use inquire::validator::Validation;
use colored::Colorize;
use crate::config::{Template, TemplateScripts};
use anyhow::{Context, Result};

pub fn add() -> Result<()> {

    let mut conf = config::Config::load_or_new(&files::config_file_path())
        .with_context(|| "Failed to load config from file.")?;

    // Header
    println!("{}", "- Create a new template -".blue().bold());
    println!("");
    println!("");

    // Alias message
    println!("{}", "+ Choose an alias for your new template.".blue().bold());
    println!("  This will be used in commands such as:");
    println!("   -  cf template delete <alias>");
    println!("   -  cf gen <alias>");
    println!("");

    // Alias input
    let existing_aliases = conf.templates.keys().map(|k| k.clone())
        .collect::<Vec<_>>();
    let alias_re = Regex::new(r"^[\w-]+$").unwrap();
    let alias_validator = move |input: &str| 
        if input.len() == 0 {
            Ok(Validation::Invalid("Alias should not be empty.".into()))
        } else if alias_re.is_match(&input) {
            if existing_aliases.contains(&input.to_string()) {
                Ok(Validation::Invalid("You already have a template with \
                        this alias.".into()))
            } else {
                Ok(Validation::Valid)
            }
        } else {
            Ok(Validation::Invalid("Alias may only contain alphanumberic \
                    characters, underscores or dashes.".into()))
        };
    let alias = inquire::Text::new("Alias:")
        .with_validator(alias_validator)
        .with_help_message(" Examples: \"cpp\", \"py\", \"template1\", ... ")
        .prompt().with_context(|| "Failed to get input from user.")?;
    println!("");
    println!("");
    println!("");

    // Language message
    println!("{}", "+ Choose a language for your new template.".blue().bold());
    println!("  If this list is outdated: github.com/kriepiekrollie/cf-tool/");
    println!("");

    // Language input
    let langs = cf::languages::Languages::default();
    let lang_options = langs.id_map.keys().sorted().collect::<Vec<_>>();
    let lang = inquire::Select::new("Language:", lang_options)
        .prompt().with_context(|| "Failed to get input from user.")?;
    let lang_id = langs.id_map.get(lang)
        .expect("Failed to get language id?").clone();
    println!("");
    println!("");
    println!("");

    // Filepath message
    println!("{}", "+ Provide a path to your template source.".blue().bold());
    println!("  This path will be used to generate source files. When generating source");
    println!("  files from your template, the same suffix will be used as the template,");
    println!("  and cf-tool will replace the these placeholders as follows:");
    println!("   -  $%U%$   Handle   e.g. kriepiekrollie");
    println!("   -  $%L%$   Link     e.g. https://codeforces.com/contest/1985/problem/A");
    println!("   -  $%Y%$   Year     e.g. 2016");
    println!("   -  $%M%$   Month    e.g. 05");
    println!("   -  $%D%$   Day      e.g. 28");
    println!("   -  $%h%$   Hour     e.g. 20");
    println!("   -  $%m%$   Minute   e.g. 00");
    println!("   -  $%s%$   Second   e.g. 00");
    println!("");

    // Filepath input
    let path_validator = |input: &str| {
        let expand_homedir = shellexpand::tilde(input).to_string();
        let path = Path::new(&expand_homedir);
        if !path.exists() {
            Ok(Validation::Invalid("Path does not exist.".into()))
        } else if !path.is_file() {
            Ok(Validation::Invalid("Path does not point to file.".into()))
        } else {
            Ok(Validation::Valid)
        }
    };
    let path: String = inquire::Text::new("Path:")
        .with_validator(path_validator)
        .with_help_message(" Example: ~/cf/template.cpp ")
        .prompt().with_context(|| "Failed to get input from user.")?;
    let path = shellexpand::tilde(&path).to_string();
    println!("");
    println!("");
    println!("");

    println!("{}", "+ Write scripts for executing your code.".blue().bold());
    println!("  cf-tool will run 3 scripts when the \"cf test\" command is used:");
    println!("   1. before_script   (executed once)");
    println!("   2. execute_script  (executed for each testcase)");
    println!("   3. after_script    (executed once)");
    println!("");
    println!("  \"execute_script\" assumes that your code uses standard input/output.");
    println!("  You may use the following placeholders in your scripts:");
    println!("   -  $%path%$  Current directory.            e.g. /home/user/cf/gym/123/a");
    println!("   -  $%file%$  Source filename.              e.g. a.cpp");
    println!("   -  $%name%$  Source filename, no suffix.   e.g. a");
    println!("   -  $%time%$  Time in seconds since epoch.  e.g. 1464465600");
    println!("");

    let before_script = inquire::Text::new("before_script:")
        .with_help_message(" Examples: \"g++ $%file%$ -o $%name%$ -DLOCAL\", \"\" ")
        .prompt_skippable().with_context(|| "Failed to get input from user.")?;
    let execute_script = inquire::Text::new("execute_script:")
        .with_help_message(" Examples: \"./$%name%$\", \"python3 $%file%$\" ")
        .prompt().with_context(|| "Failed to get input from user.")?;
    let after_script = inquire::Text::new("after_script:")
        .with_help_message(" Examples: \"rm $%name%$\", \"\"")
        .prompt_skippable().with_context(|| "Failed to get input from user.")?;

    let make_new_default = match conf.default {
        None => true,
        _ => {
            println!("");
            println!("");
            println!("");
            println!("{}", "+ Make this template the new default?".blue().bold());
            inquire::Confirm::new("")
                .with_default(false)
                .prompt().with_context(|| "Failed to get input from user.")?
        },
    };
    println!("");

    conf.templates.insert(alias.clone(), Template {
        lang_id,
        path: path.into(),
        suffix: Vec::new(),
        scripts: TemplateScripts {
            before: before_script,
            execute: execute_script,
            after: after_script,
        },
    });
    if make_new_default {
        conf.default = Some(alias);
    }

    conf.write(&files::config_file_path())
        .with_context(|| "Failed to save config to file.")?;

    println!("{}", "Template successfully added!".green().bold());
    Ok(())
}

pub fn delete(args: cli::TemplateArgs) -> Result<()> {

    let mut conf = config::Config::load_or_new(&files::config_file_path())
        .with_context(|| "Failed to load config from file.")?;

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

    conf.write(&files::config_file_path())
        .with_context(|| "Failed to save config to file.")?;

    Ok(())
}

pub fn set_default(args: cli::TemplateArgs) -> Result<()> {

    let mut conf = config::Config::load_or_new(&files::config_file_path())
        .with_context(|| "Failed to load config from file.")?;

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
