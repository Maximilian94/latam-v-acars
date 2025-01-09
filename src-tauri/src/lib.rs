mod backend;
mod xplane_ws;
mod xplane_rest;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn connect_to_xplane() -> Result<String, String> {
    use tokio::join;

    // Executa as operações simultaneamente
    let (ws_result, datarefs_result) = join!(
        xplane_ws::connect_to_xplane(),
        xplane_rest::list_datarefs()
    );

    // Verifica se a conexão WebSocket foi bem-sucedida
    if let Err(e) = ws_result {
        return Err(format!("Erro ao conectar ao X-Plane via WebSocket: {}", e));
    }

    // Verifica o resultado da listagem de datarefs
    match datarefs_result {
        Ok(datarefs) => {
            println!("Lista de DataRefs disponíveis:");
            for dataref in datarefs {
                println!(
                    "- {} (Writable: {}) - Type: {}",
                    dataref.name,
                    dataref.is_writable, // Campo correto
                    dataref.value_type
                );
            }
            Ok("Conexão e listagem de DataRefs concluídas com sucesso.".to_string())
        }
        Err(e) => Err(format!(
            "Conexão WebSocket estabelecida, mas falha ao listar DataRefs: {}",
            e
        )),
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
