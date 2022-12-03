pub fn read_file_vec(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path: std::path::PathBuf = [env!("CARGO_MANIFEST_DIR"), filename].into_iter().collect();
    Ok(std::fs::read_to_string(path)?)
}
