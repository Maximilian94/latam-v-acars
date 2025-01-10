use serde::Serialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::StreamExt;
use futures_util::SinkExt;
use url::Url;
use anyhow::{Result, Context};

#[derive(Serialize)]
pub struct XPlaneConnectionState {
    pub connected: bool,
    pub message: String,
}

// Função para conectar ao WebSocket do X-Plane
pub async fn connect_to_xplane() -> Result<XPlaneConnectionState> {
    let url = Url::parse("ws://localhost:8086/api/v1")
        .context("Erro ao parsear URL do WebSocket")?;

    let (ws_stream, _) = connect_async(url).await
        .context("Erro ao conectar ao WebSocket")?;
    let (mut write, _) = ws_stream.split();

    write
        .send(Message::Text("ACARS conectado ao X-Plane".to_string()))
        .await
        .context("Erro ao enviar mensagem pelo WebSocket")?;

    println!("Conexão via WebSocket estabelecida!");

    Ok(XPlaneConnectionState {
        connected: true,
        message: "Conexão via WebSocket estabelecida com sucesso".to_string(),
    })
}
