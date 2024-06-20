use anyhow::{Context, Result};

pub mod template;

mod test;
mod submit;
mod parse;
mod login;
mod gen;

pub use {
    gen::gen,
    test::test,
    submit::submit,
    parse::parse,
    login::login,
};

// TODO: Add a command to allow configuration of the <cf_root> folder.

pub fn source() -> Result<()> {
    let url = "https://www.github.com/kriepiekrollie/cf-tool";
    open::that(url)
        .with_context(|| format!("Failed to open link in browser: {}", url))?;
    Ok(())
}
