use super::structs::Nups;
use std::collections::HashMap;

pub fn compare_old_with_new(
    old: &HashMap<String, HashMap<String, Option<String>>>,
    new: &HashMap<String, HashMap<String, Option<String>>>,
    platforms: Vec<String>,
) -> Vec<Nups> {
    let mut nups_paths: Vec<Nups> = vec![];
    for (key, el) in new {
        if old.contains_key(key)
            && el.contains_key("platform")
            && platforms.contains(&el["platform"].as_ref().unwrap())
        {
            if el["versionid"] != old[key]["versionid"] {
                let description = format!(
                    "Changes in module version: \n -> Module name: \"{key}\" \n -> Old versionID: \"{}\" \n -> New versionID: \"{}\" \n -> File: \"{}\"",
                    old[key]["versionid"].as_ref().unwrap(),
                    el["versionid"].as_ref().unwrap(),
                    el["file"].as_ref().unwrap()
                );

                match &el["file"] {
                    Some(value) => {
                        nups_paths.push(Nups {
                            path: value.to_string(),
                            description,
                        });
                    }
                    None => {
                        println!("nup url is missing in update.ver")
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
            match &el["file"] {
                Some(value) => {
                    nups_paths.push(Nups {
                        path: value.to_string(),
                        description,
                    });
                }
                None => {
                    println!("nup url is missing in update.ver")
                }
            }
        }
    }

    if old == new {
        println!(
            "There is no changes in \"update.ver\" file for chosen platforms. Nothing to download!"
        );
        return nups_paths;
    }
    nups_paths
}
