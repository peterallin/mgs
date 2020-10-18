use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub enum Change {
    Added(String),
    Modified(String),
    Removed(String),
    Conflicted(String),
}

pub fn find_git_repos(path: &Path) -> impl Iterator<Item = Result<git2::Repository, git2::Error>> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_name().to_string_lossy().ends_with(".git"))
        .map(|e| e.path().to_owned())
        .map(git2::Repository::open)
}

pub fn changes(repo: &git2::Repository) -> Result<Vec<Change>, git2::Error> {
    Ok(repo
        .statuses(Some(&mut unignored_and_untracked()))?
        .iter()
        .map(to_change)
        .filter_map(|c| c)
        .collect())
}

fn to_change(status_entry: git2::StatusEntry) -> Option<Change> {
    let status = status_entry.status();
    let path = String::from_utf8_lossy(status_entry.path_bytes()).to_string();
    if status.is_wt_new() || status.is_index_new() {
        Some(Change::Added(path))
    } else if status.is_wt_modified() || status.is_index_modified() {
        Some(Change::Modified(path))
    } else if status.is_wt_deleted() || status.is_index_deleted() {
        Some(Change::Removed(path))
    } else if status.is_conflicted() {
        Some(Change::Conflicted(path))
    }
    // TODO: Type changed * 2, renamed * 2, ignored (skip?)
    else {
        None
    }
}

pub fn unignored_and_untracked() -> git2::StatusOptions {
    let mut options = git2::StatusOptions::new();
    options.include_ignored(false).include_untracked(true);
    options
}
