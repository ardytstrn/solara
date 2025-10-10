use tauri::AppHandle;

mod models;
mod manager;

#[tauri::command]
fn get_accounts(app_handle: AppHandle) -> Result<Vec<models::UserProfile>, String> {
    manager::account_manager::get_all_accounts(&app_handle)
}

#[tauri::command]
fn add_offline_account(app_handle: AppHandle, username: String) -> Result<models::UserProfile, String> {
    if username.is_empty() || username.len() < 3 {
        return Err("Username must be at least 3 characters long.".to_string());
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
