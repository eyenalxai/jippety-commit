use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(["diff", "--staged", "--minimal"])
        .output()?;

    if !output.status.success() {
        eprintln!("Failed to run git diff");
        std::process::exit(1);
    }

    println!("{}", String::from_utf8(output.stdout)?);
    Ok(())
}
