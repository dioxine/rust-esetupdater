use std::fs::File;

use indexmap::IndexMap;

mod internal;

use internal::config::process_cfg;
use internal::network::read_remote_ini_file;
use internal::parser::ModuleInfo;
use internal::parser::deserialize_remote_ini;
use internal::process::process_ini;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Getting config from file or CLI or close program

    let final_config = match process_cfg() {
        Ok(config) => config,
        Err(err) => {
            log::error!("{}", err);
            log::error!("Exiting program due to config error.");
            std::process::exit(1);
        }
    };

    let host = &final_config.host;
    let user_agent = &final_config.user_agent;
    let root_dir = &final_config.root_dir;
    let local_sub_dir = &final_config.local_sub_dir.unwrap_or("".to_string());
    let remote_sub_dir = &final_config.remote_sub_dir.unwrap_or("".to_string());

    // Read the INI file
    let ini_data = read_remote_ini_file(
        format!("{}{}/dll/update.ver", host, remote_sub_dir).as_str(),
        user_agent,
    )?;

    // Deserialize the INI file
    let module_map: IndexMap<String, ModuleInfo> = deserialize_remote_ini(&ini_data)?;

    log::info!("Using host: {}", host);
    log::info!("Root dir: {}", root_dir);

    log::info!("Starting processing...");
    process_ini(&module_map, host, user_agent, root_dir, local_sub_dir)?;

    log::info!("{}", "✅ Update completed successfully!");

    // Getting remote and local sub directories
    // let remote_sub_dir = config.remote_sub_dir.unwrap_or("".to_string());
    // let local_sub_dir = config.local_sub_dir.unwrap_or("".to_string());

    Ok(())
}
