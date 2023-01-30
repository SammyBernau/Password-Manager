use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

lazy_static! {
    static ref JSON_PATH: PathBuf = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("password_manager_json.json");
}

pub fn get_json_path() -> PathBuf {
    JSON_PATH.clone()
}
