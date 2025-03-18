mod internal;

use internal::*;

use args::get_command_line_args;
use config::get_config_and_print_it;
use files::{check_or_create_empty, copy_update_ver_file};
use logger;
use network::{download_nup_files, download_update_ver_file};
use process::compare_and_get_nups_paths;
use structs::{Credentials, New, Old, UpdateVer};

fn main() -> std::io::Result<()> {
    logger::init();

    let filename = get_command_line_args();

    let config = get_config_and_print_it(filename.as_str());

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

    let creds = Credentials::new(host, host_path, user, password, user_agent);

    let mut update_old: Old = UpdateVer::new();
    let mut update_new: New = UpdateVer::new();

    download_update_ver_file(&local_path, &update_ver_new, &creds);

    check_or_create_empty(&local_path, &update_ver_old);

    update_old.deserialize(&local_path, &update_ver_old);
    update_new.deserialize(&local_path, &update_ver_new);

    if let Some(nups_paths) =
        compare_and_get_nups_paths(&update_old.map, &update_new.map, platforms)
    {
        download_nup_files(nups_paths, &root_dir, &creds);
    }

    copy_update_ver_file(&local_path, &update_ver_new, &update_ver_old);
    Ok(())
}
