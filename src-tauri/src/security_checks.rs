// src/security_checks.rs
pub mod account_management;
pub mod service_management;
pub mod patch_management;
pub mod security_management;

// Re-export all check functions for easy access
pub use security_management::{
    check_screen_saver_settings,
    check_autorun_settings,
    check_browser_temp_files_settings,
    check_remote_access_settings,
};

pub use service_management::{
    check_ntfs_filesystem,
    check_multiboot_config,
    check_unnecessary_services,
    check_firewall_status,
};

pub use account_management::{
    check_password_expiration,
    check_password_policy,
    check_recovery_console_settings,
};
