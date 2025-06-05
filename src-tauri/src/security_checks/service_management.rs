// src/security_checks/service_management.rs
use crate::models::{CheckResult, CheckStatus, Importance};
use crate::utils::wmi::WMIClient;
use serde::Deserialize;

/// PC-16: Check if all fixed drives use NTFS
pub fn check_ntfs_filesystem() -> Result<CheckResult, Box<dyn std::error::Error>> {
    let wmi_client = WMIClient::new()?;
    
    #[allow(non_snake_case, non_camel_case_types)]
    #[derive(Deserialize)]
    struct Win32_LogicalDisk {
        DeviceID: String,
        DriveType: u32,
        FileSystem: Option<String>,
    }
    
    let disks: Vec<Win32_LogicalDisk> = wmi_client.query()?;
    let fixed_disks: Vec<_> = disks.into_iter()
        .filter(|d| d.DriveType == 3) // Fixed drives only
        .collect();
    
    let non_ntfs: Vec<String> = fixed_disks.iter()
        .filter(|d| d.FileSystem.as_ref().map_or(true, |fs| fs != "NTFS"))
        .map(|d| format!("{} ({})", 
            d.DeviceID, 
            d.FileSystem.as_ref().unwrap_or(&"Unknown".to_string())
        ))
        .collect();
    
    let (status, detail) = if non_ntfs.is_empty() {
        (
            CheckStatus::Good, 
            "모든 고정 드라이브가 NTFS 파일 시스템을 사용하고 있습니다.".to_string()
        )
    } else {
        (
            CheckStatus::Vulnerable, 
            format!("NTFS 파일 시스템을 사용하지 않는 드라이브가 있습니다: {}", 
                non_ntfs.join(", "))
        )
    };
    
    Ok(CheckResult {
        category: "서비스 관리".to_string(),
        code: "PC-16".to_string(),
        item: "파일 시스템이 NTFS 포맷으로 설정".to_string(),
        importance: Importance::Medium,
        status,
        detail,
    })
}

/// PC-17: Check for multi-boot configuration
pub fn check_multiboot_config() -> Result<CheckResult, Box<dyn std::error::Error>> {
    use std::process::Command;
    
    // Run bcdedit command
    let output = Command::new("bcdedit")
        .arg("/enum")
        .output()?;
    
    if !output.status.success() {
        return Ok(CheckResult {
            category: "서비스 관리".to_string(),
            code: "PC-17".to_string(),
            item: "대상 시스템이 Windows 서버를 제외한 다른 OS로 멀티 부팅이 가능하지 않도록 설정".to_string(),
            importance: Importance::Medium,
            status: CheckStatus::CheckFailed,
            detail: "BCDEdit 명령 실행에 실패했습니다. 관리자 권한이 필요할 수 있습니다.".to_string(),
        });
    }
    
    let bcd_output = String::from_utf8_lossy(&output.stdout);
    let os_entries = bcd_output.matches("Windows Boot Loader").count();
    
    let (status, detail) = if os_entries <= 1 {
        (
            CheckStatus::Good,
            "멀티 부팅 설정이 되어 있지 않습니다.".to_string()
        )
    } else {
        (
            CheckStatus::Vulnerable,
            format!("멀티 부팅 설정이 되어 있습니다. 운영체제 항목 수: {}", os_entries)
        )
    };
    
    Ok(CheckResult {
        category: "서비스 관리".to_string(),
        code: "PC-17".to_string(),
        item: "대상 시스템이 Windows 서버를 제외한 다른 OS로 멀티 부팅이 가능하지 않도록 설정".to_string(),
        importance: Importance::Medium,
        status,
        detail,
    })
}

