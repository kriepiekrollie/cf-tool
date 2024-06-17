use std::path::PathBuf;

pub fn config_dir_path() -> PathBuf {
    dirs::config_dir().unwrap().join("cf-tool")
}
pub fn config_file_path() -> PathBuf {
    config_dir_path().join("config.json")
}
pub fn session_file_path() -> PathBuf {
    config_dir_path().join("session.json")
}
