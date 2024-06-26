use crate::{
    cf::{
        self,
        ProblemInfo,
    }, 
    client, 
    config, 
    files
};
use std::{
    env, 
    fs, 
    io::{
        self, 
        BufRead, 
        BufReader, 
        Write
    }, 
    path::PathBuf, 
    thread::sleep, 
    time::Duration
};

use anyhow::{Context, Result};
use colored::Colorize;

pub fn submit() -> Result<()> {

    let conf = config::Config::load_or_new(&files::config_file_path())
        .with_context(|| "Failed to load config.")?;

    let client = client::Client::load(&files::session_file_path())
        .with_context(|| "Failed to load config.")?;

    let current_dir = env::current_dir()
        .with_context(|| "Failed to get current directory.")?;

    let files_in_current_dir = fs::read_dir(&current_dir)
        .with_context(|| "Failed to get files from current directory.")?
        .flatten().map(|entry| entry.path().to_path_buf()).collect::<Vec<_>>();

    let template_extensions = conf.templates.values()
        .map(|val| val.path.extension()).flatten().collect::<Vec<_>>();

    let source_files = files_in_current_dir.iter().filter(|file| {
        if let Some(ext) = &file.extension() {
            template_extensions.contains(ext)
        } else { false }
    }).map(|file| file.file_name()).flatten().map(|s| s.to_str())
    .flatten().map(|s| s.to_string()).collect::<Vec<_>>();

    if source_files.len() == 0 {
        println!("{}", "Couldn't find any source files in this directory.".red().bold());
        std::process::exit(1);
    }

    let source_file = if source_files.len() > 1 {
        inquire::Select::new("Source file:", source_files)
            .with_help_message(" There seems to be more than one source file associated with a template ")
            .prompt().with_context(|| "Failed to get input from user.")?
    } else {
        let source_file = source_files.iter().next().unwrap().to_string();
        println!("{} Source file: {}", ">".green(), source_file.cyan());
        source_file
    };

    let source_file_pb = PathBuf::from(&source_file);
    let source_file_name = source_file_pb.file_stem()
        .expect("Couldn't get name from file.").to_str()
        .expect("Couldn't get name from file.").to_string();
    let source_file_ext = source_file_pb.extension()
        .expect("Couldn't get extension of source file, even though i could just a second ago???");

    let potential_templates = conf.templates.iter().filter(|(_, template)| {
        if let Some(ext) = template.path.extension() {
            ext == source_file_ext
        } else { false }
    }).collect::<Vec<_>>();

    // Ok we definitely should rather just let the user specify through
    // command line arguments if they wanna use a different template.
    // TODO: For now this works...
    let (_, template) = match potential_templates.len() {
        0 => panic!("This should not be possible"),
        1 => {
            let (alias, template) = potential_templates.into_iter().next()
                .expect("Failed to get template.");
            println!("{} Template: {}", ">".green(), alias.cyan());
            (alias, template)
        },
        _ => {
            let aliases = potential_templates.into_iter().map(|(alias, _)| alias).collect();
            let alias = inquire::Select::new("Template:", aliases)
                .with_help_message(" There seems to be more than one template associated with this source file. ")
                .prompt().with_context(|| "Failed to get input from user.")?;
            (alias, conf.templates.get(alias).expect("Failed to get template."))
        }
    };

    // we now have the source files?
    let source = fs::File::open(&source_file).map(BufReader::new)
        .with_context(|| format!("Failed to open file for reading: {:?}", source_file))?
        .lines().flatten().collect::<Vec<_>>().join("\n");

    let problem_info = ProblemInfo::from_path(&current_dir, &conf.cf_root)
        .with_context(|| "Failed to get problem information from current directory.")?;

    let mut submission_info = client.submit_code(&problem_info, &source, &template.lang_id)
        .with_context(|| "Failed to submit code.")?;

    // Hides the cursor
    print!("{}", ansi_escapes::CursorHide);
    // Prints first message
    println!("{}", "Submitted".green().bold());
    println!("        #: {}", submission_info.id);
    println!("      Who: {}", submission_info.who); //.custom_color(orange).bold());
    println!("     When: {}", submission_info.when);
    println!("  Problem: {}", submission_info.problem);
    println!("     Lang: {}", submission_info.lang);
    println!("  Verdict: {}", submission_info.verdict);
    println!("     Time: {}", submission_info.time);
    println!("   Memory: {}", submission_info.memory);
    print!("{}", ansi_escapes::CursorUp(3));

    loop {
        if !(submission_info.verdict.contains("Running on") 
          || submission_info.verdict.contains("queue") 
          || submission_info.verdict.contains("Submit")
          || submission_info.verdict.contains("Waiting")) {
            break;
        }
        sleep(Duration::from_secs(2));
        submission_info = client.get_submission(&problem_info.contest, &submission_info.id)?;
        print!("  Verdict: {}          ", submission_info.verdict.yellow().bold());
        print!("{}", ansi_escapes::CursorLeft);
        io::stdout().flush().unwrap();
    }
    if submission_info.verdict == String::from("Accepted") {
        println!("  Verdict: {}                   ", submission_info.verdict.green().bold());
    } else {
        println!("  Verdict: {}                   ", submission_info.verdict.red().bold());
    }
    println!("     Time: {}", submission_info.time);
    println!("   Memory: {}", submission_info.memory);

    print!("{}", ansi_escapes::CursorShow);
    Ok(())
}
