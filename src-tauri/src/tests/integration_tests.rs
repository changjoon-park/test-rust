// src/tests/integration_tests.rs
use crate::models::SecurityReport;
use crate::run_all_security_checks_with_progress;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_full_security_check_workflow() {
    // Create a mock window for testing
    // Note: In real tests, you might want to mock this
    
    // For now, let's test the report generation
    let mut results = Vec::new();
    
    // Run a few checks
    if let Ok(result) = crate::security_checks::check_screen_saver_settings() {
        results.push(result);
    }
    
    if let Ok(result) = crate::security_checks::check_autorun_settings() {
        results.push(result);
    }
    
    let report = SecurityReport::new(results);
    
    // Test report structure
    assert!(!report.computer_name.is_empty());
    assert!(!report.date_time.is_empty());
    assert_eq!(report.os, "windows");
    assert!(!report.results.is_empty());
    
    // Test JSON serialization
    let json = serde_json::to_string(&report);
    assert!(json.is_ok());
    
    println!("Report JSON sample: {}", json.unwrap());
}

#[test]
fn test_check_result_serialization() {
    use crate::models::{CheckResult, CheckStatus, Importance};
    
    let result = CheckResult {
        category: "테스트".to_string(),
        code: "TEST-01".to_string(),
        item: "테스트 항목".to_string(),
        importance: Importance::High,
        status: CheckStatus::Good,
        detail: "테스트 상세".to_string(),
    };
    
    let json = serde_json::to_string(&result).unwrap();
    assert!(json.contains("\"중요도\":\"상\""));
    assert!(json.contains("\"점검결과\":\"양호\""));
}
