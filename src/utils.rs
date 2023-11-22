use git2::Diff;

use crate::git::get_repo_files_diffs;

pub fn diff_to_string(diff: &Diff) -> Result<String, &'static str> {
    let mut patch_str = String::new();
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        patch_str.push_str(std::str::from_utf8(line.content()).unwrap());
        true
    })
    .map_err(|_| "Error printing patch")?;

    Ok(patch_str)
}

pub fn concatenate_diffs(diff_strings: &Vec<String>) -> String {
    diff_strings.join("\n")
}

pub fn get_concatenated_diffs() -> Result<String, &'static str> {
    let diffs = get_repo_files_diffs()?;

    Ok(concatenate_diffs(&diffs))
}
