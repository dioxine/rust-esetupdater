use super::cache::{Cache, SectionState};
use super::errors::AppError;
use super::network::download_file;
use super::parser::ModuleInfo;
use super::utils::{
    derive_local_path, modified_path, remove_file_and_dir_if_empty, save_modified_ini,
};
// use crate::HOST;

use indexmap::IndexMap;

pub fn process_ini(
    ini_data: &IndexMap<String, ModuleInfo>,
    host: &str,
    user_agent: &str,
    root_dir: &str,
    local_sub_dir: &str,
) -> Result<(), AppError> {
    let old_cache = Cache::load(root_dir);
    let mut new_cache = Cache {
        sections: IndexMap::new(),
    };

    let mut modified_ini_data: IndexMap<String, ModuleInfo> = IndexMap::new();

    // Process current sections
    for (section_name, info) in ini_data {
        let url = format!("{}{}", host, info.file);
        let local_path = derive_local_path(&info.file, root_dir); // Implement this

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
                // For debugging diffs
                if prev.versionid != current_state.versionid
                    || prev.version != current_state.version
                {
                    log::debug!(
                        "----------------------------DIFF STATE BEGINS--------------------------------------"
                    );
                    log::debug!("Prev: {:#?} - Current: {:#?}", prev, current_state);
                    log::debug!(
                        "----------------------------DIFF STATE ENDS----------------------------------------"
                    );
                }
                prev.versionid != current_state.versionid || prev.version != current_state.version // True if differs
            }
            None => {
                // For debugging absents
                log::debug!(
                    "----------------------------ABSENT STATE BEGINS--------------------------------------"
                );
                log::debug!("New: {:#?}", current_state);
                log::debug!(
                    "----------------------------ABSENT STATE ENDS----------------------------------------"
                );
                true
            } // True if absent
        };

        if should_download {
            // for debugging REVERSE sections purposes
            if section_name.contains("REVERSE") {
                log::debug!("REVERSE section should be downloaded?: {}", should_download);
                log::debug!("REVERSE section name: {}", section_name);
                log::debug!("REVERSE section info: {:#?}", info);
            }
            // Downloader of files
            if let Err(e) = download_file(&url, &local_path, user_agent) {
                log::error!("❌ Download failed: {}", e);
                // Remove absent section from modified INI data
                modified_ini_data.swap_remove(section_name);
                continue;
            } else {
                log::info!("✅ Downloaded: {}", local_path);
                log::debug!("✅ Downloaded: {}", local_path);
                log::debug!("");
            }
            // current_state.checksum = Some(calculate_checksum(&local_path));
        }

        new_cache
            .sections
            .insert(section_name.clone(), current_state);
    }

    log::info!("-----------------------------🧹🧹🧹🧹🧹🧹🧹🧹🧹-----------------------------");
    log::info!("Checking for orphaned files and directories and performing cleanup!");

    // Find oprhaned sections and delete orphaned files with their parent directories one level up
    old_cache
        .sections
        .iter()
        .filter_map(|(old_section, old_state)| {
            if !new_cache.sections.contains_key(old_section) {
                Some(old_state) // Filling orphans vector
            } else {
                // Additional check for versionid field even section with same name exists
                if old_cache.sections.get(old_section).unwrap().versionid
                    != new_cache.sections.get(old_section).unwrap().versionid
                    || old_cache.sections.get(old_section).unwrap().local_path
                        != new_cache.sections.get(old_section).unwrap().local_path
                {
                    log::debug!("Section: {:#?}", old_section);
                    log::debug!(
                        "Changes detected in versionid(old <-> new): {} <-> {}",
                        old_cache.sections.get(old_section).unwrap().versionid,
                        new_cache.sections.get(old_section).unwrap().versionid
                    );
                    return Some(old_state);
                }
                None
            }
        })
        // Propagate errors upwards if remove_file_and_dir_if_empty fails
        .try_for_each(|orphan| remove_file_and_dir_if_empty(&orphan.local_path))?;

    new_cache.save(root_dir);

    // Serialize and same back modified INI

    save_modified_ini(&modified_ini_data, root_dir, local_sub_dir)?;

    Ok(())
}
