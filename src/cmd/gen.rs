use crate::{cli, config, files};
use std::{env, fs, io};
use std::io::{Write, BufRead, BufReader};
use itertools::Itertools;
use std::path::Path;
use anyhow::{Context, Result};
use colored::Colorize;

pub fn gen(args: cli::TemplateArgs) -> Result<()> {
    let conf = config::Config::load_or_new(&files::config_file_path())
        .with_context(|| "Failed to load config.")?;

    if conf.templates.len() == 0 {
        println!("{}", "You don't have any templates yet!".red().bold());
        println!("Consider adding one by running {}.", "\"cf template add\""
            .underline());
        return Ok(());
    } 

    let alias = match args.alias {
        Some(alias) => alias,
        None => match conf.default {
            Some(default) => default,
            None => {
                println!("{}", "You don't have a default templates yet!".red().bold());
                println!("Consider setting one by running {}.", "\"cf template set\""
                    .underline());
                let aliases = conf.templates.keys()
                    .map(|k| k.clone())
                    .collect::<Vec<_>>();
                inquire::Select::new("Which template would you like to generate from?", 
                    aliases).prompt().with_context(|| "Failed to get input from user.")?
            },
        },
    };

    let template = conf.templates.get(&alias).unwrap_or_else(|| {
        println!("{}", format!("There are no templates with the name {}.", 
            alias.underline()).red().bold());
        std::process::exit(0);
    });

    let current_dir = env::current_dir()
        .with_context(|| "Failed to get current directory.")?;

    let name = current_dir.file_name().with_context(||
        "Failed to deduce name from current directory.")?
        .to_str().expect("Failed to convert string thingy.").to_string();

    let ext = template.path.extension().with_context(|| 
        format!("Failed to decude extension from template path: {:?}", template.path))?
        .to_str().expect("Failed to convert string thingy.").to_string();

    let mut i = 0;
    let mut idx = String::new();
    while Path::new(&current_dir.join(format!("{}{}.{}", name, idx, ext))).exists() {
        i += 1;
        idx = i.to_string();
    }
    let path = &current_dir.join(format!("{}{}.{}", name, idx, ext));

    let file = fs::File::open(&template.path).with_context(||
        format!("Failed to open file for reading: {:?}", path))?;
    let reader = BufReader::new(file);
    let source = reader.lines().flatten()
        // // I'm too lazy to actually do this
        // .map(|line| line
        //     .replace("$%U%$", "handle")
        //     .replace("", "")
        //     .replace("", "")
        //     .replace("", "")
        //     .replace("", "")
        //     .replace("", "")
        //     .replace("", "")
        //     .replace("", "")
        // )
        .join("\n");

    let mut writer = fs::File::create(&path).map(io::BufWriter::new)
        .with_context(|| format!("Failed to open file for writing: {:?}", path))?;
    writer.write(source.as_bytes()).with_context(|| 
        format!("Failed to write to file: {:?}", path))?;

    // TODO: This entire file deserves to be modularized a lot so the code can 
    // be reused for cf parse...
    println!("{}", format!("Generated! see {}{}.{}", name, idx, ext).green().bold());

    Ok(())
}
