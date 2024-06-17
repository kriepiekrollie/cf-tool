mod cf;
mod cli;
mod cmd;
mod config;
mod client;
mod utils;
use clap::Parser;

fn main() {
    let args = cli::Cli::parse();
    match &args.command {
        cli::Commands::Template(template_command_args) =>
            match &template_command_args.command {
                cli::TemplateCommands::Add =>
                    cmd::template_add(),
                cli::TemplateCommands::Delete(template_args) =>
                    cmd::template_delete(&template_args),
                cli::TemplateCommands::Set(template_args) =>
                    cmd::template_set_default(&template_args),
            },
        cli::Commands::Login =>
            cmd::login(),
        cli::Commands::Parse(contest_args) =>
            cmd::parse(&contest_args),
        cli::Commands::Gen(template_args) =>
            cmd::generate_file(&template_args),
        cli::Commands::Submit =>
            cmd::submit(),
        cli::Commands::Test =>
            cmd::test_code(),
        cli::Commands::Source =>
            cmd::source(),
    }
}

// use anyhow::{Context,Result};

// use std::thread::sleep;
// use std::time::Duration;
// use colored::Colorize;
// use std::io::{self, Write};
// fn test_ansi() {
//     // Hides the cursor
//     print!("{}", ansi_escapes::CursorHide);

//     let orange = colored::CustomColor::new(255, 140, 0);

//     // Prints first message
//     println!("{}", "Submitted".blue().bold());
//     println!("        #: {}", "57151524");
//     println!("      Who: {}", "kriepiekrollie".custom_color(orange).bold());
//     println!("     When: {}", "2019-07-16 07:59");
//     println!("  Problem: {}", "A - Vus the Cossack and a Contest");
//     println!("     Lang: {}", "GNU C++11");
//     println!("  Verdict: {}", "In queue...");
//     println!("     Time: {}", "-");
//     println!("   Memory: {}", "-");
//     io::stdout().flush().unwrap();
//     print!("{}", ansi_escapes::CursorUp(3));

//     // Waits one seconds
//     sleep(Duration::from_secs(1));
//     print!("  Verdict: {}", "Running on testcases 1");
//     print!("{}", ansi_escapes::CursorLeft);
//     io::stdout().flush().unwrap();

//     sleep(Duration::from_secs(1));
//     print!("  Verdict: {}", "Running on testcases 3");
//     print!("{}", ansi_escapes::CursorLeft);
//     io::stdout().flush().unwrap();

//     sleep(Duration::from_secs(1));
//     print!("  Verdict: {}", "Running on testcases 10");
//     print!("{}", ansi_escapes::CursorLeft);
//     io::stdout().flush().unwrap();

//     sleep(Duration::from_secs(1));
//     print!("  Verdict: {}", "Running on testcases 39");
//     print!("{}", ansi_escapes::CursorLeft);
//     io::stdout().flush().unwrap();

//     sleep(Duration::from_secs(1));
//     println!("  Verdict: {}", "Accepted                  ".green().bold());
//     println!("     Time: {}", "31 ms");
//     println!("   Memory: {}", "5.2 MB");
//     print!("{}", ansi_escapes::CursorShow);
// }

