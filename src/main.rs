mod cf;
mod cli;
mod client;
mod cmd;
mod config;
mod files;
mod utils;
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    match args.command {
        cli::Commands::Template(template_command_args) =>
            match template_command_args.command {
                cli::TemplateCommands::Add =>
                    cmd::template::add(),
                cli::TemplateCommands::Delete(template_args) =>
                    cmd::template::delete(template_args),
                cli::TemplateCommands::Set(template_args) =>
                    cmd::template::set_default(template_args),
            },
        // // Maybe i add a "list" command like this?
        // cli::Commands::List(list_command_args) =>
        //     match &list_command_args.command {
        //         cli::ListCommands::Problems(contest_args) =>
        //             cmd::list::problems(&contest_args),
        //         cli::ListCommands::Contests =>
        //             cms::list::contests(),
        //     }
        cli::Commands::Login =>
            cmd::login(),
        cli::Commands::Parse(contest_args) =>
            cmd::parse(contest_args),
        cli::Commands::Gen(template_args) =>
            cmd::gen(template_args),
        cli::Commands::Submit =>
            cmd::submit(),
        cli::Commands::Test =>
            cmd::test(),
        cli::Commands::Source =>
            cmd::source(),
    }
}
