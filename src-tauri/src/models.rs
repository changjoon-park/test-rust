// lib.rs
use crate::models::SecurityReport;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub async fn run_all_security_checks() -> Result<SecurityReport, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    
    // Registry-based checks (fast, no admin required)
    let registry_checks = vec![
        security_checks::security_management::check_screen_saver_settings(),
        security_checks::security_management::check_autorun_settings(),
        security_checks::security_management::check_browser_temp_files_settings(),
        security_checks::security_management::check_remote_access_settings(),
    ];
    
    for check in registry_checks {
        match check {
            Ok(result) => results.push(result),
            Err(e) => eprintln!("Check failed: {}", e),
        }
    }
    
    // TODO: Add WMI-based checks
    // TODO: Add admin-required checks
    
    Ok(SecurityReport::new(results))
}

