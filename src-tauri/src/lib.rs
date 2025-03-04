pub mod config;
pub mod win_operator;

use std::time::Duration;

use config::Config;
use tauri::Window;

/// 加载设置选项
#[tauri::command]
fn get_config() -> String {
    let config = Config::load().unwrap();
    serde_json::to_string(&config).unwrap()
}

// 保存设置选项
#[tauri::command]
fn save_config(config: String) {
    let config: Config = serde_json::from_str(&config).unwrap();
    config.save().unwrap();
}

// 锁屏
#[tauri::command]
fn lock_screen(window: Window) {
    window.show().unwrap();
    window.set_focus().unwrap();
    win_operator::begin_lock(Duration::from_secs(1));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            lock_screen
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
