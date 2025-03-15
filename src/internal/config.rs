use log::{error, info};
use std::fs;

use super::logger::push_into_syslog;

use serde::Deserialize;
#[derive(Deserialize)]
pub struct Config {
    pub local: Local,
    pub remote: Remote,
    pub settings: Settings,
}

#[derive(Deserialize)]
pub struct Local {
    pub root_dir: String,
    pub sub_dir: String,
    pub update_ver_new: String,
    pub update_ver_old: String,
}

#[derive(Deserialize)]
pub struct Remote {
    pub host: String,
    pub host_path: String,
    pub user: String,
    pub password: String,
    pub user_agent: String,
}

#[derive(Deserialize)]
pub struct Settings {
    pub platforms: Vec<String>,
}

pub fn get_config_and_print_it(filename: &str) -> Config {
    let result = fs::read_to_string(filename);
    match result {
        Ok(value) => {
            let config: Config = toml::from_str(&value).unwrap();
            info!("");
            info!("Local settings:");
            info!("  root_dir: {:#?}", config.local.root_dir);
            info!("  sub_dir: {:#?}", config.local.sub_dir);
            info!("  update_ver_new: {:#?}", config.local.update_ver_new);
            info!("  update_ver_old: {:#?}", config.local.update_ver_old);

            info!("");
            info!("Remote settings:");
            info!("  host: {:#?}", config.remote.host);
            info!("  host_path: {:#?}", config.remote.host_path);
            info!("  user: {:#?}", config.remote.user);
            info!("  password: {:#?}", config.remote.password);
            info!("  user_agent: {:#?}", config.remote.user_agent);

            info!("");
            info!("Main settings:");
            info!("  chosen platform(s): {:?}", config.settings.platforms);
            info!("");
            config
        },
        Err(error) => {
            push_into_syslog(format!("Error reading config file {filename}: {}", error));
            error!("Error reading config file {filename}: {}", error);
            error!("You can specify a custom config file name as the first argument.");
            std::process::exit(1);
        }
    }
}
