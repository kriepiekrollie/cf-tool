pub mod config;
pub mod contest;
pub mod template;
use clap::{Args, Parser, Subcommand};

/*
 * This module declares how the user can interact with the tool.
 * I am using the clap library and at some points rprompt and rpassword.
 */

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/*
 * Currently, cf_tool supports the following subcommands:
 *   config
 *   parse
 *   gen
 *   submit
 *   test
 *   source
 * 
 * Perhaps, maybe in future implement the stuff from original cftool?
 *   contest list
 *   problem list (with colors and symbols indicating submission results?)
 *   register
 *   open (shouldn't be too hard)
 */

#[derive(Subcommand)]
pub enum Commands {
    /// Change configuration
    Config(config::ConfigArgs),

    /// Fetch samples from contest
    Parse(contest::ContestArgs),

    /// Generate file from template
    Gen(template::TemplateArgs),

    /// Submit code for problem
    Submit,

    /// Test code locally
    Test,

    /// Open the Github page of cf_tool
    Source,
}
