use std::path::{Path, PathBuf};
use structopt::StructOpt;
use walkdir::WalkDir;

mod repostate;
use repostate::{get_repo_state, RepoState};

fn git_repos(path: &Path) -> impl Iterator<Item = git2::Repository> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_name().to_string_lossy().ends_with(".git"))
        .map(|e| e.path().to_owned())
        .map(|p| git2::Repository::open(p).unwrap())
}

fn print_changed(path: &Path) {
    for repo in git_repos(path).filter(|r| has_changes(r)) {
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

fn has_changes(repo: &git2::Repository) -> bool {
    let repo_unclean = get_repo_state(repo) != RepoState::Clean;
    let mut options = git2::StatusOptions::new();
    let file_changes = !repo
        .statuses(Some(options.include_ignored(false).include_untracked(true)))
        .unwrap()
        .is_empty();

    repo_unclean || file_changes
}

#[derive(StructOpt)]
struct Options {
    top_dir: PathBuf,
}

fn main() {
    let options = Options::from_args();
    print_changed(&options.top_dir);
}
