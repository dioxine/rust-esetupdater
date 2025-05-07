use crate::{ROOT_DIR, HOST};
use super::cache::{Cache, SectionState};
use super::errors::AppError;
use super::network::download_file;
use super::parser::{ModuleInfo, serialize_ini};
use super::utils::{derive_local_path, modified_path};

use indexmap::IndexMap;
use std::collections::HashMap;

pub fn process_ini(ini_data: &IndexMap<String, ModuleInfo>) -> Result<(), AppError> {
    let old_cache = Cache::load();
    let mut new_cache = Cache {
        sections: HashMap::new(),
    };
    let mut orphans = Vec::new();

    let mut modified_ini_data: IndexMap<String, ModuleInfo> = IndexMap::new();

    // Process current sections
    for (section_name, info) in ini_data {
        let url = format!("{}{}", HOST, info.file);
        let local_path = derive_local_path(&info.file); // Implement this

        modified_ini_data.insert(
            section_name.to_string(),
            ModuleInfo {
                version: info.version.to_string(),
                versionid: info.versionid,
                build: info.build,
                type_: info.type_.to_string(),
                category: info.category.to_string(),
                level: info.level, // Optional field
                base: info.base,
                platform: info.platform.to_string(),
                group: info.group.to_string(),
                file: modified_path(&local_path),
                size: info.size,
            },
        );

        let current_state = SectionState {
            versionid: info.versionid,
            version: info.version.clone(),
            url: url.clone(),
            local_path: local_path.clone(),
            checksum: None,
        };

        // Check if we need to download
        let should_download = match old_cache.sections.get(section_name) {
            Some(prev) => {
                prev.versionid != current_state.versionid
                    || prev.version != current_state.version
                    || prev.url != current_state.url
            }
            None => true,
        };

        if should_download {
            if let Err(e) = download_file(&url, &local_path) {
                eprintln!("Download failed: {}", e);
                continue;
            }
            println!("Downloaded: {}", local_path);
            // current_state.checksum = Some(calculate_checksum(&local_path));
        }

        new_cache
            .sections
            .insert(section_name.clone(), current_state);
    }

    // Find and clean up orphans
    for (old_section, old_state) in &old_cache.sections {
        if !new_cache.sections.contains_key(old_section) {
            orphans.push(old_state);
        }
    }

    // Delete orphaned files
    for orphan in orphans {
        if let Err(e) = std::fs::remove_file(&orphan.local_path) {
            eprintln!("Failed to delete {}: {}", orphan.local_path, e);
        } else {
            println!("Cleaned up: {}", orphan.local_path);
        }
    }

    new_cache.save();

    // Serialize back to INI format
    let serialized = serialize_ini(&modified_ini_data)?;
    let dll_path = format!("{}/dll/update.ver", ROOT_DIR);

    // Create parent directories if needed
    if let Some(parent) = std::path::Path::new(&dll_path).parent() {
        // println!("Creating parent directory: {:?}", url);
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(dll_path, &serialized)?;

    Ok(())
}
