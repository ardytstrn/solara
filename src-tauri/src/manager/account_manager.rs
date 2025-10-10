use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use uuid::Uuid;
use crate::models::{AccountType, UserProfile};

fn get_accounts_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    fs::create_dir_all(&path).map_err(|e| format!("Failed to create app data directory: {}", e))?;

    Ok(path.join("accounts.json"))
}
pub fn get_all_accounts(app_handle: &AppHandle) -> Result<Vec<UserProfile>, String> {
    let path = get_accounts_path(app_handle)?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut file = File::open(path).map_err(|e| format!("Failed to open accounts file: {}", e))?;
    let mut contents = String::new();

    file.read_to_string(&mut contents).map_err(|e| format!("Failed to read accounts file: {}", e))?;

    if contents.is_empty() {
        return Ok(Vec::new());
    }

    serde_json::from_str(&contents).map_err(|e| format!("Failed to parse accounts file: {}", e))
}

pub fn add_offline_account(app_handle: &AppHandle, username: &str) -> Result<UserProfile, String> {
    let mut accounts = get_all_accounts(app_handle)?;

    if accounts.iter().any(|p| p.username.eq_ignore_ascii_case(username)) {
        return Err(format!("An account with the username '{}' already exists", username));
    }

    let new_profile = UserProfile {
        uuid: Uuid::new_v4(),
        username: username.to_string(),
        avatar_url: format!("https://api.dicebear.com/8.x/pixel-art/svg?seed={}", username),
        account_type: AccountType::Offline,
    };

    accounts.push(new_profile.clone());

    let path = get_accounts_path(app_handle)?;
    let mut file = File::create(path).map_err(|e| format!("Failed to create/truncate accounts file: {}", e))?;

    let json_string = serde_json::to_string_pretty(&accounts)
        .map_err(|e| format!("Failed to serialize accounts: {}", e))?;

    file.write_all(json_string.as_bytes())
        .map_err(|e| format!("Failed to write to accounts file: {}", e))?;

    Ok(new_profile)
}