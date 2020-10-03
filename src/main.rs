use std::path::{Path, PathBuf};
use structopt::StructOpt;
use walkdir::WalkDir;

fn dot_git_dirs(path: &Path) -> impl Iterator<Item = git2::Repository> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_name().to_string_lossy().ends_with(".git"))
        .map(|e| e.path().to_owned())
        .map(|p| git2::Repository::open(p).unwrap())
}

fn print_git_dirs(path: &Path) {
    for p in dot_git_dirs(path) {
        println!(
            "{:?}: {:?} -- {}",
            p.path(),
            p.state(),
            if !p.index().unwrap().is_empty() {
                "I"
            } else {
                ""
            }
        );
        for s in p
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
    print_git_dirs(&options.top_dir);
}
