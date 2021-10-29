pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(&pattern.to_lowercase()) {
            let result = writeln!(writer, "{}", line);
            match result {
                Ok(_) => (),
                Err(e) => println!("Error in writing: {:?}", e)
            }
        }
    }
}


#[test]
fn should_find_matches() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

#[test]
fn should_find_zero_matches() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "loram", &mut result);
    assert_eq!(result, b"");
}

#[test]
fn should_find_case_insensitive_matches() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "LOREM", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}