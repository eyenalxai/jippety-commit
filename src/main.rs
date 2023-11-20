use crate::git::get_modified_files;

mod git;
mod models;
mod utils;

fn main() {
    let modified_files = match get_modified_files() {
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
