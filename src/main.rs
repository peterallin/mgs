use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod repostate;
use repostate::get_repo_state;

mod repos;
use repos::{find_git_repos, has_changes};

fn print_statuses(statuses: git2::Statuses) {
    for s in statuses.iter() {
        println!("  {:?}: {:?}", s.path(), s.status());
    }
    println!();
}

fn ignored_and_untracked() -> git2::StatusOptions {
    let mut options = git2::StatusOptions::new();
    options.include_ignored(false).include_untracked(true);
    options
}

fn print_changed(path: &Path) {
    let (oks, errs) : (Vec<_>, Vec<_>)= find_git_repos(path).partition(Result::is_ok);

    for repo in oks
        .into_iter()
        .filter_map(Result::ok)
        .filter(|r| has_changes(r))
    {
        println!("{:?}: {:?}", repo.path(), get_repo_state(&repo),);
        match repo.statuses(Some(&mut ignored_and_untracked())) {
            Ok(statuses) => print_statuses(statuses),
            Err(e) => println!("Failed to get status of {}: {}", repo.path().display(), e),
        }
    }

    if !errs.is_empty() {
        // TODO: It would be nice if this could list the directories that we
        // were unable to treat as repositories along with the error message.
        // Most likely the error message will contain the path, but still...
        // I probably need to make an error type wrapping git2::Error.
        println!("The following erros occurred while querying repositories:");
        for e in errs.into_iter().filter_map(Result::err) {
            println!("  {}", e);
        }
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
