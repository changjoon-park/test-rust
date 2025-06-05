// src/models.rs
use serde::{Serialize, Deserialize};
use chrono::Local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckStatus {
    #[serde(rename = "양호")]
    Good,
    #[serde(rename = "취약")]
    Vulnerable,
    #[serde(rename = "점검 실패")]
    CheckFailed,
    #[serde(rename = "수동 점검")]
    ManualCheck,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Importance {
    #[serde(rename = "상")]
    High,
    #[serde(rename = "중")]
    Medium,
    #[serde(rename = "하")]
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    #[serde(rename = "분류")]
    pub category: String,
    #[serde(rename = "항목코드")]
    pub code: String,
    #[serde(rename = "점검항목")]
    pub item: String,
    #[serde(rename = "중요도")]
    pub importance: Importance,
    #[serde(rename = "점검결과")]
    pub status: CheckStatus,
    #[serde(rename = "점검내용")]
    pub detail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityReport {
    #[serde(rename = "ComputerName")]
    pub computer_name: String,
    #[serde(rename = "DateTime")]
    pub date_time: String,
    #[serde(rename = "OS")]
    pub os: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Results")]
    pub results: Vec<CheckResult>,
}

impl SecurityReport {
    pub fn new(results: Vec<CheckResult>) -> Self {
        Self {
            computer_name: whoami::hostname(),
            date_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            os: std::env::consts::OS.to_string(),
            version: "1.0.0".to_string(),
            results,
        }
    }
}
