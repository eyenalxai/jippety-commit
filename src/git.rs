use std::env;
use std::path::{Path, PathBuf};

use git2::{DiffOptions, Repository};

use crate::utils::diff_to_string;

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

pub fn get_repo_files_diffs() -> Result<Vec<String>, &'static str> {
    let repo = get_repo().map_err(|_| "Error getting repository")?;

    let mut opts = DiffOptions::new();
    opts.include_untracked(true).recurse_untracked_dirs(true);

    let head = repo.head().map_err(|_| "Error getting HEAD")?;
    let head_commit = head
        .peel_to_commit()
        .map_err(|_| "Error getting HEAD commit")?;
    let tree = head_commit.tree().map_err(|_| "Error getting HEAD tree")?;

    let diffs = repo
        .diff_tree_to_index(Some(&tree), None, Some(&mut opts))
        .map_err(|_| "Error getting diff between HEAD and index")?;

    let mut files_diffs = Vec::new();

    diffs
        .foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    if let Ok(diff) = repo.diff_tree_to_workdir_with_index(
                        Some(&tree),
                        Some(&mut DiffOptions::new().pathspec(path)),
                    ) {
                        if let Ok(diff_str) = diff_to_string(&diff) {
                            files_diffs.push(diff_str);
                        }
                    }
                }
                true
            },
            None,
            None,
            None,
        )
        .map_err(|_| "Error iterating over diffs")?;

    Ok(files_diffs)
}
