use std::error::Error;
use std::process::Command;

fn get_diff() -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(["diff", "--staged", "--minimal"])
        .output()?;

    if !output.status.success() {
        return Err("Failed to execute git diff".into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let diff = get_diff()?;

    println!("{}", diff);
    Ok(())
}
