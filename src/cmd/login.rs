use crate::client;
use crate::files;
use colored::Colorize;
use email_address::EmailAddress;
use regex::Regex;
use inquire::validator::Validation;
use crate::config::LoginDetails;
use anyhow::{Context, Result};

pub fn login() -> Result<()> {
    // TODO: Check if user is already logged in and then do stuff
    let mut clint = client::Client::load_or_new(&files::session_file_path())
        .with_context(|| "Failed to load config.")?;

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
        .prompt().with_context(|| "Failed to get input from user.")?;

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
        .prompt().with_context(|| "Failed to get input from user.")?;

    let remember = inquire::Confirm::new("Remember me?")
        .with_default(true)
        .with_help_message(" only for a month... ")
        .prompt().with_context(|| "Failed to get input from user.")?;

    if clint.login(LoginDetails{handle_or_email, password, remember})? {
        clint.write(&files::session_file_path())?;
    }

    Ok(())
}
