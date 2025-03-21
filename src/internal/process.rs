use super::structs::Nups;
use log::{error, info};
use std::collections::HashMap;

pub fn compare_and_get_nups_paths(
    old: &HashMap<String, HashMap<String, Option<String>>>,
    new: &HashMap<String, HashMap<String, Option<String>>>,
    platforms: Vec<String>,
) -> Option<Vec<Nups>> {
    let mut nups_paths: Vec<Nups> = vec![];
    for (key, el) in new {
        if old.contains_key(key)                                            // Checks old map has corresponging key from new map
            && el.contains_key("platform")                                  // Checks if module section has "platform" key
            && platforms.contains(&el["platform"].as_ref().unwrap())
        // Checking is platform key in map has value that
        // is equal to any of elements in "platform" vector of strings
        {
            if el["versionid"] != old[key]["versionid"] {
                //
                let description = format!(
                    "Changes in module version: \n -> Module name: \"{key}\" \n -> Old versionID: \"{}\" \n -> New versionID: \"{}\" \n -> File: \"{}\"",
                    old[key]["versionid"].as_ref().unwrap(),
                    el["versionid"].as_ref().unwrap(),
                    el["file"].as_ref().unwrap()
                );

                //DEBUG insertion
                if key.contains("REVERSE") {
                    println!("-------PROBLEMATIC KEY IN PROCESS-CHANGE {key}-------");
                    match &el["file"] {
                        Some(val) => println!("---PROBLEMATIC NUP-URL {}---", val),
                        None => error!("nup url is missing in update.ver"),
                    }
                }

                match &el["file"] {
                    Some(value) => {
                        nups_paths.push(Nups {
                            path: value.to_string(),
                            description,
                        });
                    }
                    None => {
                        error!("nup url is missing in update.ver")
                    }
                }
            }
        } else if !old.contains_key(key)
            && el.contains_key("platform")
            && platforms.contains(&el["platform"].as_ref().unwrap())
        {
            let description = format!(
                "New module in update.ver: \n -> Module name: \"{}\" \n -> File: \"{}\"",
                key,
                el["file"].as_ref().unwrap()
            );

            //DEBUG insertion
            if key.contains("REVERSE") {
                println!("-------PROBLEMATIC KEY IN PROCESS-CHANGE {key}-------");
                match &el["file"] {
                    Some(val) => println!("---PROBLEMATIC NUP-URL {}---", val),
                    None => error!("nup url is missing in update.ver"),
                }
            }

            match &el["file"] {
                Some(value) => {
                    nups_paths.push(Nups {
                        path: value.to_string(),
                        description,
                    });
                }
                None => {
                    error!("nup url is missing in update.ver")
                }
            }
        }
    }

    println!(
        "Parsed update.ver successfully. Found {} NUP-modules for chosen platform(s).",
        nups_paths.len()
    );

    if old == new {
        println!(
            "There is no changes in \"update.ver\" file for chosen platforms. Nothing to download."
        );
        info!(
            "There is no changes in \"update.ver\" file for chosen platforms. Nothing to download."
        );
        return None;
    }
    Some(nups_paths)
}
