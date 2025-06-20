// src-tauri/src/lib.rs
mod window_manager;

use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{Window, Emitter};
use serde::Serialize;

#[tauri::command]
fn greet() -> String {
  let now = SystemTime::now();
  let epoch_ms = now.duration_since(UNIX_EPOCH).unwrap().as_millis();
  format!("Hello world from Rust! Current epoch: {}", epoch_ms)
}

#[derive(Serialize, Clone)]  // Clone 추가!
struct ProgressEvent {
    value: i32,
    message: Option<String>,
}

#[tauri::command]
async fn analyze_system(window: Window) -> Result<String, String> {
    window.emit("analyze_system_progress", ProgressEvent {
        value: 0,
        message: Some("시스템 정보 수집 중...".to_string()),
    }).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(500));

    window.emit("analyze_system_progress", ProgressEvent {
        value: 30,
        message: Some("CPU 정보 분석 중...".to_string()),
    }).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(500));

    window.emit("analyze_system_progress", ProgressEvent {
        value: 60,
        message: Some("메모리 사용량 확인 중...".to_string()),
    }).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(500));

    window.emit("analyze_system_progress", ProgressEvent {
        value: 90,
        message: Some("분석 완료 중...".to_string()),
    }).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(500));

    Ok("시스템 분석 완료: CPU 8코어, RAM 16GB, 디스크 500GB 사용 가능".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
        greet,
        analyze_system,
        window_manager::switch_to_main_window,
        window_manager::switch_to_login_window,
        window_manager::get_active_window,
        window_manager::check_session,
        window_manager::save_session,
        window_manager::clear_session
    ])
    .setup(|app| {
        // Initialize the appropriate window based on session state
        window_manager::initialize_app_window(&app.handle())
            .expect("Failed to initialize app window");
        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
