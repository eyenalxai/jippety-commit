use crate::git::get_repo_files_diffs;

mod git;
mod utils;

fn main() {
    let modified_files = match get_repo_files_diffs() {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    modified_files.iter().for_each(|f| {
        println!("{}: \n\n\n", f);
    })
}
