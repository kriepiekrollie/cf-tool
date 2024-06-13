use clap::{Args, Subcommand};
use crate::cli::template::TemplateArgs;

#[derive(Args)]
pub struct TemplateConfigArgs {
    #[command(subcommand)]
    cmd: TemplateConfigCommands,
}

#[derive(Subcommand)]
pub enum TemplateConfigCommands {
    /// Add a template
    Add,

    /// Set a default template
    SetDefault(TemplateArgs),

    /// Delete a template
    Delete(TemplateArgs)
}
