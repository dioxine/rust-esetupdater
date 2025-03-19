use log::{info, warn, error};
use std::fs::{File, copy, create_dir_all, exists, metadata};
use std::path::Path;

use super::helpers::local_path_fixer;

pub fn check_or_create_empty(local_path: &str, update_ver_old: &str) {
    let path = local_path_fixer(local_path, update_ver_old);
    let path2 = Path::new(&path);
    let _ = create_dir_all(path2.parent().unwrap());

    let is_exits = exists(&path).unwrap();
    info!("Checking if old .update.ver file exists...");

    // Checking with exists() before unwrapping metadata() is CRITICAL!

    if is_exits && metadata(&path).unwrap().len() != 0 {
        info!(
            "{} exists with {} bytes of length",
            path, metadata(&path).unwrap().len()
        );
        info!("Checking if there are fresh NUP-files to download");
    } else if is_exits && metadata(&path).unwrap().len() == 0 {
        warn!(
            "{} exists but it is empty. Starting full downloading of all NUP-files",
            path
        );
    } else {
        warn!(
            "{:#?} does not exist. Creating empty one and starting full downloading of all NUP-files",
            path2.parent().unwrap()
        );
        let _empty_file = File::create_new(path.as_str()).unwrap();
    }
}

pub fn copy_update_ver_file(
    local_directory: &str,
    update_ver_new: &str,
    update_ver_old: &str,
) {
    let from = local_path_fixer(local_directory, update_ver_new);
    let to = local_path_fixer(local_directory, update_ver_old);
    let _result = match copy(&from, &to) {
        Ok(_val) => {
            info!("Finalizing...");
            info!("File update.ver was copied:");
            info!("from path:{}", from);
            info!("to path:{}", to);
            info!("Finished successfully!");
            info!("-------------------------------------------------------------------------------------------------------------");
        }
        Err(err) => {
            error!("Error copying file: {}", err);
            std::process::exit(1);
        }
    };
}
