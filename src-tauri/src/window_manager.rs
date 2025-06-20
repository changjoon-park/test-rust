// src-tauri/src/window_manager.rs
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use std::path::Path;

/// Check user session (needs actual implementation)
fn check_user_session() -> Result<bool, Box<dyn std::error::Error>> {
    // TODO: Implement actual session validation logic
    // For now, just check if a session token file exists
    Ok(Path::new("session.token").exists())
}

/// Initialize app window on startup
pub fn initialize_app_window(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let has_session = check_user_session()?;

    if has_session {
        // Only create main window if session exists
        create_main_window(app)?;
    } else {
        // Only create login window if no session
        create_login_window(app)?;
    }

    Ok(())
}

/// Create login window
fn create_login_window(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    WebviewWindowBuilder::new(
        app,
        "login",
        WebviewUrl::App("/login".into())
    )
    .title("Login - Monori")
    .inner_size(960.0, 600.0)
    .center()
    .resizable(false)
    .maximizable(false)
    .visible(true)
    .build()?;

    Ok(())
}

/// Create main window
fn create_main_window(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let window = WebviewWindowBuilder::new(
        app,
        "main",
        WebviewUrl::App("/".into())  // Main page at root
    )
    .title("Monori")
    .inner_size(1920.0, 1080.0)
    .resizable(true)
    .maximizable(true)
    .visible(false)  // Start hidden to prevent flickering
    .build()?;

    // Maximize after creation
    window.maximize()?;

    // Then show the window
    window.show()?;

    Ok(())
}

/// Switch to main window after successful login
#[tauri::command]
pub async fn switch_to_main_window(app: AppHandle) -> Result<(), String> {
    // First, close the login window
    if let Some(login_window) = app.get_webview_window("login") {
        login_window.close()
            .map_err(|e| format!("Failed to close login window: {}", e))?;
    }

    // Small delay to ensure clean transition
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Check if main window already exists
    if let Some(main_window) = app.get_webview_window("main") {
        // If it exists, just show and focus it
        main_window.show()
            .map_err(|e| format!("Failed to show main window: {}", e))?;
        main_window.set_focus()
            .map_err(|e| format!("Failed to focus main window: {}", e))?;
    } else {
        // Create new main window
        create_main_window(&app)
            .map_err(|e| format!("Failed to create main window: {}", e))?;
    }

    Ok(())
}

/// Switch to login window on logout
#[tauri::command]
pub async fn switch_to_login_window(app: AppHandle) -> Result<(), String> {
    // Clear session first
    clear_session_internal()?;

    // Close main window
    if let Some(main_window) = app.get_webview_window("main") {
        main_window.close()
            .map_err(|e| format!("Failed to close main window: {}", e))?;
    }

    // Small delay
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // Create login window
    create_login_window(&app)
        .map_err(|e| format!("Failed to create login window: {}", e))?;

    Ok(())
}

/// Helper function to clear session
fn clear_session_internal() -> Result<(), String> {
    if Path::new("session.token").exists() {
        std::fs::remove_file("session.token")
            .map_err(|e| format!("Failed to clear session: {}", e))?;
    }
    Ok(())
}

/// Get currently active window
#[tauri::command]
pub fn get_active_window(app: AppHandle) -> Result<String, String> {
    if app.get_webview_window("main").is_some() {
        Ok("main".to_string())
    } else if app.get_webview_window("login").is_some() {
        Ok("login".to_string())
    } else {
        Err("No active window found".to_string())
    }
}

/// Check session status
#[tauri::command]
pub async fn check_session() -> Result<bool, String> {
    check_user_session()
        .map_err(|e| format!("Failed to check session: {}", e))
}

/// Save session after successful login
#[tauri::command]
pub async fn save_session(token: String) -> Result<(), String> {
    std::fs::write("session.token", token)
        .map_err(|e| format!("Failed to save session: {}", e))
}

/// Clear session
#[tauri::command]
pub async fn clear_session() -> Result<(), String> {
    clear_session_internal()
}
