use clap::{Args, Parser, Subcommand};
use crate::cf;

// The commands I am currently planning to support:
//   cf login
//   cf template 
//   cf parse
//   cf submit
//   cf test
//   cf contrib

#[derive(Parser)]
#[command(version, about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Login
    Login,

    /// Configure templates
    Template(TemplateCommandArgs),

    /// Fetch samples from contest
    Parse(ContestArgs),

    /// Generate file from template
    Gen(TemplateArgs),

    /// Submit code for problem
    Submit,

    /// Test code locally
    Test,

    /// Open the Github page of cf-tool
    Source,
}

#[derive(Args)]
pub struct TemplateCommandArgs {
    #[command(subcommand)]
    pub command: TemplateCommands,
}

#[derive(Subcommand)]
pub enum TemplateCommands {
    /// Add a template
    Add,

    /// Set a default template
    Set(TemplateArgs),

    /// Delete a template
    Delete(TemplateArgs)
}

// These are the arguments that are required to specify a contest.
// for example:
//   cf parse 2000
//   cf parse --gym 1234
//   cf parse --contest 12341243
//
//   Note that --gym and --contest can't be used together (using arg groups)

#[derive(Args)]
pub struct ContestArgs {
    /// Specify a contest (default)
    #[arg(short, long, group = "contest_type")]
    contest: bool,

    /// Specify a gym
    #[arg(short, long, group = "contest_type")]
    gym: bool,

    /// The contest ID
    pub contest_id: String,
}

impl ContestArgs {
    pub fn contest_type(&self) -> cf::ContestType {
        if self.gym {
            return cf::ContestType::Gym;
        } else if self.contest {
            return cf::ContestType::Contest;
        }
        cf::ContestType::default()
    }
}

// These are the arguments that are required to specify a template.
// for example:
//   cf gen java
//   cf template delete py
//   cf template set cpp
// etc.

#[derive(Args)]
pub struct TemplateArgs {
    /// Specify a template by its alias
    pub alias: Option<String>,
}
