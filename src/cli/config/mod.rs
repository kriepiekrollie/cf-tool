pub mod template;
use clap::{Args, Subcommand};

/*
 * This module declares how the config subcommand works.
 * Unlike xalanq's cftool, I'm mostly using command-line arguments, 
 * but sometimes still stdin and stdout.
 */

#[derive(Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    command: ConfigCommands,
}

/*
 * For now i'm only implementing the following config options:
 *
 *   login
 *   template add
 *            delete
 *            set-default
 *              
 * I might add some of the original options later:
 *
 *   gen-after-parse
 *   set-host
 *   set-proxy
 *   set-folder root
 *              contest
 *              gym
 *              group
 *              acmsguru
 * 
 * I never really found any of these useful, other than gen-after-parse
 * so I'll just enable it by default.
 *
 * I can see how host and proxy could be useful for some...
 *
 * I really don't get the point of set-folder though.
 * If you want your own directory structure just fork this project lol
 */

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Login
    Login,

    /// Configure templates
    Template(template::TemplateConfigArgs)
}
