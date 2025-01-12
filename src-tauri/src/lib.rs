mod backend;
mod infrastructure;
mod domain;
mod application;


use infrastructure::xplane_rest;
use infrastructure::xplane_ws;
use domain::systems::electrical::battery;
use application::useCases::connect_to_xplane::ConnectToXPlaneUseCase;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn connect_to_xplane() -> Result<(), String> {
    use tokio::join;

    let mut use_case = ConnectToXPlaneUseCase::new();

    match use_case.execute().await {
        Ok(state) => Ok(state),
        Err(err) => Err(format!("Erro na conexão: {}", err)),
    }
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
        .invoke_handler(tauri::generate_handler![greet, connect_to_xplane])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
