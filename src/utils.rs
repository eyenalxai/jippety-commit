use git2::Diff;

pub fn patch_to_string(patch: &Diff) -> Result<String, &'static str> {
    let mut patch_str = String::new();
    patch
        .print(git2::DiffFormat::Patch, |_, _, line| {
            patch_str.push_str(std::str::from_utf8(line.content()).unwrap());
            true
        })
        .map_err(|_| "Error printing patch")?;

    Ok(patch_str)
}
