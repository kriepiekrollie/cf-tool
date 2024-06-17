use crate::cli;
use crate::config;
use crate::files;

pub fn add() {
    let mut conf = config::Config::load_or_new(&files::config_file_path()).unwrap();

    conf.add_template(cli::prompt_new_template());
    conf.save(&files::config_file_path()).unwrap();
}

pub fn delete(args: &cli::TemplateArgs) {
    println!("Unimplemented.");
}

pub fn set_default(args: &cli::TemplateArgs) {
    println!("Unimplemented.");
}

