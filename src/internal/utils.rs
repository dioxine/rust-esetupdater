use super::errors::AppError;
use super::parser::ModuleInfo;
use super::parser::serialize_ini;
use indexmap::IndexMap;
use std::fs;
use std::fs::File;
use std::path::Path;

// use crate::{LOCAL_SUB_DIR, ROOT_DIR};

// Helper function to determine local storage path
pub fn derive_local_path(url_path: &str, root_dir: &str) -> String {
    let dirname = url_path.split('/').rev().nth(1).unwrap_or("default_dir");
    let filename = url_path.split('/').last().unwrap_or("default.nup");
    format!("{}/modules/{}/{}", root_dir, dirname, filename)
}

// Helper function to modify the path for modified INI
pub fn modified_path(url_path: &str) -> String {
    let path = url_path
        .split_once('/')
        .unwrap_or(("default_pt1", "default_pt2"))
        .1;
    format!("/{}", path)
}

// Helper function to create cache file
#[allow(unused)]
pub fn create_cache_file(path: &str) -> Result<(), AppError> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(path)?;
    Ok(())
}

// Helper function to save modified INI
pub fn save_modified_ini(data: &IndexMap<String, ModuleInfo>, root_dir: &str, local_sub_dir: &str) -> Result<(), AppError> {
    let serialized = serialize_ini(data)?;
    let ini_path = format!("{}{}/dll/update.ver", root_dir, local_sub_dir);

    if let Some(parent) = Path::new(&ini_path).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(ini_path, serialized)?;
    Ok(())
}

// Helper function to remove empty directories
pub fn remove_file_and_dir_if_empty(local_path: &str) -> Result<(), AppError> {
    let parent_dir = std::path::Path::new(local_path)
        .parent()
        .expect("Failed to get parent directory");

    fs::remove_file(local_path)
        .inspect(|_| log::info!("✅ Deleted file: {}", local_path))
        .ok();

    if parent_dir
        .read_dir()
        .map(|mut i| i.next().is_none())
        .unwrap_or(false)
    {
        fs::remove_dir(parent_dir)
            .inspect(|_| log::info!("✅ Deleted empty directory: {}", parent_dir.display()))
            .ok();
    }
    // Else: silently do nothing (directory has files)
    Ok(())
}
