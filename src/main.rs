use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod repostate;
use repostate::get_repo_state;

mod repos;
use repos::{find_git_repos, has_changes};

fn print_changed(path: &Path) {
    let (oks, errs): (
        Vec<Result<git2::Repository, git2::Error>>,
        Vec<Result<git2::Repository, git2::Error>>,
    ) = find_git_repos(path).partition(Result::is_ok);

    for repo in oks
        .into_iter()
        .filter_map(Result::ok)
        .filter(|r| has_changes(r))
    {
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
