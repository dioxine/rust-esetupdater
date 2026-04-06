use super::errors::AppError;
use clap::{CommandFactory, Parser};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub host: String,
    pub username: String,
    pub password: String,
    pub user_agent: String,
    pub root_dir: String,
    pub remote_main_sub_dir: Option<String>,
    pub remote_custom_dll_dir: Option<String>,
    pub remote_custom_modules_dir: Option<String>,
    pub local_main_sub_dir: Option<String>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, AppError> {
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}

#[derive(Parser, Debug)]
#[command(name = "rust-esetupdater")]
#[command(version, about, long_about = None)]
struct Args {
    /// Custom config file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    /// Override host URL
    #[arg(long)]
    host: Option<String>,

    /// Override root directory
    #[arg(long)]
    root_dir: Option<String>,
}

pub fn process_cfg() -> Result<Config, AppError> {
    let args = Args::parse();
    // Check config file existence
    if !Path::new(&args.config).exists() {
        eprintln!();
        eprintln!("⚠️  Config file not found: {}", args.config);
        eprintln!("ℹ️  Create 'config.toml' or specify via CLI:");
        eprintln!();
        Args::command().print_help().unwrap();
        eprintln!();
    }

    // Load config or defaults
    let mut config = if Path::new(&args.config).exists() {
        Config::load(&args.config)?
    } else {
        Config::default()
    };

    // Injecting default user_agent in case of no config file
    config.user_agent = "EEA Update (Windows; U; 64bit; BPC 12.1.2057.3; OS: 10.0.26100 SP 0.0 NT; HWF: FE1FF0BE-03BC-4DB3-AF7C-F7450021B4E4; PLOC ru_ru; PCODE 107.0.0; PAR -1; ATH -1; DC 0; PLID 3AC-9SP-9D9; SEAT 154b3474; RET 2103)".to_string();

    // Apply CLI overrides
    let final_config = Config {
        host: args.host.unwrap_or(config.host),
        root_dir: args.root_dir.unwrap_or(config.root_dir),
        ..config
    };

    if final_config.host.is_empty() && final_config.root_dir.is_empty() {
        return Err(AppError::EmptyConfig);
    }

    Ok(final_config)
}
