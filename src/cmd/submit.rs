use crate::{cf, client, config, files};
use std::{env, fs, io::{self, Write}};
use std::thread::sleep;
use std::time::Duration;

use anyhow::{Context, Result};
use colored::Colorize;

pub fn submit() -> Result<()> {

    // Hides the cursor
    print!("{}", ansi_escapes::CursorHide);

    let orange = colored::CustomColor::new(255, 140, 0);

    // Prints first message
    println!("{}", "        Submitted".blue().bold());
    println!("        #: {}", "57151524");
    println!("      Who: {}", "kriepiekrollie".custom_color(orange).bold());
    println!("     When: {}", "2019-07-16 07:59");
    println!("  Problem: {}", "A - Vus the Cossack and a Contest");
    println!("     Lang: {}", "GNU C++11");
    println!("  Verdict: {}", "In queue...");
    println!("     Time: {}", "-");
    println!("   Memory: {}", "-");
    io::stdout().flush().unwrap();
    print!("{}", ansi_escapes::CursorUp(3));

    // Waits one seconds
    sleep(Duration::from_secs(1));
    print!("  Verdict: {}", "Running on testcases 1");
    print!("{}", ansi_escapes::CursorLeft);
    io::stdout().flush().unwrap();

    sleep(Duration::from_secs(1));
    print!("  Verdict: {}", "Running on testcases 3");
    print!("{}", ansi_escapes::CursorLeft);
    io::stdout().flush().unwrap();

    sleep(Duration::from_secs(1));
    print!("  Verdict: {}", "Running on testcases 10");
    print!("{}", ansi_escapes::CursorLeft);
    io::stdout().flush().unwrap();

    sleep(Duration::from_secs(1));
    print!("  Verdict: {}", "Running on testcases 39");
    print!("{}", ansi_escapes::CursorLeft);
    io::stdout().flush().unwrap();

    sleep(Duration::from_secs(1));
    println!("  Verdict: {}", "Accepted                  ".green().bold());
    println!("     Time: {}", "31 ms");
    println!("   Memory: {}", "5.2 MB");
    print!("{}", ansi_escapes::CursorShow);

    // println!("Unimplemented");
    // std::process::exit(0);

    // let conf = config::Config::load_or_new(&files::config_file_path())
    //     .with_context(|| "Failed to load config.")?;
    // let clint = client::Client::load_or_new(&files::session_file_path())
    //     .with_context(|| "Failed to load config.")?;

    // Get problem information from current directory.
    // TODO: Perhaps give ability to overwrite this functionality with command-line args?
    // let current_dir = env::current_dir()
    //     .with_context(|| "Failed to get current directory.")?;
    // let problem_info = cf::ProblemInfo::from_path(&current_dir, &conf.cf_root)
    //     .with_context(|| "Failed to get problem information from current directory.")?;

    // println!("{:?}", problem_info);
    // println!("Unimplemented");

    // TODO: Delete this code and copy some code from test.rs, maybe DRY.

    // let code_info = cli::prompt_code()
    // let entries = fs::read_dir(current_dir).unwrap();

    // for entry in entries {
    //     if let Ok(entry) = entry {
    //         let file_name = entry.file_name();
    //         let file_name = file_name.to_str().unwrap();
    //         let file_name = file_name.to_lowercase();
    //         let file_name = file_name.as_str();
    //         let file_name = file_name.split('.').collect::<Vec<&str>>();
    //         let file_name = file_name[file_name.len()-1];
    //         let file_name = file_name.to_string();

    //         println!("{}", file_name);
    //         let sussy: Vec<_> = conf.templates
    //             .values()
    //             .filter(|x| x.suffix.contains(&file_name))
    //             .collect();

    //         println!("{:?}", sussy);
    //         // if conf.templates.contains_key(file_name) {
    //         //     println!("Using template: {}", file_name);
    //         // } else {
    //         //     println!("Using default template.");
    //         // }
    //     }
    //     // if let Ok(file_type) = pathprintln!();
    // }
    // let submission = cli::get_submission(&conf).unwrap();
    // client.submit_code(problem_info);
    Ok(())
}
