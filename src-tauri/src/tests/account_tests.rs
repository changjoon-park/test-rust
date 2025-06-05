// src/tests/account_tests.rs
use crate::security_checks;
use crate::models::CheckStatus;
use serial_test::serial;

#[test]
#[serial]
fn test_password_expiration_check() {
    let result = security_checks::check_password_expiration();
    
    assert!(result.is_ok());
    
    if let Ok(check) = result {
        assert_eq!(check.code, "PC-01");
        assert_eq!(check.category, "계정 관리");
        
        // If not admin, should fail gracefully
        if !is_running_as_admin() {
            assert!(matches!(check.status, CheckStatus::CheckFailed));
        } else {
            // Should contain days information
            assert!(check.detail.contains("일") || check.detail.contains("설정"));
        }
    }
}

#[test]
#[serial]
fn test_password_policy_check() {
    let result = security_checks::check_password_policy();
    
    assert!(result.is_ok());
    
    if let Ok(check) = result {
        assert_eq!(check.code, "PC-02");
        
        match check.status {
            CheckStatus::Good | CheckStatus::Vulnerable => {
                // Should mention both length and complexity
                assert!(check.detail.contains("최소 길이"));
                assert!(check.detail.contains("복잡성"));
            },
            _ => {}
        }
    }
}

#[test]
#[serial]
fn test_recovery_console_check() {
    let result = security_checks::check_recovery_console_settings();
    
    assert!(result.is_ok());
    
    if let Ok(check) = result {
        assert_eq!(check.code, "PC-15");
        assert!(check.detail.contains("복구 콘솔"));
    }
}

fn is_running_as_admin() -> bool {
    // Same helper function as above
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
