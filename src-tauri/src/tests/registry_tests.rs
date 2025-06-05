// src/tests/registry_tests.rs
use crate::security_checks;
use crate::models::{CheckStatus, Importance};
use winreg::{enums::*, RegKey};
use serial_test::serial;

#[test]
#[serial]
fn test_screen_saver_check_vulnerable() {
    // This test reads actual registry values
    let result = security_checks::check_screen_saver_settings().unwrap();
    
    // Test structure
    assert_eq!(result.code, "PC-12");
    assert_eq!(result.category, "보안 관리");
    assert!(matches!(result.importance, Importance::High));
    
    // The actual status depends on system configuration
    println!("Screen saver check result: {:?}", result.status);
    println!("Details: {}", result.detail);
}

#[test]
#[serial]
fn test_autorun_check() {
    let result = security_checks::check_autorun_settings().unwrap();
    
    assert_eq!(result.code, "PC-13");
    assert_eq!(result.category, "보안 관리");
    assert!(matches!(result.importance, Importance::High));
    
    // Verify the check runs without panic
    match result.status {
        CheckStatus::Good => {
            assert!(result.detail.contains("자동 실행 차단 정책이 적절히 설정"));
        },
        CheckStatus::Vulnerable => {
            assert!(result.detail.contains("자동 실행 차단 정책"));
        },
        _ => {}
    }
}

#[test]
#[serial]
fn test_browser_temp_files_check() {
    let result = security_checks::check_browser_temp_files_settings().unwrap();
    
    assert_eq!(result.code, "PC-18");
    assert_eq!(result.category, "서비스 관리");
    assert!(matches!(result.importance, Importance::Low));
    
    println!("Browser temp files check: {:?}", result.status);
}

#[test]
#[serial]
fn test_remote_access_check() {
    let result = security_checks::check_remote_access_settings().unwrap();
    
    assert_eq!(result.code, "PC-19");
    assert_eq!(result.category, "보안 관리");
    assert!(matches!(result.importance, Importance::Medium));
    
    // Should contain info about both remote assistance and desktop
    assert!(result.detail.contains("원격 지원") || result.detail.contains("원격 데스크톱"));
}
