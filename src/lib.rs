
pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            let result = writeln!(writer, "{}", line);
            match result {
                Ok(_) => (),
                Err(e) => println!("Error in writing: {:?}", e)
            }
        }
    }
}