// src-tauri/src/window_manager.rs
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TestUser {
    email: &'static str,
    password: &'static str,
    name: &'static str,
}

// Test user account
const TEST_USER: TestUser = TestUser {
    email: "test@example.com",
    password: "password123",
    name: "Test User",
};

/// Get the session file path in the app data directory
fn get_session_file_path(app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let app_dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&app_dir)?;
    Ok(app_dir.join("session.token"))
}

/// Check user session (needs actual implementation)
fn check_user_session(app: &AppHandle) -> Result<bool, Box<dyn std::error::Error>> {
    // TODO: Implement actual session validation logic
    // For now, just check if a session token file exists
    let session_path = get_session_file_path(app)?;
    let session_exists = session_path.exists();
    println!("Checking session file at {:?}: exists = {}", session_path, session_exists);

    if session_exists {
        if let Ok(content) = std::fs::read_to_string(&session_path) {
            println!("Session file content: {}", content);
        }
    }

    Ok(session_exists)
}

/// Initialize app window on startup
pub fn initialize_app_window(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let has_session = check_user_session(app)?;
    println!("App startup - Session exists: {}", has_session);

    if has_session {
        // Only create main window if session exists
        println!("Creating main window");
        create_main_window(app)?;
    } else {
        // Only create login window if no session
        println!("Creating login window");
        create_login_window(app)?;
    }

    Ok(())
}

/// Create login window
fn create_login_window(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("create_login_window: Creating login window");

    // In development, we need to use the dev server URL
    let url = if cfg!(debug_assertions) {
        WebviewUrl::External("http://localhost:3000/login".parse().unwrap())
    } else {
        WebviewUrl::App("login".into())
    };

    println!("create_login_window: Using URL: {:?}", url);

    let _window = WebviewWindowBuilder::new(
        app,
        "login",
        url
    )
    .title("Login - Monori")
    .inner_size(960.0, 600.0)
    .center()
    .resizable(false)
    .maximizable(false)
    .visible(true)
    .build()?;

    println!("create_login_window: Login window created successfully");

    Ok(())
}

/// Create main window
fn create_main_window(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("create_main_window: Starting window creation");

    // In development, we need to use the dev server URL
    let url = if cfg!(debug_assertions) {
        WebviewUrl::External("http://localhost:3000".parse().unwrap())
    } else {
        WebviewUrl::App("index.html".into())
    };

    println!("create_main_window: Using URL: {:?}", url);

    let window = WebviewWindowBuilder::new(
        app,
        "main",
        url
    )
    .title("Monori")
    .inner_size(1920.0, 1080.0)
    .resizable(true)
    .maximizable(true)
    .visible(false)  // Start hidden to prevent flickering
    .build()?;

    println!("create_main_window: Window built successfully");

    // Maximize after creation
    window.maximize()?;
    println!("create_main_window: Window maximized");

    // Then show the window
    window.show()?;
    println!("create_main_window: Window shown");

    // Set focus to ensure it comes to front
    window.set_focus()?;
    println!("create_main_window: Window focused");

    Ok(())
}

