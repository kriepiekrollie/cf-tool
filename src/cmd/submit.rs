use crate::cf;
use crate::client;
use crate::config;
use crate::files;
use std::{env, fs};
use anyhow::{Context, Result};

pub fn submit() -> Result<()> {
    let conf = config::Config::load_or_new(&files::config_file_path())
        .with_context(|| "Failed to load config.")?;
    let clint = client::Client::load_or_new(&files::session_file_path())
        .with_context(|| "Failed to load config.")?;

    // Get problem information from current directory.
    // TODO: Perhaps give ability to overwrite this functionality with command-line args?
    let current_dir = env::current_dir()
        .with_context(|| "Failed to get current directory.")?;
    let problem_info = cf::ProblemInfo::from_path(&current_dir, &conf.cf_root)
        .with_context(|| "Failed to get problem information from current directory.")?;

    println!("{:?}", problem_info);
    println!("Unimplemented");

    // TODO: Delete this code and copy some code from test.rs, maybe DRY.

    // let code_info = cli::prompt_code()
    let entries = fs::read_dir(current_dir).unwrap();

    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap();
            let file_name = file_name.to_lowercase();
            let file_name = file_name.as_str();
            let file_name = file_name.split('.').collect::<Vec<&str>>();
            let file_name = file_name[file_name.len()-1];
            let file_name = file_name.to_string();

            println!("{}", file_name);
            let sussy: Vec<_> = conf.templates
                .values()
                .filter(|x| x.suffix.contains(&file_name))
                .collect();

            println!("{:?}", sussy);
            // if conf.templates.contains_key(file_name) {
            //     println!("Using template: {}", file_name);
            // } else {
            //     println!("Using default template.");
            // }
        }
        // if let Ok(file_type) = pathprintln!();
    }
    // let submission = cli::get_submission(&conf).unwrap();
    // client.submit_code(problem_info);
    Ok(())
}
