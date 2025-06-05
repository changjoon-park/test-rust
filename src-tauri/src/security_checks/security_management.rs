// security_checks/security_management.rs
use crate::models::{CheckResult, CheckStatus, Importance};
use crate::utils::registry::RegistryReader;
use winreg::enums::*;

/// PC-12: Screen saver with password protection
pub fn check_screen_saver_settings() -> Result<CheckResult, Box<dyn std::error::Error>> {
    let desktop_path = r"Control Panel\Desktop";
    
    let screen_save_active = RegistryReader::read_string(HKEY_CURRENT_USER, desktop_path, "ScreenSaveActive")?
        .unwrap_or_else(|| "0".to_string());
    
    let screen_save_timeout = RegistryReader::read_string(HKEY_CURRENT_USER, desktop_path, "ScreenSaveTimeOut")?
        .unwrap_or_else(|| "0".to_string());
    
    let screen_saver_secure = RegistryReader::read_string(HKEY_CURRENT_USER, desktop_path, "ScreenSaverIsSecure")?
        .unwrap_or_else(|| "0".to_string());
    
    let timeout_seconds: i32 = screen_save_timeout.parse().unwrap_or(0);
    
    let (status, detail) = if screen_save_active == "1" && 
                              timeout_seconds > 0 && 
                              timeout_seconds <= 600 && 
                              screen_saver_secure == "1" {
        (
            CheckStatus::Good,
            format!("화면보호기가 활성화되어 있고, 대기 시간이 10분 이하({}초)이며, 암호가 설정되어 있습니다.", timeout_seconds)
        )
    } else {
        (
            CheckStatus::Vulnerable,
            format!("화면보호기 설정이 올바르지 않습니다. 활성화: {}, 대기 시간: {} 초, 암호 설정: {}", 
                    screen_save_active, timeout_seconds, screen_saver_secure)
        )
    };
    
    Ok(CheckResult {
        category: "보안 관리".to_string(),
        code: "PC-12".to_string(),
        item: "화면보호기 대기 시간 설정 및 재시작 시 암호 보호 설정".to_string(),
        importance: Importance::High,
        status,
        detail,
    })
}

