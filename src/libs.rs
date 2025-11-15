pub fn grrs(pattern: &String, content: String) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", pattern)
        }
    }
}
