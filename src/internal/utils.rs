use crate::ROOT_DIR;

// Helper function to determine local storage path
pub fn derive_local_path(url_path: &str) -> String {
    let dirname = url_path.split('/').rev().nth(1).unwrap_or("default_dir");
    let filename = url_path.split('/').last().unwrap_or("default.nup");
    format!("{}/modules/{}/{}", ROOT_DIR, dirname, filename)
}

// Helper function to modify the path for modified INI
pub fn modified_path(url_path: &str) -> String {
    let path = url_path
        .split_once('/')
        .unwrap_or(("default_pt1", "default_pt2"))
        .1;
    format!("/{}", path)
}