/// PC-13: Removable media autorun prevention
pub fn check_autorun_settings() -> Result<CheckResult, Box<dyn std::error::Error>> {
    let mut is_compliant = false;
    let mut found_settings = Vec::new();
    
    // Check both HKCU and HKLM
    let paths = vec![
        (HKEY_CURRENT_USER, r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer"),
        (HKEY_LOCAL_MACHINE, r"Software\Microsoft\Windows\CurrentVersion\Policies\Explorer"),
    ];
    
    for (hive, path) in paths {
        // Check NoDriveTypeAutoRun (255 = disable all)
        if let Ok(Some(value)) = RegistryReader::read_dword(hive, path, "NoDriveTypeAutoRun") {
            found_settings.push(format!("NoDriveTypeAutoRun = {}", value));
            if value == 255 {
                is_compliant = true;
            }
        }
        
        // Check DisableAutoplay (1 = disabled)
        if let Ok(Some(value)) = RegistryReader::read_dword(hive, path, "DisableAutoplay") {
            found_settings.push(format!("DisableAutoplay = {}", value));
            if value == 1 {
                is_compliant = true;
            }
        }
    }
    
    let (status, detail) = if is_compliant {
        (
            CheckStatus::Good,
            format!("자동 실행 차단 정책이 적절히 설정되어 있습니다.\n발견된 설정:\n{}", found_settings.join("\n"))
        )
    } else if found_settings.is_empty() {
        (
            CheckStatus::Vulnerable,
            "자동 실행 차단 정책이 전혀 설정되어 있지 않습니다.\n권장값: NoDriveTypeAutoRun=255 또는 DisableAutoplay=1".to_string()
        )
    } else {
        (
            CheckStatus::Vulnerable,
            format!("자동 실행 차단 정책이 존재하나 기준에 미달합니다.\n현재 설정:\n{}\n\n권장값: NoDriveTypeAutoRun=255 또는 DisableAutoplay=1", 
                    found_settings.join("\n"))
        )
    };
    
    Ok(CheckResult {
        category: "보안 관리".to_string(),
        code: "PC-13".to_string(),
        item: "CD, DVD, USB 메모리 등과 같은 미디어의 자동실행 방지등 이동식 미디어에 대한 보안대책 수립".to_string(),
        importance: Importance::High,
        status,
        detail,
    })
}

/// PC-18: Browser temporary files deletion
pub fn check_browser_temp_files_settings() -> Result<CheckResult, Box<dyn std::error::Error>> {
    let reg_path = r"Software\Microsoft\Windows\CurrentVersion\Internet Settings\Cache";
    
    let persistent = RegistryReader::read_dword(HKEY_CURRENT_USER, reg_path, "Persistent")?;
    
    let (status, detail) = match persistent {
        Some(0) => (
            CheckStatus::Good,
            "브라우저 종료 시 임시 인터넷 파일을 삭제하도록 설정되어 있습니다. (Persistent=0)".to_string()
        ),
        Some(1) => (
            CheckStatus::Vulnerable,
            "브라우저 종료 시 임시 인터넷 파일이 삭제되지 않도록 설정되어 있습니다. (Persistent=1)".to_string()
        ),
        Some(value) => (
            CheckStatus::Vulnerable,
            format!("알 수 없는 설정값입니다. (Persistent={})", value)
        ),
        None => (
            CheckStatus::Vulnerable,
            "해당 설정(Persistent)이 존재하지 않아 기본값(1)으로 간주됩니다. 임시 인터넷 파일이 자동으로 삭제되지 않을 수 있습니다.".to_string()
        ),
    };
    
    Ok(CheckResult {
        category: "서비스 관리".to_string(),
        code: "PC-18".to_string(),
        item: "브라우저 종료 시 임시 인터넷 파일 폴더의 내용을 삭제하도록 설정".to_string(),
        importance: Importance::Low,
        status,
        detail,
    })
}

/// PC-19: Remote assistance/desktop settings
pub fn check_remote_access_settings() -> Result<CheckResult, Box<dyn std::error::Error>> {
    let mut remote_support_details = Vec::new();
    let mut any_enabled = false;
    
    // Check Remote Assistance
    let remote_assistance = RegistryReader::read_dword(
        HKEY_LOCAL_MACHINE, 
        r"SYSTEM\CurrentControlSet\Control\Remote Assistance", 
        "fAllowToGetHelp"
    )?;
    
    match remote_assistance {
        Some(1) => {
            remote_support_details.push("- 원격 지원이 활성화되어 있습니다.");
            any_enabled = true;
        },
        Some(0) => remote_support_details.push("- 원격 지원이 비활성화되어 있습니다."),
        None => remote_support_details.push("- 원격 지원 설정을 찾을 수 없습니다."),
        _ => {},
    }
    
    // Check Remote Desktop
    let remote_desktop = RegistryReader::read_dword(
        HKEY_LOCAL_MACHINE, 
        r"SYSTEM\CurrentControlSet\Control\Terminal Server", 
        "fDenyTSConnections"
    )?;
    
    match remote_desktop {
        Some(0) => {
            remote_support_details.push("- 원격 데스크톱이 활성화되어 있습니다.");
            any_enabled = true;
        },
        Some(1) => remote_support_details.push("- 원격 데스크톱이 비활성화되어 있습니다."),
        None => remote_support_details.push("- 원격 데스크톱 설정을 찾을 수 없습니다."),
        _ => {},
    }
    
    let (status, detail) = if !any_enabled {
        (
            CheckStatus::Good,
            format!("원격 지원 및 원격 데스크톱이 모두 비활성화되어 있습니다.\n{}", remote_support_details.join("\n"))
        )
    } else {
        (
            CheckStatus::Vulnerable,
            format!("원격 지원 또는 원격 데스크톱이 활성화되어 있습니다.\n{}", remote_support_details.join("\n"))
        )
    };
    
    Ok(CheckResult {
        category: "보안 관리".to_string(),
        code: "PC-19".to_string(),
        item: "원격 지원을 금지하도록 정책이 설정".to_string(),
        importance: Importance::Medium,
        status,
        detail,
    })
}