/// Switch to main window after successful login
#[tauri::command]
pub async fn switch_to_main_window(app: AppHandle) -> Result<(), String> {
    println!("switch_to_main_window called");

    // Store reference to login window (to close it later)
    let login_window = app.get_webview_window("login");

    // FIRST: Create main window (while login window still exists)
    if app.get_webview_window("main").is_none() {
        println!("Creating new main window");
        create_main_window(&app)
            .map_err(|e| format!("Failed to create main window: {}", e))?;
        println!("Main window created successfully");

        // Wait for window to be fully loaded
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    // SECOND: Verify main window exists and show it
    if let Some(main_window) = app.get_webview_window("main") {
        println!("Main window exists, showing and focusing");
        main_window.show()
            .map_err(|e| format!("Failed to show main window: {}", e))?;
        main_window.set_focus()
            .map_err(|e| format!("Failed to focus main window: {}", e))?;

        // Try to maximize again in case it wasn't properly maximized
        let _ = main_window.maximize();
        println!("Main window shown, focused, and maximized");
    } else {
        return Err("Main window does not exist after creation!".to_string());
    }

    // THIRD: Now close the login window (after main window is ready)
    if let Some(login) = login_window {
        println!("Closing login window");
        // Small delay before closing to ensure smooth transition
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        login.close()
            .map_err(|e| format!("Failed to close login window: {}", e))?;
        println!("Login window closed successfully");
    } else {
        println!("Warning: Login window not found");
    }

    println!("switch_to_main_window completed successfully");
    Ok(())
}

/// Switch to login window on logout
#[tauri::command]
pub async fn switch_to_login_window(app: AppHandle) -> Result<(), String> {
    println!("switch_to_login_window called");

    // Store reference to main window (to close it later)
    let main_window = app.get_webview_window("main");

    // FIRST: Create login window (while main window still exists)
    if app.get_webview_window("login").is_none() {
        println!("Creating new login window");
        create_login_window(&app)
            .map_err(|e| format!("Failed to create login window: {}", e))?;
        println!("Login window created successfully");

        // Wait for window to be fully loaded
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    }

    // SECOND: Verify login window exists and show it
    if let Some(login_window) = app.get_webview_window("login") {
        println!("Login window exists, showing and focusing");
        login_window.show()
            .map_err(|e| format!("Failed to show login window: {}", e))?;
        login_window.set_focus()
            .map_err(|e| format!("Failed to focus login window: {}", e))?;
        println!("Login window shown and focused");
    } else {
        return Err("Login window does not exist after creation!".to_string());
    }

    // THIRD: Clear session
    clear_session_internal(&app)?;
    println!("Session cleared");

    // FOURTH: Now close the main window (after login window is ready)
    if let Some(main) = main_window {
        println!("Closing main window");
        // Small delay before closing to ensure smooth transition
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        main.close()
            .map_err(|e| format!("Failed to close main window: {}", e))?;
        println!("Main window closed successfully");
    } else {
        println!("Warning: Main window not found");
    }

    println!("switch_to_login_window completed successfully");
    Ok(())
}

/// Helper function to clear session
fn clear_session_internal(app: &AppHandle) -> Result<(), String> {
    let session_path = get_session_file_path(app)
        .map_err(|e| format!("Failed to get session path: {}", e))?;

    if session_path.exists() {
        std::fs::remove_file(&session_path)
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
pub async fn check_session(app: AppHandle) -> Result<bool, String> {
    check_user_session(&app)
        .map_err(|e| format!("Failed to check session: {}", e))
}

/// Validate login credentials
#[tauri::command]
pub async fn validate_login(email: String, password: String) -> Result<String, String> {
    // Trim inputs
    let email = email.trim();
    let password = password.trim();

    // Debug logging
    println!("Login attempt - Email: '{}', Password: '{}'", email, password);
    println!("Expected - Email: '{}', Password: '{}'", TEST_USER.email, TEST_USER.password);

    // Check against test user
    if email == TEST_USER.email && password == TEST_USER.password {
        // Generate a simple token with user info
        let token = format!("{}:{}", TEST_USER.email, TEST_USER.name);
        println!("Login successful, token generated");
        Ok(token)
    } else {
        println!("Login failed - credentials don't match");
        Err("Invalid email or password".to_string())
    }
}

/// Save session after successful login
#[tauri::command]
pub async fn save_session(app: AppHandle, token: String) -> Result<(), String> {
    println!("Saving session token: {}", token);

    // Get the app's data directory for proper file storage
    let session_path = get_session_file_path(&app)
        .map_err(|e| format!("Failed to get session path: {}", e))?;

    println!("Saving session to: {:?}", session_path);

    let result = std::fs::write(&session_path, &token);

    match result {
        Ok(_) => {
            println!("Session saved successfully");
            // Verify the file was written
            if let Ok(content) = std::fs::read_to_string(&session_path) {
                println!("Verified session file content: {}", content);
            }
            Ok(())
        }
        Err(e) => {
            let error_msg = format!("Failed to save session: {}", e);
            println!("Error: {}", error_msg);
            Err(error_msg)
        }
    }
}

/// Get current user info from session
#[tauri::command]
pub async fn get_current_user(app: AppHandle) -> Result<String, String> {
    let session_path = get_session_file_path(&app)
        .map_err(|e| format!("Failed to get session path: {}", e))?;

    if let Ok(token) = std::fs::read_to_string(&session_path) {
        // Extract name from token (format: "email:name")
        if let Some(name) = token.split(':').nth(1) {
            Ok(name.to_string())
        } else {
            Ok("User".to_string())
        }
    } else {
        Err("No active session".to_string())
    }
}

/// Clear session
#[tauri::command]
pub async fn clear_session(app: AppHandle) -> Result<(), String> {
    clear_session_internal(&app)
}
