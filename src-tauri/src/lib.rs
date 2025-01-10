use xplane_ws::XPlaneConnectionState;

mod backend;
mod xplane_ws;
mod xplane_rest;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn connect_to_xplane() -> Result<XPlaneConnectionState, String> {
    use tokio::join;

    // Executa as operações simultaneamente
    let (ws_result, datarefs_result) = join!(
        xplane_ws::connect_to_xplane(),
        xplane_rest::fetch_datarefs()
    );

    // Verifica o estado da conexão WebSocket
    let ws_state = ws_result.map_err(|e| e.to_string())?; // Obtém o estado da conexão
    println!("WebSocket: {}", ws_state.message);

    // Verifica o resultado da listagem de datarefs
    let datarefs = datarefs_result.map_err(|e| e.to_string())?;
    println!("Lista de DataRefs disponíveis:");
    for dataref in datarefs {
        println!(
            "- {} (Writable: {}) - Type: {}",
            dataref.name, dataref.is_writable, dataref.value_type
        );
    }

    Ok(ws_state)
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
