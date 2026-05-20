use std::path::PathBuf;
use tauri::Manager;

pub fn get_exe_dir(app: Option<&tauri::AppHandle>) -> PathBuf {
    if let Some(app) = app {
        if let Ok(dir) = app.path().executable_dir() {
            return dir;
        }
    }

    std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|parent| parent.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}
