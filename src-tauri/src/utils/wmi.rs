// src/utils/wmi.rs
use wmi::{COMLibrary, WMIConnection};
use std::collections::HashMap;
use serde::Deserialize;

pub struct WMIClient {
    wmi_con: WMIConnection,
}

impl WMIClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let com_con = COMLibrary::new()?;
        let wmi_con = WMIConnection::new(com_con)?;
        Ok(Self {
            wmi_con,
        })
    }

    pub fn query<T>(&self) -> Result<Vec<T>, Box<dyn std::error::Error>> 
    where
        T: for<'de> Deserialize<'de>,
    {
        let results: Vec<T> = self.wmi_con.query()?;
        Ok(results)
    }

    #[allow(dead_code)]
    pub fn raw_query(&self, query: &str) -> Result<Vec<HashMap<String, wmi::Variant>>, Box<dyn std::error::Error>> {
        let results = self.wmi_con.raw_query(query)?;
        Ok(results)
    }
}
