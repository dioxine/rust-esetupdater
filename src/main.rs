use std::fs::File;

use indexmap::IndexMap;

mod internal;

use internal::config::process_cfg;
use internal::errors::AppError;
use internal::hyper::read_remote_ini_file;
use internal::parser::ModuleInfo;
use internal::parser::deserialize_remote_ini;
use internal::process::process_ini;

#[tokio::main]

async fn main() -> Result<(), AppError> {
    // Settings for logger
    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::ConfigBuilder::new()
                .set_time_offset_to_local()
                .unwrap()
                .build(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Debug,
            simplelog::ConfigBuilder::new()
                .set_time_offset_to_local()
                .unwrap()
                .build(),
            File::create("output.log")?,
        ),
    ])?;

    // // Getting config from file or CLI or close program

    let final_config = match process_cfg() {
        Ok(config) => config,
        Err(err) => {
            log::error!("{}", err);
            log::error!("Exiting program due to config error.");
            std::process::exit(1);
        }
    };

    let host = &final_config.host;
    let username = &final_config.username;
    let password = &final_config.password;
    let user_agent = &final_config.user_agent;
    let root_dir = &final_config.root_dir;
    let remote_main_sub_dir = &final_config.remote_main_sub_dir.unwrap_or("".to_string());
    let local_main_sub_dir = &final_config.local_main_sub_dir.unwrap_or("".to_string());
    let remote_custom_modules_dir = &final_config
        .remote_custom_modules_dir
        .unwrap_or("".to_string());

    // Read the INI file
    let ini_data = read_remote_ini_file(
        username,
        &password,
        format!("{}{}/dll/update.ver", host, remote_main_sub_dir).as_str(),
        user_agent,
    )
    .await?;

    // Deserialize the INI file
    let module_map: IndexMap<String, ModuleInfo> = deserialize_remote_ini(&ini_data)?;

    log::info!("Using host: {}", host);
    log::info!("Root dir: {}", root_dir);

    log::info!("Starting processing...");
    process_ini(
        &module_map,
        username,
        password,
        host,
        user_agent,
        root_dir,
        local_main_sub_dir,
        remote_custom_modules_dir,
    )
    .await?;

    log::info!("{}", "✅ Update completed successfully!");

    Ok(())
}
