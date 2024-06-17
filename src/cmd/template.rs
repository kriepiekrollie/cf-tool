use crate::cli;
use crate::config;
use crate::files;
use colored::Colorize;

pub fn add() {
    let mut conf = config::Config::load_or_new(&files::config_file_path()).unwrap();

    conf.add_template(cli::prompt_new_template(&conf));
    conf.save(&files::config_file_path()).unwrap();
}

pub fn delete(args: &cli::TemplateArgs) {
    let mut conf = config::Config::load_or_new(&files::config_file_path()).unwrap();
    if let Some(alias) = &args.alias {
        match conf.delete_template(alias) {
            _ => println!("TODO"),
        }
        println!("Deleting template {}.", alias.bold());
    } else {
        let aliases = cli::prompt_delete_template(&conf);
        for alias in &aliases {
            println!("Deleting template {}.", alias.bold());
        }
    }
}

pub fn set_default(args: &cli::TemplateArgs) {
    println!("Unimplemented.");
}

