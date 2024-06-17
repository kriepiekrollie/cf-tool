use crate::cli;
use crate::config;

pub fn add() {
    let config_dir = dirs::config_dir().unwrap().join("cf-tool");
    let config_file_path = config_dir.join("config.json");
    let mut conf = config::Config::load_or_new(&config_file_path).unwrap();

    conf.add_template(cli::prompt_new_template());
    conf.save(&config_file_path).unwrap();
}

pub fn delete(args: &cli::TemplateArgs) {
    config::delete_template(&args).unwrap();
}

pub fn set_default(args: &cli::TemplateArgs) {
    config::set_default_template(&args);
}

