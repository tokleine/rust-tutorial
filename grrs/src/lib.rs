pub fn find_matches(line: &str, pattern: &str , mut writer: impl std::io::Write) {
    if line.contains(pattern) {
        match write!(writer, "{}", line){
            Ok(_) => {},
            Err(e) => eprintln!("Error: {}", e),
        };
    }
}