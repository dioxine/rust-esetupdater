use indexmap::IndexMap;
mod internal;
use internal::network::read_remote_file;
use internal::parser::ModuleInfo;
use internal::parser::deserialize_remote_ini;
use internal::process::process_ini;

const HOST: &str = "http://madr0b0t.lol";
const USER_AGENT: &str = "EEA Update (Windows; U; 64bit; BPC 11.0.2044.0; OS: 10.0.26100 SP 0.0 NT; HWF: 921b979f-686d-4fa2-bebb-3ffe2ab877da; PLOC ru_ru; PCODE 107.0.0; PAR -1; ATH -1; DC 0; PLID 3AC-9SP-9D9; SEAT 154b3474; RET 2107)";
const ROOT_DIR: &str = "data";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the INI file
    let ini_data = read_remote_file(format!("{}/dll/update.ver", HOST).as_str())?;
    let module_map: IndexMap<String, ModuleInfo> = deserialize_remote_ini(&ini_data)?;

    // Process the INI data
    process_ini(&module_map)?;

    Ok(())
}
