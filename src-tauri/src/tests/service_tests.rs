// src/tests/service_tests.rs
use crate::security_checks;
use crate::models::{CheckStatus, Importance};
use serial_test::serial;

#[test]
#[serial]
fn test_ntfs_filesystem_check() {
    let result = security_checks::check_ntfs_filesystem();
    
    // Should not panic even if WMI fails
    assert!(result.is_ok());
    
    if let Ok(check) = result {
        assert_eq!(check.code, "PC-16");
        assert_eq!(check.category, "서비스 관리");
        
        match check.status {
            CheckStatus::Good => {
                assert!(check.detail.contains("NTFS"));
            },
            CheckStatus::Vulnerable => {
                assert!(check.detail.contains("NTFS 파일 시스템을 사용하지 않는"));
            },
            _ => {}
        }
    }
}

#[test]
#[serial]
fn test_multiboot_config_check() {
    let result = security_checks::check_multiboot_config();
    
    assert!(result.is_ok());
    
    if let Ok(check) = result {
        assert_eq!(check.code, "PC-17");
        
        // If not admin, should return CheckFailed
        if !is_running_as_admin() {
            assert!(matches!(check.status, CheckStatus::CheckFailed));
            assert!(check.detail.contains("관리자 권한"));
        }
    }
}

#[test]
#[serial]
fn test_unnecessary_services_check() {
    let result = security_checks::check_unnecessary_services();
    
    if let Ok(check) = result {
        assert_eq!(check.code, "PC-04");
        assert!(matches!(check.importance, Importance::High));
        
        println!("Unnecessary services status: {:?}", check.status);
        println!("Details: {}", check.detail);
    }
}

#[test]
#[serial]
fn test_firewall_status_check() {
    let result = security_checks::check_firewall_status();
    
    assert!(result.is_ok());
    
    if let Ok(check) = result {
        assert_eq!(check.code, "PC-11");
        
        // Should contain profile information
        assert!(check.detail.contains("프로필"));
    }
}

// Helper function
fn is_running_as_admin() -> bool {
    use std::ptr;
    use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
    use winapi::um::securitybaseapi::GetTokenInformation;
    use winapi::um::winnt::{TokenElevation, HANDLE, TOKEN_ELEVATION, TOKEN_QUERY};
    
    unsafe {
        let mut token_handle: HANDLE = ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut size = 0;
        
        if GetTokenInformation(
            token_handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        ) == 0 {
            return false;
        }

        elevation.TokenIsElevated != 0
    }
}
