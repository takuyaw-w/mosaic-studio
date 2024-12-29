// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod db;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::test_postgres_connection,
            commands::test_sqlite_connection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
