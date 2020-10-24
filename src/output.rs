use crate::repos::{changes, find_git_repos, Change};
use crate::repostate::{get_repo_state, RepoState};
use itertools::chain;
use std::path::Path;

pub fn print_changed(path: &Path) {
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
            let added = count(&changes, |c| matches!(c, Change::Added(_)));
            let modified = count(&changes, |c| matches!(c, Change::Modified(_)));
            let removed = count(&changes, |c| matches!(c, Change::Removed(_)));
            let conflicted = count(&changes, |c| matches!(c, Change::Conflicted(_)));

            print!("{}: ", repo_path.display());
            if repo_state != RepoState::Clean {
                print!("{}, ", repo_state)
            }
            if added > 0 {
                print!("{} added ", added)
            };
            if modified > modified {
                print!("{} modified ", modified)
            };
            if removed > 0 {
                print!("{} removed ", removed)
            };
            if conflicted > 0 {
                print!("{} conflicted", conflicted)
            };
            println!();
        }
    }

    if !find_errs.is_empty() || !changes_errs.is_empty() {
        println!(
            "\n\nThe following problems occurred while looking for git repos and their statuses:"
        );

        for error in chain(
            find_errs.into_iter().filter_map(Result::err),
            changes_errs.into_iter().filter_map(Result::err),
        ) {
            println!("{:?}\n\n", error);
        }
    }
}

fn count<F>(changes: &Vec<Change>, f: F) -> usize
where
    F: Fn(&Change) -> bool,
{
    changes.iter().filter(|c| f(c)).count()
}
