use crate::repos::{changes, find_git_repos, Change};
use crate::repostate::{get_repo_state, RepoState};
use anyhow::{anyhow, Context};
use colored::*;
use itertools::chain;
use std::path::Path;

pub fn print_changed(path: &Path) -> anyhow::Result<()> {
    let (oks, find_errs): (Vec<_>, Vec<_>) = find_git_repos(path).partition(Result::is_ok);
    let (oks, changes_errs): (Vec<_>, Vec<_>) = oks
        .into_iter()
        .filter_map(Result::ok)
        .map(|repo| {
            changes(&repo).map(|changes| (repo.path().to_owned(), get_repo_state(&repo), changes))
        })
        .partition(Result::is_ok);

    let top_path = dunce::canonicalize(path)
        .with_context(|| format!("Failed to canonicalize {}", path.display()))?
        .parent()
        .unwrap_or_else(|| Path::new("/"))
        .to_owned();
    for (repo_path, repo_state, changes) in oks.into_iter().filter_map(Result::ok) {
        if repo_state != RepoState::Clean || !changes.is_empty() {
            let relative_path = repo_path.strip_prefix(&top_path)?.parent().ok_or_else(|| {
                anyhow!(
                    "{} is a git repository but has no parent",
                    repo_path.display()
                )
            })?;

            let last_part = relative_path.file_name().unwrap();
            let start = relative_path.parent().unwrap();
            print!(
                "{}{}{}: ",
                correct_separators(&start.display().to_string()),
                std::path::MAIN_SEPARATOR,
                last_part.to_string_lossy().bright_cyan()
            );

            let added = count(&changes, |c| matches!(c, Change::Added(_)));
            let modified = count(&changes, |c| matches!(c, Change::Modified(_)));
            let removed = count(&changes, |c| matches!(c, Change::Removed(_)));
            let conflicted = count(&changes, |c| matches!(c, Change::Conflicted(_)));

            if repo_state != RepoState::Clean {
                print!("{}, ", repo_state)
            }
            if added > 0 {
                print!("{}", format!("{}+ ", added).green());
            };
            if removed > 0 {
                print!("{}", format!("{}- ", removed).bright_red());
            };
            if modified > 0 {
                print!("{}", format!("{}M ", modified).yellow());
            };
            if conflicted > 0 {
                print!("{}", format!("{}C ", conflicted).purple());
            };
            println!();
        }
    }

    if !find_errs.is_empty() || !changes_errs.is_empty() {
        println!(
            "{}",
            "\n\nThe following problems occurred while looking for git repos and their statuses:"
                .red()
        );

        for error in chain(
            find_errs.into_iter().filter_map(Result::err),
            changes_errs.into_iter().filter_map(Result::err),
        ) {
            println!("{}", format!("{:?}\n\n", error).red());
        }
    }

    Ok(())
}

fn correct_separators(path: &str) -> String {
    let sep = std::path::MAIN_SEPARATOR.to_string();
    path.replace("/", &sep).replace("\\", &sep)
}

fn count<F>(changes: &[Change], f: F) -> usize
where
    F: Fn(&Change) -> bool,
{
    changes.iter().filter(|c| f(c)).count()
}
