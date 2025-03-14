use std::fs;
use std::io::Error;

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

pub fn print_and_get_config(filename: &str) -> Result<Config, Error> {
    let result = fs::read_to_string(filename);
    match result {
        Ok(value) => {
            let config: Config = toml::from_str(&value).unwrap();

            println!("");
            println!("Local settings:");
            println!("  root_dir: {:#?}", config.local.root_dir);
            println!("  sub_dir: {:#?}", config.local.sub_dir);
            println!("  update_ver_new: {:#?}", config.local.update_ver_new);
            println!("  update_ver_old: {:#?}", config.local.update_ver_old);

            println!("");
            println!("Remote settings:");
            println!("  host: {:#?}", config.remote.host);
            println!("  host_path: {:#?}", config.remote.host_path);
            println!("  user: {:#?}", config.remote.user);
            println!("  password: {:#?}", config.remote.password);
            println!("  user_agent: {:#?}", config.remote.user_agent);

            println!("");
            println!("Main settings:");
            println!("  chosen platform(s): {:?}", config.settings.platforms);
            println!("");

            Ok(config)
        }
        Err(_) => Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Config file not found", 
        ))
    }
}
