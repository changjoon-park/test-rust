// utils/registry.rs
use winreg::{enums::*, RegKey};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RegistryError {
    message: String,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Registry error: {}", self.message)
    }
}

impl Error for RegistryError {}

pub struct RegistryReader;

impl RegistryReader {
    pub fn read_dword(hive: HKEY, path: &str, value_name: &str) -> Result<Option<u32>, Box<dyn Error>> {
        match RegKey::predef(hive).open_subkey(path) {
            Ok(key) => match key.get_value::<u32, _>(value_name) {
                Ok(value) => Ok(Some(value)),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => Ok(None),
                    _ => Err(Box::new(RegistryError { 
                        message: format!("Failed to read {}: {}", value_name, e) 
                    })),
                }
            },
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(None),
                _ => Err(Box::new(RegistryError { 
                    message: format!("Failed to open key {}: {}", path, e) 
                })),
            }
        }
    }

    pub fn read_string(hive: HKEY, path: &str, value_name: &str) -> Result<Option<String>, Box<dyn Error>> {
        match RegKey::predef(hive).open_subkey(path) {
            Ok(key) => match key.get_value::<String, _>(value_name) {
                Ok(value) => Ok(Some(value)),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => Ok(None),
                    _ => Err(Box::new(RegistryError { 
                        message: format!("Failed to read {}: {}", value_name, e) 
                    })),
                }
            },
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Ok(None),
                _ => Err(Box::new(RegistryError { 
                    message: format!("Failed to open key {}: {}", path, e) 
                })),
            }
        }
    }
}
