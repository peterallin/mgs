#[derive(Debug, Eq, PartialEq)]
pub enum RepoState {
    Clean,
    Merging,
    Reverting,
    CherryPicking,
    Bisecting,
    Rebasing,
    ApplyingMailbox,
}

impl std::fmt::Display for RepoState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn get_repo_state(repo: &git2::Repository) -> RepoState {
    match repo.state() {
        git2::RepositoryState::Clean => RepoState::Clean,
        git2::RepositoryState::Merge => RepoState::Merging,
        git2::RepositoryState::Revert => RepoState::Reverting,
        git2::RepositoryState::RevertSequence => RepoState::Reverting,
        git2::RepositoryState::CherryPick => RepoState::CherryPicking,
        git2::RepositoryState::CherryPickSequence => RepoState::CherryPicking,
        git2::RepositoryState::Bisect => RepoState::Bisecting,
        git2::RepositoryState::Rebase => RepoState::Rebasing,
        git2::RepositoryState::RebaseInteractive => RepoState::Rebasing,
        git2::RepositoryState::RebaseMerge => RepoState::Rebasing, // TODO: Find out what RebaseMerge means
        git2::RepositoryState::ApplyMailbox => RepoState::ApplyingMailbox,
        git2::RepositoryState::ApplyMailboxOrRebase => RepoState::ApplyingMailbox,
    }
}
