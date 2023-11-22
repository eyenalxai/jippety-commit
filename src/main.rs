use crate::git::get_repo_files_diffs;

mod git;
mod utils;

fn main() {
    let diffs = match get_repo_files_diffs() {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    diffs.iter().for_each(|f| {
        println!("{}: \n\n\n", f);
    })
}
