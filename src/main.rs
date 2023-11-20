use git2::{Delta, DiffDelta, DiffOptions, Repository, Status, StatusOptions};
use std::env;
use std::path::{Display, Path, PathBuf};

fn find_git_directory(start_path: &Path) -> Option<PathBuf> {
    let mut current_path = start_path.to_path_buf();

    for _ in 0..50 {
        if current_path.join(".git").is_dir() {
            return Some(current_path);
        }
        if !current_path.pop() {
            return None;
        }
    }

    None
}

fn get_repo() -> Repository {
    let current_path = env::current_dir().expect("Error getting current directory");
    let repo_directory = find_git_directory(&current_path).expect("Unable to find .git directory");
    Repository::open(repo_directory).expect("Error opening repository")
}
#[derive(Debug)]
struct File {
    status: Delta,
    path: PathBuf,
}

fn get_modified_files(repo: &Repository) -> Vec<File> {
    let mut modified_files = Vec::new();

    let diffs = repo
        .diff_index_to_workdir(None, Some(DiffOptions::new().include_untracked(true)))
        .expect("Error getting diffs");

    diffs
        .foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    let status = delta.status();
                    modified_files.push(File {
                        status,
                        path: path.to_path_buf(),
                    });
                }
                true
            },
            None,
            None,
            None,
        )
        .expect("Error iterating over diffs");

    modified_files
}

fn main() {
    let repo = get_repo();

    let modified_files = get_modified_files(&repo);

    modified_files.iter().for_each(|f| println!("{:?}", f))
}
