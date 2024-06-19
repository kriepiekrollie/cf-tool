use std::path::PathBuf;

pub fn path_relative_home(path: &PathBuf) -> Option<String> {
    match dirs::home_dir() {
        Some(home_dir) => match pathdiff::diff_paths(path, home_dir) {
            Some(diff) => Some(format!("~/{}", diff.display().to_string())),
            _ => None,
        }
        _ => None,
    }
}

pub fn path_relative_cwd(path: &PathBuf) -> Option<String> {
    match std::env::current_dir() {
        Ok(current_dir) => match pathdiff::diff_paths(path, current_dir) {
            Some(diff) => Some(format!("./{}", diff.display().to_string())),
            _ => None,
        }
        _ => None,
    }
}

/// Tries to represent a path in as short a string as possible.
pub fn path_shortest_repr(path: &PathBuf) -> String {
    let mut result: String = path.display().to_string();
    if let Some(p) = path_relative_home(path) {
        if p.len() < result.len() {
            result = p;
        }
    }
    if let Some(p) = path_relative_cwd(path) {
        if p.len() < result.len() {
            result = p;
        }
    }
    result
}
