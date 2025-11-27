
use std::fs;
use std::path::Path;

pub fn run(pattern: &str, path: &Path) -> anyhow::Result<()> {
    let content = fs::read_to_string(path)?;

    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
