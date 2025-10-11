use crate::models::UserProfile;
use tauri::AppHandle;

mod error;
mod manager;
mod models;

type CommandResult<T> = std::result::Result<T, error::Error>;

#[tauri::command]
fn get_accounts(app_handle: AppHandle) -> CommandResult<Vec<UserProfile>> {
    manager::account_manager::get_all_accounts(&app_handle)
}

#[tauri::command]
fn add_offline_account(app_handle: AppHandle, username: String) -> CommandResult<UserProfile> {
    if username.is_empty() || username.len() < 3 {
        return Err(error::Error::Validation(
            "Username must be at least 3 characters long".to_string(),
        ));
    }
    manager::account_manager::add_offline_account(&app_handle, &username)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_accounts, add_offline_account])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
