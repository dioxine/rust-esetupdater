use log::info;

pub fn get_command_line_args() -> String {
    let mut args = std::env::args();
    let filename = args.nth(1).unwrap_or("config.toml".to_string());

    info!(
        "-------------------------------------------------------------------------------------------------------------"
    );
    info!("Program starts!");
    info!("Using default config file: {}", filename);
    filename
}
