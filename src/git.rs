use std::env;
use std::path::{Path, PathBuf};

use git2::{DiffOptions, Repository};

use crate::models::RepoFile;
use crate::utils::sort_repo_files_by_status;

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

pub fn get_repo() -> Result<Repository, &'static str> {
    let current_path = env::current_dir().map_err(|_| "Unable to get current directory")?;

    let repo_directory =
        find_git_directory(&current_path).ok_or("Unable to find .git directory")?;

    Repository::open(repo_directory).map_err(|_| "Error opening repository")
}

pub fn get_modified_files() -> Result<Vec<RepoFile>, &'static str> {
    let repo = get_repo().map_err(|_| "Error getting repository")?;

    let mut modified_files = Vec::new();

    let diffs = repo
        .diff_index_to_workdir(None, Some(DiffOptions::new().include_untracked(true)))
        .map_err(|_| "Error getting diff")?;

    diffs
        .foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    let status = delta.status();
                    modified_files.push(RepoFile {
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

    Ok(sort_repo_files_by_status(modified_files))
}
