use std::collections::HashMap;
use once_cell::sync::Lazy;

pub static REPLACEMENTS: Lazy<HashMap<String, String>> = Lazy::new(||{
    let mut replacements = HashMap::new();
    replacements.insert(" ".to_string(), "".to_string());
    replacements.insert("_".to_string(), "".to_string());
    replacements
});

pub static OS_REPLACEMENTS: Lazy<HashMap<String, Vec<String>>> = Lazy::new(||{
    let mut os_replacements = HashMap::new();
    os_replacements.insert("windows".to_string(), vec!["win10".to_string(), "win7".to_string()]);
    os_replacements
});

pub static SHORTCUTS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut shortcuts = HashMap::new();
    shortcuts.insert("microsoft edge".to_string(), "edge".to_string());
    shortcuts.insert("google".to_string(), "chrome".to_string());
    shortcuts.insert("googlechrome".to_string(), "chrome".to_string());
    shortcuts.insert("ff".to_string(), "firefox".to_string());
    shortcuts
});