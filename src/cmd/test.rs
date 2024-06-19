use crate::{config, files};
use std::{env, fs, time};
use std::io::{BufRead, BufReader};
use std::time::{Duration, SystemTime};
use std::path::PathBuf;
use std::collections::HashMap;
use itertools::Itertools;
use colored::Colorize;
use anyhow::{Context, Result};

pub fn test() -> Result<()> {
    let conf = config::Config::load_or_new(&files::config_file_path())
        .with_context(|| "Failed to load config.")?;

    // We don't care about what problem folder we're in.
    // Just run whatever code is here lmao (that's actually what the original
    // cf-tool does)

    let current_dir = env::current_dir()
        .with_context(|| "Failed to get current directory.")?;

    let files_in_current_dir = fs::read_dir(&current_dir)
        .with_context(|| "Failed to get files from current directory.")?
        .flatten().map(|entry| entry.path().to_path_buf()).collect::<Vec<_>>();

    let template_extensions = conf.templates.values()
        .map(|val| val.path.extension()).flatten().collect::<Vec<_>>();

    // This is so bad i hate this please help

    let source_files = files_in_current_dir.iter().filter(|file| {
        if let Some(ext) = &file.extension() {
            template_extensions.contains(ext)
        } else { false }
    }).map(|file| file.file_name()).flatten().map(|s| s.to_str())
    .flatten().map(|s| s.to_string()).collect::<Vec<_>>();

    let input_files = files_in_current_dir.iter().filter(|file| {
        if let Some(ext) = &file.extension() {
            *ext == "in"
        } else { false }
    }).map(|file| file.file_stem()).flatten().map(|s| s.to_str())
    .flatten().map(|s| s.to_string()).collect::<Vec<_>>();

    let output_files = files_in_current_dir.iter().filter(|file| {
        if let Some(ext) = &file.extension() {
            *ext == "out"
        } else { false }
    }).map(|file| file.file_stem()).flatten().map(|s| s.to_str())
    .flatten().map(|s| s.to_string()).collect::<Vec<_>>();

    let unhappiness = input_files.len() > 0 || output_files.len() > 0;

    let sample_tests = input_files.into_iter()
        .filter(|file| output_files.contains(file)).collect::<Vec<_>>();

    if source_files.len() == 0 {
        println!("{}", "Couldn't find any source files in this directory.".red().bold());
        std::process::exit(0);
    }
    if sample_tests.len() == 0 {
        println!("{}", "Couldn't find any sample testcases in this directory.".red().bold());
        if unhappiness {
            println!("(There were some files without partners?)");
        }
        std::process::exit(0);
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

    // Now, we have 
    // - the id's of the sample tests
    // - the source file
    // - the associated template (and thus the scripts)

    let env_vars = Some(HashMap::from([
        (String::from("FILE"), source_file),
        (String::from("NAME"), source_file_name),
        (String::from("TIME"), SystemTime::now().duration_since(time::UNIX_EPOCH)
             .unwrap_or_else(|_| Duration::new(0, 0)).as_secs().to_string()),
    ]));
    let options = run_script::ScriptOptions {
        runner: None,
        runner_args: None,
        working_directory: None,
        input_redirection: run_script::IoOptions::Inherit,
        output_redirection: run_script::IoOptions::Pipe,
        exit_on_error: false,
        print_commands: false,
        env_vars,
    };
    let args = vec![];

    if let Some(before_script) = &template.scripts.before {
        println!("");
        println!("before-script: {:?}", before_script);
        let (code, output, error) = run_script::run(
            &before_script,
            &args,
            &options,
        ).with_context(|| "Failed to run script.")?;
        if output.chars().any(|c| c != ' ') {
            println!("Output: {}", output);
        }
        if code != 0 {
            println!("{}", "Exited with non-zero exit code:".red().bold());
            println!("{}", error);
            std::process::exit(0);
        }
        println!("{}", "Successfully ran before-script.".green().bold());
    }

    println!("");
    println!("execute-script: {:?}", &template.scripts.execute);
    let children = sample_tests.iter().sorted().map(|i| {
        let script = format!("{} < {}.in", &template.scripts.execute, i);
        let child = run_script::spawn(
            &script,
            &args,
            &options,
        );
        match child {
            Ok(child) => {
                println!("Running code on testcase {}...", i);
                Ok((i, child))
            },
            Err(e) => {
                println!("Failed to run script on testcase {}.", i);
                Err((i, e))
            },
        }
    }).collect::<Vec<_>>();

    // TODO: I could potentially add support for TLE using child.try_wait but
    // realistically, knowning whether or not your local code runs into TLE
    // is useless.

    for result in children {
        display_test_result(result)?;
    }

    println!("");
    Ok(())
}

fn display_test_result(result: Result<(&String, std::process::Child), (&String, run_script::ScriptError)>) -> Result<()> {
    println!("");

    if let Err((i, e)) = result {
        println!("{}", format!(" - Testcase {} result - ", i).blue().bold());
        println!("{}", " Execution failed:".red().bold());
        println!("{}", format!("{}", e).red().bold());
        return Ok(());
    }

    let (i, child) = result.expect("All of my beliefs were wrong.");
    println!("{}", format!(" - Testcase {} result - ", i).blue().bold());

    let result = child.wait_with_output();

    if let Err(e) = &result {
        println!("{}", " Execution failed:".red().bold());
        println!("{}", format!("{:?}", e).red().bold());
        return Ok(());
    }

    let result = result.expect("I have to reconsider all of my choices in life.");

    if !result.status.success() {
        println!("{}", " Execution failed:".red().bold());
        println!("{}", String::from_utf8_lossy(&result.stderr));
        return Ok(());
    }

    let path = format!("{}.out", i);
    let file = fs::File::open(&path).with_context(||
        format!("Failed to open file for reading: {:?}", path))?;

    let expected = BufReader::new(file).lines().flatten()
        .filter(|s| s.chars().any(|c| c != ' ' && c != '\n'))
        .map(|s| s.split(" ").filter(|t| t.len() > 0).join(" "))
        .join("\n");

    let stdout = String::from_utf8_lossy(&result.stdout).split("\n")
        .filter(|s| s.chars().any(|c| c != ' ' && c != '\n'))
        .map(|s| s.split(" ").filter(|t| t.len() > 0).join(" "))
        .join("\n");


    let stderr = String::from_utf8_lossy(&result.stderr).split("\n")
        .filter(|s| s.chars().any(|c| c != ' ' && c != '\n'))
        .map(|s| s.split(" ").filter(|t| t.len() > 0).join(" "))
        .join("\n");

    if stdout == expected {
        println!("{}", format!(" Accepted!").green().bold());
        return Ok(());
    }

    println!("{}", format!(" Wrong Answer!").red().bold());
    {
        use comfy_table::presets::UTF8_FULL;
        use comfy_table::*;

        let mut table = Table::new();
        if supports_unicode::on(supports_unicode::Stream::Stdout) {
            table.load_preset(UTF8_FULL);
        }
        table
            .set_header(vec![
                Cell::new("Expected").set_alignment(CellAlignment::Center),
                Cell::new(" stdout ").set_alignment(CellAlignment::Center),
                Cell::new(" stderr ").set_alignment(CellAlignment::Center),
            ])
            .add_row(vec![
                expected,
                stdout,
                stderr,
            ]);

        // Set the default alignment for the third column to right
        let column = table.column_mut(2).expect("Failed to draw table.");
        column.set_cell_alignment(CellAlignment::Right);

        println!("{table}");
    }

    Ok(())
}

