use super::helpers::local_path_fixer;
use serde_ini::from_str as from_str_ini;
use std::collections::HashMap;
use std::fs::read_to_string;

pub trait UpdateVer {
    fn new() -> Self;
    fn deserialize(&mut self, local_path: &str, update_ver_file_with_path: &str);
}

pub struct Old {
    pub map: HashMap<String, HashMap<String, Option<String>>>,
}

impl UpdateVer for Old {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    fn deserialize(&mut self, local_path: &str, update_ver_file_with_path: &str) {
        let path = local_path_fixer(local_path, update_ver_file_with_path);
        let ini_string = read_to_string(path).expect("Failed to read file");

        let ini: HashMap<String, HashMap<String, Option<String>>> = match from_str_ini(&ini_string)
        {
            Ok(data) => data,
            Err(err) => panic!("Error while parsing ini-string: {}", err),
        };
        self.map = ini;
    }
}

pub struct New {
    pub map: HashMap<String, HashMap<String, Option<String>>>,
}

impl UpdateVer for New {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn deserialize(&mut self, local_path: &str, update_ver_file_with_path: &str) {
        let path = local_path_fixer(local_path, update_ver_file_with_path);
        let ini_string = read_to_string(path).expect("Failed to read file");

        let ini: HashMap<String, HashMap<String, Option<String>>> = match from_str_ini(&ini_string)
        {
            Ok(data) => data,
            Err(err) => panic!("Error while parsing ini-string: {}", err),
        };
        self.map = ini;
    }
}

pub struct Credentials {
    pub host: String,
    pub host_path: String,
    pub user: String,
    pub password: String,
    pub user_agent: String,
}

impl Credentials {
    pub fn new(
        host: String,
        host_path: String,
        user: String,
        password: String,
        user_agent: String,
    ) -> Self {
        Self {
            host,
            host_path,
            user,
            password,
            user_agent,
        }
    }
}
pub struct Nups {
    pub path: String,
    pub description: String,
}
