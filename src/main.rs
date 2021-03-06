#[cfg(target_os = "windows")]
use anyhow::anyhow;

use anyhow::Context;
use std::path::PathBuf;
use structopt::StructOpt;

mod output;
mod repos;
mod repostate;

#[derive(StructOpt)]
struct Options {
    top_dir: PathBuf,
}

fn main() -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support()
        .map_err(|e| anyhow!("Failed to enable ANSI color support, error code: {}", e))?;

    let options = Options::from_args();
    output::print_changed(&options.top_dir)
        .with_context(|| format!("Failed to print changes of {}", options.top_dir.display()))?;
    Ok(())
}
