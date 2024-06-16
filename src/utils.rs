use std::path::PathBuf;
use std::fmt::Display;

pub fn path_relative_home(path: &PathBuf) -> Option<String> {
    match std::env::home_dir() {
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

pub fn get_problem_details_cwd() -> Option<(String, String)> {
    match std::env::current_dir() {
        Ok(current_dir) => {
            // assume user is in <cf_root>/contest/<contest_id>/<problem_id>
            let contest_id = current_dir.parent().and_then(|p| p.file_name().map(|s| format!("{:?}", s).to_string()));
            let problem_id = current_dir.file_name().map(|s| format!("{:?}", s).to_string());
            match (contest_id, problem_id) {
                (Some(c), Some(p)) => Some((c, p)),
                _ => None,
            }
        }
        Err(e) => None
    }
}
