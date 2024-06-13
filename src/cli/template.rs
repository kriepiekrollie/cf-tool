use clap::Args;

/*
 * Declare how a user can specify which template a command should use.
 * This struct will be used in utils to fetch the template from config.
 */

#[derive(Args)]
pub struct TemplateArgs {
    /// Specify a template by its alias
    #[arg(short, long, group = "template_specifier")]
    alias: Option<String>,

    /// Specify a template by its index
    #[arg(short, long, group = "template_specifier")]
    index: Option<usize>,
}
