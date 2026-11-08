//! Windows registry read helpers for Altair Hyperworks.

pub const APP_KEY: &str = "Software\\Altair\\Altair Hyperworks";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Altair Hyperworks";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Altair Hyperworks") },
        RegValue { name: "Version",     value: String::from("v2.0.16") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Altair Hyperworks") },
        RegValue { name: "Publisher",       value: String::from("Altair") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Altair Hyperworks\\uninstall.exe\"") },
    ]
}
