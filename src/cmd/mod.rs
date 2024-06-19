use crate::cli;
use crate::client;
use crate::config;
use crate::files;
pub mod template;
use colored::Colorize;
use email_address::EmailAddress;
use regex::Regex;
use inquire::validator::Validation;
use crate::config::LoginDetails;
use anyhow::Result;

pub fn test_code() -> Result<()> {
    println!("Testing code.");
    Ok(())
}

pub fn generate_file(_args: cli::TemplateArgs) -> Result<()> {
    println!("Generating file.");
    Ok(())
}

pub fn parse(args: cli::ContestArgs) -> Result<()> {
    let conf = config::Config::load_or_new(&files::config_file_path()).unwrap();
    let clint = client::Client::load_or_new(&files::session_file_path())?;
    // Definitely move some stuff out of this function.
    clint.parse_sample_testcases(&args, &conf.cf_root)?;
    // clint is doing some heavy lifting
    Ok(())
}

pub fn login() -> Result<()> {
    // TODO: Check if user is already logged in and then do stuff
    let mut clint = client::Client::load_or_new(&files::session_file_path())?;

    println!("{}", "- Codeforces Login -".blue().bold());
    println!("  This will overwrite your previous details if you were already \
        logged in.");

    // This is what codeforces.com/register says
    let handle_re = Regex::new(r"^[\w-]+$").unwrap();
    let handle_or_email_validator = move |input: &str| 
        if EmailAddress::is_valid(&input) {
            Ok(Validation::Valid)
        } else if input.len() == 0 {
            Ok(Validation::Invalid("Field should not be empty.".into()))
        } else if handle_re.is_match(&input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Field should contain only Latin letters, \
                digits, underscore or dash characters.".into()))
        };
    let handle_or_email = inquire::Text::new("Handle/Email:")
        .with_validator(handle_or_email_validator)
        .prompt().unwrap();

    let password_validator = |input: &str|
        if input.len() < 5 {
            Ok(Validation::Invalid("Field should contain at least 5 characters.".into()))
        } else {
            Ok(Validation::Valid)
        };
    let password = inquire::Password::new("Password:")
        .with_validator(password_validator)
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .without_confirmation()
        .prompt().unwrap();

    let remember = inquire::Confirm::new("Remember me?")
        .with_default(true)
        .with_help_message(" only for a month... ")
        .prompt().unwrap();

    if clint.login(LoginDetails{handle_or_email, password, remember})? {
        clint.write(&files::session_file_path())?;
    }

    Ok(())
}

pub fn submit() -> Result<()> {
    //    let current_dir = std::env::current_dir().unwrap();
    //    let mut conf = config::Config::load_or_new(&files::config_file_path()).unwrap();
    //    let mut clint = client::Client::load_or_new(&files::session_file_path());

    //    // Get problem information from current directory.
    //    // TODO: Perhaps give ability to overwrite this functionality with command-line args?
    //    let problem_info = cf::ProblemInfo::from_path(&current_dir, &conf.cf_root).unwrap();

    //    // let code_info = cli::prompt_code()
    //    let current_dir = std::env::current_dir().unwrap();
    //    let entries = std::fs::read_dir(current_dir).unwrap();

    //    for entry in entries {
    //        if let Ok(entry) = entry {
    //            let file_name = entry.file_name();
    //            let file_name = file_name.to_str().unwrap();
    //            let file_name = file_name.to_lowercase();
    //            let file_name = file_name.as_str();
    //            let file_name = file_name.split('.').collect::<Vec<&str>>();
    //            let file_name = file_name[file_name.len()-1];
    //            let file_name = file_name.to_string();

    //            println!("{}", file_name);
    //            let sussy: Vec<_> = conf.templates
    //                .values()
    //                .filter(|x| x.suffix.contains(&file_name))
    //                .collect();

    //            println!("{:?}", sussy);
    //            // if conf.templates.contains_key(file_name) {
    //            //     println!("Using template: {}", file_name);
    //            // } else {
    //            //     println!("Using default template.");
    //            // }
    //        }
    //        // if let Ok(file_type) = pathprintln!();
    //    }
    //    // let submission = cli::get_submission(&conf).unwrap();
    //    // client.submit_code(problem_info);
    Ok(())
}

pub fn source() -> Result<()> {
    let url = "https://www.github.com/";
    open::that(url)
        .unwrap_or_else(|_| {
            println!("Failed to open link in browser: {}", url)
        });
    Ok(())
}
