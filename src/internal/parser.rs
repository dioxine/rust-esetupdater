use super::errors::AppError;
use indexmap::IndexMap;
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, Visitor},
};
use serde_ini::to_string;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub version: String,
    pub versionid: u32,
    pub build: u32,
    #[serde(rename = "type")]
    pub type_: String,
    pub category: String,
    #[serde(default)] // Optional field (None if missing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<u32>,
    pub base: u32,
    pub platform: String,
    pub group: String,
    pub file: String,
    pub size: u32,
}

struct ModuleMapVisitor;

impl<'de> Visitor<'de> for ModuleMapVisitor {
    type Value = IndexMap<String, ModuleInfo>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an INI file with module sections")
    }
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: de::MapAccess<'de>,
    {
        let mut section_counter = 0;
        let mut map = IndexMap::new();
        let mut errors = Vec::new();

        while let Some(result) = access.next_entry::<String, ModuleInfo>().transpose() {
            section_counter += 1;
            match result {
                Ok((key, value)) if !key.starts_with("PICO_") => {
                    map.insert(key, value);
                }
                Err(e) => {
                    errors.push((section_counter, e));
                }
                _ => {} // Skip PICO sections
            }
        }

        if !errors.is_empty() {
            log::warn!("Encountered {} warnings during parsing:", errors.len());
            for (section_num, _) in errors {
                log::warn!(
                    "[Section number {}] - Contains unused PICO structure",
                    section_num
                );
            }
        }

        log::info!("Parsed {} sections successfully.", section_counter);

        Ok(map)
    }
}

pub fn serialize_ini(data: &IndexMap<String, ModuleInfo>) -> Result<String, AppError> {
    let serialized = to_string(data)?;
    Ok(serialized)
}

pub fn deserialize_remote_ini(data: &[u8]) -> Result<IndexMap<String, ModuleInfo>, AppError> {
    let mut deserializer = serde_ini::Deserializer::from_read(std::io::Cursor::new(data));
    let visitor = ModuleMapVisitor;
    let module_map: IndexMap<String, ModuleInfo> = deserializer.deserialize_map(visitor)?;
    Ok(module_map)
}