/// PC-04: Check for unnecessary services
pub fn check_unnecessary_services() -> Result<CheckResult, Box<dyn std::error::Error>> {
    let wmi_client = WMIClient::new()?;
    
    #[allow(non_snake_case, non_camel_case_types)]
    #[derive(Deserialize)]
    struct Win32_Service {
        Name: String,
        State: String,
        StartMode: String,
    }
    
    let unsafe_services = vec![
        "Alerter", "wuauserv", "ClipSrv", "Browser", 
        "CryptSvc", "Dhcp", "TrkWks", "TrkSvr",
        "Dnscache", "ERSvc", "HidServ", "ImapiService", 
        "Irmon", "Messenger", "mnmsrvc", "WmdmPmSp",
        "Spooler", "RemoteRegistry", "Simptcp", "SSDPSRV", 
        "WebClient"
    ];
    
    let services: Vec<Win32_Service> = wmi_client.query()?;
    
    let mut running_unsafe = Vec::new();
    let mut enabled_unsafe = Vec::new();
    
    for service in services {
        if unsafe_services.contains(&service.Name.as_str()) {
            if service.State == "Running" {
                running_unsafe.push(service.Name.clone());
            }
            if service.StartMode == "Auto" || service.StartMode == "Automatic" {
                enabled_unsafe.push(service.Name.clone());
            }
        }
    }
    
    let (status, detail) = if running_unsafe.is_empty() && enabled_unsafe.is_empty() {
        (
            CheckStatus::Good,
            "보안에 불필요한 서비스가 실행 중이거나 자동 시작으로 설정되어 있지 않습니다.".to_string()
        )
    } else {
        let mut detail = "보안에 불필요한 서비스가 실행 중이거나 자동 시작으로 설정되어 있습니다.\n".to_string();
        if !running_unsafe.is_empty() {
            detail.push_str(&format!("실행 중인 서비스: {}\n", running_unsafe.join(", ")));
        }
        if !enabled_unsafe.is_empty() {
            detail.push_str(&format!("자동 시작 설정된 서비스: {}", enabled_unsafe.join(", ")));
        }
        (CheckStatus::Vulnerable, detail)
    };
    
    Ok(CheckResult {
        category: "서비스 관리".to_string(),
        code: "PC-04".to_string(),
        item: "항목의 불필요한 서비스 제거".to_string(),
        importance: Importance::High,
        status,
        detail,
    })
}

/// PC-11: Check Windows Firewall status
pub fn check_firewall_status() -> Result<CheckResult, Box<dyn std::error::Error>> {
    use std::process::Command;
    
    // Using netsh command as it's more reliable than WMI for firewall status
    let output = Command::new("netsh")
        .args(&["advfirewall", "show", "allprofiles", "state"])
        .output()?;
    
    if !output.status.success() {
        return Ok(CheckResult {
            category: "보안 관리".to_string(),
            code: "PC-11".to_string(),
            item: "OS에서 제공하는 침입차단 기능 활성화".to_string(),
            importance: Importance::High,
            status: CheckStatus::CheckFailed,
            detail: "방화벽 상태를 확인할 수 없습니다.".to_string(),
        });
    }
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    let profiles = ["Domain", "Private", "Public"];
    let mut firewall_status = Vec::new();
    let mut all_enabled = true;
    
    for profile in &profiles {
        if output_str.contains(&format!("{} Profile Settings:", profile)) {
            let enabled = output_str.contains("State                                 ON") ||
                         output_str.contains("상태                                 사용");
            firewall_status.push(format!("- {} 프로필: {}", 
                profile, 
                if enabled { "활성화" } else { "비활성화" }
            ));
            if !enabled {
                all_enabled = false;
            }
        }
    }
    
    let (status, detail) = if all_enabled {
        (
            CheckStatus::Good,
            format!("모든 Windows 방화벽 프로필이 활성화되어 있습니다.\n{}", 
                firewall_status.join("\n"))
        )
    } else {
        (
            CheckStatus::Vulnerable,
            format!("일부 Windows 방화벽 프로필이 비활성화되어 있습니다.\n{}", 
                firewall_status.join("\n"))
        )
    };
    
    Ok(CheckResult {
        category: "보안 관리".to_string(),
        code: "PC-11".to_string(),
        item: "OS에서 제공하는 침입차단 기능 활성화".to_string(),
        importance: Importance::High,
        status,
        detail,
    })
}
