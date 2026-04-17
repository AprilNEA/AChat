mod commands;

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::greet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
