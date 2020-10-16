use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod repostate;
use repostate::get_repo_state;

mod repos;
use repos::{find_git_repos, has_changes};

fn print_changed(path: &Path) {
    for repo in find_git_repos(path).filter(|r| has_changes(r)) {
        println!("{:?}: {:?}", repo.path(), get_repo_state(&repo),);
        for s in repo
            .statuses(Some(
                git2::StatusOptions::new()
                    .include_ignored(false)
                    .include_untracked(true),
            ))
            .unwrap()
            .iter()
        {
            println!("  {:?}: {:?}", s.path(), s.status());
        }
        println!();
    }
}

#[derive(StructOpt)]
struct Options {
    top_dir: PathBuf,
}

fn main() {
    let options = Options::from_args();
    print_changed(&options.top_dir);
}
