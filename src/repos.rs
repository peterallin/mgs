use std::path::Path;
use walkdir::WalkDir;

use crate::repostate::{get_repo_state, RepoState};

pub fn git_repos(path: &Path) -> impl Iterator<Item = git2::Repository> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_name().to_string_lossy().ends_with(".git"))
        .map(|e| e.path().to_owned())
        .map(|p| git2::Repository::open(p).unwrap())
}

pub fn has_changes(repo: &git2::Repository) -> bool {
    let repo_unclean = get_repo_state(repo) != RepoState::Clean;
    let mut options = git2::StatusOptions::new();
    let file_changes = !repo
        .statuses(Some(options.include_ignored(false).include_untracked(true)))
        .unwrap()
        .is_empty();

    repo_unclean || file_changes
}
