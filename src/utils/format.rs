pub fn get_formatted_path(path: &str, max_length: usize) -> String {
    if path.len() > max_length {
        return format!("{}...", &path[..=max_length]);
    }

    path.to_string()
}
