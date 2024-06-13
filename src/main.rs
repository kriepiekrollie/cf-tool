mod cli;
mod config;
mod template;
mod client;
use clap::Parser;

fn generate_file(_args: &cli::template::TemplateArgs) {
    println!("Generating file.");
}
fn submit_code() {
    println!("Submitting code.");
}
fn test_code() {
    println!("Testing code.");
}

fn main() {
    let args = cli::Cli::parse();

    match &args.command {
        cli::Commands::Config(config_args) => config::configure(&config_args),
        cli::Commands::Parse(contest_args) => client::parser::parse_samples(&contest_args),
        cli::Commands::Gen(template_args) => generate_file(&template_args),
        cli::Commands::Submit => client::submit_code(),
        cli::Commands::Test => test_code(),

        cli::Commands::Source => {
            let url = "https://www.github.com/";
            open::that(url)
            .unwrap_or_else(|_| panic!(
                "Failed to open link in browser. Maybe just copy this instead: {}",
                url
            ))
        },
    }
}
