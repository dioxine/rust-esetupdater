use crate::ROOT_DIR;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

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
    pub sections: HashMap<String, SectionState>,
}

impl Cache {
    pub fn load() -> Self {
        let cache_path = format!("{}/cache.json", ROOT_DIR);
        if Path::new(&cache_path).exists() {
            let data = std::fs::read_to_string(cache_path).expect("Failed to read cache");
            serde_json::from_str(&data).expect("Invalid cache format")
        } else {
            Cache {
                sections: HashMap::new(),
            }
        }
    }

    pub fn save(&self) {
        let cache_path = format!("{}/cache.json", ROOT_DIR);
        let data = serde_json::to_string_pretty(self).expect("Failed to serialize cache");
        std::fs::write(&cache_path, data).expect("Failed to write cache");
    }
}
