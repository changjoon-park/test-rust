// src/security_checks/account_management.rs
use crate::models::{CheckResult, CheckStatus, Importance};
use std::process::Command;
use std::fs;

/// PC-01: Check password expiration policy
pub fn check_password_expiration() -> Result<CheckResult, Box<dyn std::error::Error>> {
    // Export security policy
    let temp_file = std::env::temp_dir().join("secpol.cfg");
    let export_output = Command::new("secedit")
        .args(&["/export", "/cfg", temp_file.to_str().unwrap()])
        .output()?;
    
    if !export_output.status.success() {
        return Ok(CheckResult {
            category: "계정 관리".to_string(),
            code: "PC-01".to_string(),
            item: "패스워드의 주기적 변경".to_string(),
            importance: Importance::High,
            status: CheckStatus::CheckFailed,
            detail: "보안 정책을 내보낼 수 없습니다. 관리자 권한이 필요합니다.".to_string(),
        });
    }
    
    // Read the exported file
    let content = fs::read_to_string(&temp_file)?;
    
    // Clean up temp file
    let _ = fs::remove_file(&temp_file);
    
    // Parse MaximumPasswordAge
    let mut max_password_age: Option<i32> = None;
    
    for line in content.lines() {
        if line.contains("MaximumPasswordAge") {
            if let Some(value_part) = line.split('=').nth(1) {
                max_password_age = value_part.trim().parse().ok();
            }
        }
    }
    
    let (status, detail) = match max_password_age {
        Some(days) if days > 0 && days <= 90 => (
            CheckStatus::Good,
            format!("최대 암호 사용 기간이 {} 일로 설정되어 있습니다.", days)
        ),
        Some(days) if days == 0 => (
            CheckStatus::Vulnerable,
            "최대 암호 사용 기간이 설정되지 않았습니다 (무제한).".to_string()
        ),
        Some(days) => (
            CheckStatus::Vulnerable,
            format!("{} 일, 최대 암호 사용 기간이 90일을 초과합니다.", days)
        ),
        None => (
            CheckStatus::CheckFailed,
            "최대 암호 사용 기간 설정을 확인할 수 없습니다.".to_string()
        ),
    };
    
    Ok(CheckResult {
        category: "계정 관리".to_string(),
        code: "PC-01".to_string(),
        item: "패스워드의 주기적 변경".to_string(),
        importance: Importance::High,
        status,
        detail,
    })
}

/// PC-02: Check password complexity requirements
pub fn check_password_policy() -> Result<CheckResult, Box<dyn std::error::Error>> {
    // Export security policy
    let temp_file = std::env::temp_dir().join("secpol.cfg");
    let export_output = Command::new("secedit")
        .args(&["/export", "/cfg", temp_file.to_str().unwrap()])
        .output()?;
    
    if !export_output.status.success() {
        return Ok(CheckResult {
            category: "계정 관리".to_string(),
            code: "PC-02".to_string(),
            item: "패스워드 정책이 해당 기관의 보안 정책에 적합하게 설정".to_string(),
            importance: Importance::High,
            status: CheckStatus::CheckFailed,
            detail: "보안 정책을 내보낼 수 없습니다. 관리자 권한이 필요합니다.".to_string(),
        });
    }
    
    // Read the exported file
    let content = fs::read_to_string(&temp_file)?;
    
    // Clean up temp file
    let _ = fs::remove_file(&temp_file);
    
    // Parse settings
    let mut min_password_length: Option<i32> = None;
    let mut password_complexity: Option<i32> = None;
    
    for line in content.lines() {
        if line.contains("MinimumPasswordLength") {
            if let Some(value_part) = line.split('=').nth(1) {
                min_password_length = value_part.trim().parse().ok();
            }
        }
        if line.contains("PasswordComplexity") {
            if let Some(value_part) = line.split('=').nth(1) {
                password_complexity = value_part.trim().parse().ok();
            }
        }
    }
    
    let min_length_ok = min_password_length.map_or(false, |len| len >= 8);
    let complexity_ok = password_complexity.map_or(false, |comp| comp == 1);
    
    let (status, detail) = if min_length_ok && complexity_ok {
        (
            CheckStatus::Good,
            format!("패스워드 최소 길이: {} 자 - 양호, 패스워드 복잡성 설정: 활성화됨 - 양호",
                min_password_length.unwrap_or(0))
        )
    } else {
        let mut details = Vec::new();
        details.push(format!("패스워드 최소 길이: {} 자 - {}",
            min_password_length.unwrap_or(0),
            if min_length_ok { "양호" } else { "취약" }
        ));
        details.push(format!("패스워드 복잡성 설정: {} - {}",
            if password_complexity == Some(1) { "활성화됨" } else { "비활성화됨" },
            if complexity_ok { "양호" } else { "취약" }
        ));
        
        (CheckStatus::Vulnerable, details.join(", "))
    };
    
    Ok(CheckResult {
        category: "계정 관리".to_string(),
        code: "PC-02".to_string(),
        item: "패스워드 정책이 해당 기관의 보안 정책에 적합하게 설정".to_string(),
        importance: Importance::High,
        status,
        detail,
    })
}

/// PC-15: Check recovery console auto-login setting
pub fn check_recovery_console_settings() -> Result<CheckResult, Box<dyn std::error::Error>> {
    use crate::utils::registry::RegistryReader;
    use winreg::enums::HKEY_LOCAL_MACHINE;
    
    // First check via security policy
    let temp_file = std::env::temp_dir().join("secpol.cfg");
    let export_output = Command::new("secedit")
        .args(&["/export", "/cfg", temp_file.to_str().unwrap()])
        .output()?;
    
    let mut recovery_console_security_level: Option<i32> = None;
    
    if export_output.status.success() {
        if let Ok(content) = fs::read_to_string(&temp_file) {
            for line in content.lines() {
                if line.contains("RecoveryConsoleSecurityLevel") {
                    if let Some(value_part) = line.split('=').nth(1) {
                        recovery_console_security_level = value_part.trim().parse().ok();
                    }
                }
            }
        }
        let _ = fs::remove_file(&temp_file);
    }
    
    // Also check registry
    let auto_admin_logon = RegistryReader::read_string(
        HKEY_LOCAL_MACHINE,
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon",
        "AutoAdminLogon"
    )?;
    
    let auto_logon_enabled = recovery_console_security_level == Some(1) || 
                            auto_admin_logon == Some("1".to_string());
    
    let (status, detail) = if !auto_logon_enabled {
        (
            CheckStatus::Good,
            "Windows 복구 콘솔 자동 관리자 로그인이 비활성화되어 있습니다.".to_string()
        )
    } else {
        (
            CheckStatus::Vulnerable,
            "Windows 복구 콘솔 자동 관리자 로그인이 활성화되어 있습니다.".to_string()
        )
    };
    
    Ok(CheckResult {
        category: "계정 관리".to_string(),
        code: "PC-15".to_string(),
        item: "복구 콘솔에서 자동 로그온을 금지하도록 설정".to_string(),
        importance: Importance::Medium,
        status,
        detail,
    })
}
