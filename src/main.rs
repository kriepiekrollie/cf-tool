mod cf;
mod cli;
mod client;
mod cmd;
mod config;
mod files;
mod utils;
use cli::*;
use clap::Parser;
use anyhow::Result;

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Commands::Template(template_command_args) =>
            match template_command_args.command {
                TemplateCommands::Add =>
                    cmd::template::add(),
                TemplateCommands::Delete(template_args) =>
                    cmd::template::delete(template_args),
                TemplateCommands::Set(template_args) =>
                    cmd::template::set_default(template_args),
            },
        // // Maybe i add a "list" command like this?
        // Commands::List(list_command_args) =>
        //     match &list_command_args.command {
        //         ListCommands::Problems(contest_args) =>
        //             cmd::list::problems(&contest_args),
        //         ListCommands::Contests =>
        //             cms::list::contests(),
        //     }
        Commands::Login =>
            cmd::login(),
        Commands::Parse(contest_args) =>
            cmd::parse(contest_args),
        Commands::Gen(template_args) =>
            cmd::gen(template_args),
        Commands::Submit =>
            cmd::submit(),
        Commands::Test =>
            cmd::test(),
        Commands::Source =>
            cmd::source(),
    }
}
