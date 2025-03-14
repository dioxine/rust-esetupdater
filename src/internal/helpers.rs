use super::structs::Credentials;
use std::path::Path;

pub fn local_path_fixer(first_part: &str, second_part: &str) -> String {
    //clean and fix local pathes
    let path = Path::new(second_part.trim_start_matches("/"));
    first_part.trim_end_matches("/").to_string() + "/" + path.to_str().unwrap()
}

pub fn remote_path_fixer(first_part: &Credentials, second_part: &str) -> String {
    //check is update.var file is going to be downloaded
    let host_path: &str;
    if second_part.contains("update.ver") {
        host_path = first_part.host_path.as_str();
    } else {
        host_path = "";
    }

    let url = Path::new(second_part.trim_start_matches("/"));

    //clean and fix remote pathes
    if host_path != "" {
        first_part.host.trim_end_matches("/").to_string()
            + "/"
            + host_path.trim_matches('/')
            + "/"
            + url.to_str().unwrap().trim_start_matches('/')
    } else {
        first_part.host.trim_end_matches("/").to_string()
            + "/"
            + url.to_str().unwrap().trim_start_matches('/')
    }
}
