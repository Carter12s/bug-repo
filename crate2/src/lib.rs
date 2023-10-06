pub fn get_working_dir() -> String {
    std::env::current_dir().unwrap().to_str().unwrap().to_string()
}