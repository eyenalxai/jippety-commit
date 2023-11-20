use std::env;
use std::path::{Path, PathBuf};

use git2::{Delta, DiffOptions, Repository};

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

fn get_repo() -> Result<Repository, &'static str> {
    let current_path = env::current_dir().map_err(|_| "Unable to get current directory")?;

    let repo_directory =
        find_git_directory(&current_path).ok_or("Unable to find .git directory")?;

    Repository::open(repo_directory).map_err(|_| "Error opening repository")
}

#[derive(Debug)]
struct File {
    status: Delta,
    path: PathBuf,
}

fn get_modified_files(repo: &Repository) -> Result<Vec<File>, &'static str> {
    let mut modified_files = Vec::new();

    let diffs = repo
        .diff_index_to_workdir(None, Some(DiffOptions::new().include_untracked(true)))
        .map_err(|_| "Error getting diff")?;

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
        .map_err(|_| "Error iterating over diff")?;

    Ok(modified_files)
}

fn main() {
    let repo = match get_repo() {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let modified_files = match get_modified_files(&repo) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    modified_files
        .iter()
        .for_each(|f| println!("{:?}: {}", f.status, f.path.display()))
}
