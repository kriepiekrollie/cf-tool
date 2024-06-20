use crate::{cli, client, config, files, utils};
use std::{fs, io};
use std::io::Write;
use anyhow::{Context, Result};
use itertools::Itertools;
use colored::Colorize;

pub fn parse(args: cli::ContestArgs) -> Result<()> {

    let conf = config::Config::load_or_new(&files::config_file_path())
        .with_context(|| "Failed to load config.")?;
    let clint = client::Client::load_or_new(&files::session_file_path())
        .with_context(|| "Failed to load config.")?;

    let test_map = clint.parse_sample_testcases(&args)?;
    
    let contest_dir = conf.cf_root.join(format!("{}", &args.contest_type()))
        .join(&args.contest_id);

    for (problem_id, tests) in test_map.iter().sorted() {

        let problem_dir = contest_dir.join(&problem_id.to_lowercase());
        fs::create_dir_all(&problem_dir).with_context(|| 
            format!("Failed to create directory: {:?}", problem_dir))?;

        let mut count = 0;
        for (i, (input, output)) in tests.iter().enumerate() {

            let path = problem_dir.join(format!("{}.in", i));
            let mut writer = fs::File::create(&path).map(io::BufWriter::new)
                .with_context(|| format!("Failed to open file for writing: {:?}", path))?;
            writer.write(input.as_bytes()).with_context(|| 
                format!("Failed to write to file: {:?}", path))?;

            let path = problem_dir.join(format!("{}.out", i));
            let mut writer = fs::File::create(&path).map(io::BufWriter::new)
                .with_context(|| format!("Failed to open file for writing: {:?}", path))?;
            writer.write(output.as_bytes()).with_context(|| 
                format!("Failed to write to file: {:?}", path))?;

            count += 1;
        }
        
        println!("Problem {}: parsed {} sample testcase{}.", 
            problem_id, count, if count == 1 { "" } else { "s" });
    }

    println!("{}", format!("Sample testcases have been stored in {:?}", 
            utils::path_shortest_repr(&contest_dir)).green().bold());

    // TODO: IDEA!!! what if we run a user-defined script here?
    Ok(())
}
