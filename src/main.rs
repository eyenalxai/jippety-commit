use crate::utils::get_concatenated_diffs;

mod git;
mod utils;

fn main() {
    let patches = match get_concatenated_diffs() {
        Ok(patches) => patches,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("{}", patches)
}
