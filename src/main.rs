mod cli;
mod config;
mod client;
use clap::Parser;

fn generate_file(_args: &cli::TemplateArgs) {
    println!("Generating file.");
}
fn test_code() {
    println!("Testing code.");
}

fn main() {
    let args = cli::Cli::parse();

    let config_dir = dirs::config_dir().unwrap().join("cf-tool");
    std::fs::create_dir_all(&config_dir).unwrap();
    let configuration = config::Config::load_or_new(&config_dir.join("config.json"));
    let mut client = client::Client::load_or_new(&config_dir.join("session.json"));

    match &args.command {
        cli::Commands::Config(config_args) =>
            match &config_args.command {
                cli::ConfigCommands::Login => {
                    client.login(cli::prompt_login_details());
                    client.save(&config_dir.join("session.json"));
                },
                cli::ConfigCommands::Template(template_config_args) =>
                    match &template_config_args.command {
                        cli::TemplateConfigCommands::Add =>
                            config::add_template(&cli::prompt_new_template()).unwrap(),
                        cli::TemplateConfigCommands::Delete(template_args) =>
                            config::delete_template(&template_args).unwrap(),
                        cli::TemplateConfigCommands::SetDefault(template_args) =>
                            config::set_default_template(&template_args).unwrap(),
                    },
            },
        cli::Commands::Parse(contest_args) => client.parse_samples(&contest_args),
        cli::Commands::Gen(template_args) => generate_file(&template_args),
        cli::Commands::Submit => client.submit_code(),
        cli::Commands::Test => test_code(),

        cli::Commands::Source => {
            let url = "https://www.github.com/";
            open::that(url)
            .unwrap_or_else(|_| panic!(
                "Failed to open link in browser. Maybe just copy this instead: {}",
                url
            ))
        }
    }
}
