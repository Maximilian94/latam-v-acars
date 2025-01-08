use axum::{
    extract::Json,
    response::IntoResponse,
    routing::post,
    Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

// Estrutura dos dados enviados pelo cliente
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")] 
pub struct LoginRequest {
    pub email_or_username: String,
    pub password: String,
}

// Função para iniciar o backend
pub async fn start_backend() -> Result<(), Box<dyn std::error::Error>> {
    // Configuração das rotas
    let app = Router::new().route("/login", post(login_handler));

    // Endereço onde o servidor irá rodar
    let addr = "127.0.0.1:3001".parse().unwrap(); // Alterado para não colidir com o servidor existente
    println!("Backend rodando em http://{}", addr);

    // Inicia o servidor
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// Handler para login
async fn login_handler(
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // URL do servidor existente
    let server_url = "http://127.0.0.1:3000/auth/login"; // Atualizada

    // Cliente HTTP para fazer a requisição
    let client = Client::new();

    // Faz a requisição ao servidor existente
    match client
        .post(server_url)
        .json(&payload) // Envia os dados como JSON
        .send()
        .await
    {
        Ok(response) => {
            // Extrai o status antes de consumir o response
            let status = response.status();
        
            // Consome o response para obter o JSON
            let error: serde_json::Value = response.json().await.unwrap();
        
            (
                status, // Usa o status já extraído
                Json(error),
            )
        }
        Err(err) => {
            // Em caso de erro na comunicação com o servidor
            (
                axum::http::StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({
                    "error": format!("Falha ao conectar ao servidor: {}", err)
                })),
            )
        }
    }
}
