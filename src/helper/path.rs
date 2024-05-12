use std::path::PathBuf;

// Helper function to construct file paths
pub fn construct_file_path(relative_path: &str) -> String {
    let mut path_buf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_buf.push(relative_path);
    path_buf.to_str().unwrap().to_string()
}
