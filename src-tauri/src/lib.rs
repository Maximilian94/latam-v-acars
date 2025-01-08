mod backend;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Inicializa o backend em uma tarefa assíncrona usando o runtime do Tauri
    tauri::async_runtime::spawn(async {
        if let Err(e) = backend::start_backend().await {
            eprintln!("Erro ao iniciar o backend: {}", e);
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
