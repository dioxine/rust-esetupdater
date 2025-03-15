use config::print_and_get_config;
use files::{check_or_create_empty, copy_update_ver_file};
use network::{download_nup_files, download_update_ver_file};
use process::compare_old_with_new;
use structs::{Credentials, New, Nups, Old, UpdateVer};

mod internal;
use internal::{config, files, network, process, structs};

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    let filename = args.nth(1).unwrap_or("config.toml".to_string());

    println!("\nUsing default config file: {}", filename);

    let config = match print_and_get_config(filename.as_str()) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("Error reading config file: {}", error);
            println!("You can specify a config file as the first argument.");
            std::process::exit(1);
        }
    };

    let root_dir = config.local.root_dir;
    let sub_dir = config.local.sub_dir;
    let local_path = root_dir.trim_end_matches('/').to_string() + "/" + sub_dir.trim_matches('/');

    let update_ver_new = config.local.update_ver_new;
    let update_ver_old = config.local.update_ver_old;

    let host = config.remote.host;
    let host_path = config.remote.host_path;
    let user = config.remote.user;
    let password = config.remote.password;
    let user_agent = config.remote.user_agent;

    let platforms = config.settings.platforms;

    let creds = Credentials::new(host, host_path, user, password, user_agent); // Create instanse of Credentials struct

    let mut update_old: Old = UpdateVer::new(); // Create instanse of UpdateOld sctruct 
    let mut update_new: New = UpdateVer::new(); // Create instanse of UpdateVer sctruct 

    download_update_ver_file(&local_path, &update_ver_new, &creds); // Download new update.ver

    check_or_create_empty(&local_path, &update_ver_old);

    update_old.deserialize(&local_path, &update_ver_old);
    update_new.deserialize(&local_path, &update_ver_new);

    let nups_paths: Vec<Nups> = compare_old_with_new(&update_old.map, &update_new.map, platforms);

    download_nup_files(nups_paths, &root_dir, &creds);

    copy_update_ver_file(&local_path, &update_ver_new, &update_ver_old)?;
    Ok(())
}
