use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use std::path::Path;
use super::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SectionState {
    pub versionid: u32,
    pub version: String,
    pub url: String,
    pub local_path: String,
    pub checksum: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cache {
    pub sections: IndexMap<String, SectionState>,
}

impl Cache {
    pub fn load(root_dir: &str) -> Self {
        let cache_path = format!("{}/cache.json", root_dir);
        if Path::new(&cache_path).exists() {
            let data = std::fs::read_to_string(cache_path).expect("Failed to read cache");
            serde_json::from_str(&data).expect("Invalid cache format")
        } else {
            Cache {
                sections: IndexMap::new(),
            }
        }
    }

    pub fn save(&self, root_dir: &str) -> Result<(), AppError> {
        let cache_path = format!("{}/cache.json", root_dir);
        let data = serde_json::to_string_pretty(self).expect("Failed to serialize cache");
        std::fs::write(&cache_path, data).map_err(|e| {
            // Create a clear, human-readable message explaining the root cause
            let custom_msg = format!(
                "Cannot create 'cache.json' file in root directory '{}' because the directory does not exist or is unavailable. System error message:", 
                root_dir
            );
            // Construct a new IoError preserving the original error kind but updating the message
            std::io::Error::new(e.kind(), format!("{} {}", custom_msg, e))
        })?;
        Ok(())
    }
}

