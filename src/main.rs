use itertools::chain;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod repostate;
use repostate::{get_repo_state, RepoState};

mod repos;
use repos::{changes, find_git_repos};

fn print_changed(path: &Path) {
    let (oks, find_errs): (Vec<_>, Vec<_>) = find_git_repos(path).partition(Result::is_ok);
    let (oks, changes_errs): (Vec<_>, Vec<_>) = oks
        .into_iter()
        .filter_map(Result::ok)
        .map(|repo| {
            changes(&repo).map(|changes| (repo.path().to_owned(), get_repo_state(&repo), changes))
        })
        .partition(Result::is_ok);

    for (repo_path, repo_state, changes) in oks.into_iter().filter_map(Result::ok) {
        if repo_state != RepoState::Clean || !changes.is_empty() {
            println!("{}: {}", repo_path.display(), repo_state);
            for change in changes {
                println!("  {:?}", change)
            }
            println!();
        }
    }

    if !find_errs.is_empty() || !changes_errs.is_empty() {
        println!(
            "\n\nThe following problems occurred while looking for git repos and their statuses:"
        );

        for (path, error) in chain(
            find_errs.into_iter().filter_map(Result::err),
            changes_errs.into_iter().filter_map(Result::err),
        ) {
            println!("  {}: {}", path.display(), error.message());
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
