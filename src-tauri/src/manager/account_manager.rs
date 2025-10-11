use crate::error::{Error, Result};
use crate::models::{AccountType, UserProfile};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

fn get_accounts_path(app_handle: &AppHandle) -> Result<PathBuf> {
    let path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| Error::Directory(format!("Failed to resolve app data directory {}", e)))?;

    fs::create_dir_all(&path)?;

    Ok(path.join("accounts.json"))
}

fn write_accounts_to_disk(path: &Path, accounts: &[UserProfile]) -> Result<()> {
    let json_string = serde_json::to_string_pretty(accounts)?;

    fs::write(path, json_string)?;
    Ok(())
}

pub fn get_all_accounts(app_handle: &AppHandle) -> Result<Vec<UserProfile>> {
    let path = get_accounts_path(app_handle)?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(path)?;

    if contents.is_empty() {
        return Ok(Vec::new());
    }

    let profiles = serde_json::from_str(&contents)?;
    Ok(profiles)
}

pub fn add_offline_account(app_handle: &AppHandle, username: &str) -> Result<UserProfile> {
    let path = get_accounts_path(app_handle)?;
    let mut accounts = get_all_accounts(app_handle)?;

    if accounts
        .iter()
        .any(|p| p.username.eq_ignore_ascii_case(username))
    {
        return Err(Error::DuplicateUser(format!(
            "An account with the username '{}' already exists",
            username
        )));
    }

    let new_profile = UserProfile {
        uuid: Uuid::new_v4(),
        username: username.to_string(),
        avatar_url: format!(
            "https://api.dicebear.com/8.x/pixel-art/svg?seed={}",
            username
        ),
        account_type: AccountType::Offline,
    };

    accounts.push(new_profile.clone());
    write_accounts_to_disk(&path, &accounts)?;
    Ok(new_profile)
}
