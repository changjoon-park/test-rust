use winreg::enums::*;
use winreg::RegKey;

fn main() -> std::io::Result<()> {
    // 1. 레지스트리 키 열기
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")?;
    
    // 2. 값 읽기
    let product_name: String = cur_ver.get_value("ProductName")?;
    let build: String = cur_ver.get_value("CurrentBuildNumber")?;
    let ubr: u32 = cur_ver.get_value("UBR")?;  // Update Build Revision
    
    // 3. 출력
    println!("OS: {}", product_name);
    println!("Build: {}.{}", build, ubr);
    
    Ok(())
}
