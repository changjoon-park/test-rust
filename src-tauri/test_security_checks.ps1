# test_security_checks.ps1
Write-Host "Running Windows Security Checker Tests" -ForegroundColor Cyan

# Run tests without admin privileges
Write-Host "`nRunning non-admin tests..." -ForegroundColor Yellow
cargo test --lib -- --nocapture --test-threads=1

# Check if running as admin
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if ($isAdmin) {
    Write-Host "`nRunning admin tests..." -ForegroundColor Green
    cargo test --lib --features admin-tests -- --nocapture --test-threads=1
} else {
    Write-Host "`nSkipping admin tests (run as administrator for full test coverage)" -ForegroundColor Red
}

Write-Host "`nTest Summary:" -ForegroundColor Cyan
cargo test --lib -- --quiet
