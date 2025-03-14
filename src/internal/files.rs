use std::fs::{File, copy, create_dir_all, exists};
use std::path::Path;

use super::helpers::local_path_fixer;

pub fn check_or_create_empty(local_path: &str, update_ver_old: &str) {
    let path = local_path_fixer(local_path, update_ver_old);
    let path2 = Path::new(&path);
    let _ = create_dir_all(path2.parent().unwrap());

    let is_exits = exists(&path).unwrap();
    println!("Checking if old update.ver file exists...");
    if is_exits {
        println!("{} exists!", path);
    } else {
        println!(
            "{:#?} does not exist! Creating empty one.",
            path2.parent().unwrap()
        );
        let _empty_file = File::create_new(path.as_str()).unwrap();
    }
}

pub fn copy_update_ver_file(
    local_directory: &str,
    update_ver_new: &str,
    update_ver_old: &str,
) -> std::io::Result<()> {
    let from = local_path_fixer(local_directory, update_ver_new);
    let to = local_path_fixer(local_directory, update_ver_old);
    println!("File update.ver was copied:");
    println!("from path:{}", from);
    println!("to path:{}", to);
    copy(from, to)?;
    Ok(())
}
