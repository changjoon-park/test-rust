// src/lib.rs
mod models;
mod utils;
mod security_checks;

use crate::models::{SecurityReport, CheckResult};
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Clone, Serialize)]
struct ProgressPayload {
    current: usize,
    total: usize,
    message: String,
    current_check: Option<CheckResult>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn run_security_check(window: tauri::Window) -> Result<SecurityReport, String> {
    run_all_security_checks_with_progress(window)
        .await
        .map_err(|e| e.to_string())
}

pub async fn run_all_security_checks_with_progress(
    window: tauri::Window,
) -> Result<SecurityReport, Box<dyn std::error::Error>> {
    let mut results = Vec::new();
    
    // Define all checks with their descriptions
    let checks: Vec<(&str, Box<dyn Fn() -> Result<CheckResult, Box<dyn std::error::Error>>>)> = vec![
        ("화면보호기 설정 점검 중...", Box::new(|| security_checks::check_screen_saver_settings())),
        ("자동실행 설정 점검 중...", Box::new(|| security_checks::check_autorun_settings())),
        ("브라우저 임시파일 설정 점검 중...", Box::new(|| security_checks::check_browser_temp_files_settings())),
        ("원격 접속 설정 점검 중...", Box::new(|| security_checks::check_remote_access_settings())),
    ];
    
    let total_checks = checks.len();
    
    for (index, (description, check_fn)) in checks.into_iter().enumerate() {
        // Send progress update
        window.emit("check-progress", ProgressPayload {
            current: index + 1,
            total: total_checks,
            message: description.to_string(),
            current_check: None,
        }).ok();
        
        // Small delay to make progress visible
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        // Run the check
        match check_fn() {
            Ok(result) => {
                // Send result update
                window.emit("check-progress", ProgressPayload {
                    current: index + 1,
                    total: total_checks,
                    message: format!("{} 완료", description),
                    current_check: Some(result.clone()),
                }).ok();
                
                results.push(result);
            }
            Err(e) => {
                eprintln!("Check failed: {}", e);
                // Continue with other checks even if one fails
            }
        }
    }
    
    // TODO: Add WMI-based checks
    // TODO: Add admin-required checks
    
    Ok(SecurityReport::new(results))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, run_security_check])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
