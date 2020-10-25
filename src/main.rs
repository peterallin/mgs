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
    ansi_term::enable_ansi_support().with_context("Failed to enable ANSI color support")?;

    let options = Options::from_args();
    output::print_changed(&options.top_dir)?;
    Ok(())
}
