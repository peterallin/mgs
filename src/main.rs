use std::path::PathBuf;
use structopt::StructOpt;

mod output;
mod repos;
mod repostate;

#[derive(StructOpt)]
struct Options {
    top_dir: PathBuf,
}

fn main() {
    let options = Options::from_args();
    output::print_changed(&options.top_dir);
}
