// Generate shell completions

use clap::CommandFactory;
use clap_complete::{Shell, generate};
use miette::Result;
use std::io;

pub fn run(shell: Shell) -> Result<()> {
    let mut cmd = crate::Cli::command();
    let bin_name = cmd.get_name().to_string();

    generate(shell, &mut cmd, bin_name, &mut io::stdout());

    Ok(())
}
