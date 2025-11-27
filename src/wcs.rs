use std::fs;
use std::path::Path;

pub fn run(path: &Path) -> anyhow::Result<()> {
    let content = fs::read_to_string(path)?;

    let new_lines = content.lines().count();
    let mut word_count = 0;

    for line in content.lines() {
        word_count += line.split_whitespace().count();
    }

    println!("new lines: {} word count: {}", new_lines, word_count);

    Ok(())
}
