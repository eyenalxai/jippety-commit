use crate::git::{get_modified_files, get_repo};

mod git;
mod models;
mod utils;

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
