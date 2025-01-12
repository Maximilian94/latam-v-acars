use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::WebSocketStream;
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::SplitStream;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use serde::Serialize;
use serde_json::json;
use url::Url;
use anyhow::{Result, Context};

type WebSocketWrite = SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>;
type WebSocketRead = SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>;

pub struct XPlaneClient {
    write: Option<WebSocketWrite>,
    read: Option<WebSocketRead>,
}

impl XPlaneClient {
    /// Cria uma nova instância de XPlaneClient
    pub fn new() -> Self {
        Self {
            write: None,
            read: None,
        }
    }

    /// Conecta ao WebSocket do X-Plane
    pub async fn connect(&mut self, url: &str) -> Result<()> {
        let parsed_url = Url::parse(url).context("Erro ao parsear URL do WebSocket")?;

        let (ws_stream, _) = connect_async(parsed_url).await
            .context("Erro ao conectar ao WebSocket")?;

        let (write, read) = ws_stream.split();
        self.write = Some(write);
        self.read = Some(read);

        println!("Conexão via WebSocket estabelecida!");
        Ok(())
    }

    /// Subscrição ao DataRef
    pub async fn subscribe(&mut self, dataref_id: u64, index: Option<serde_json::Value>, req_id: u64) -> Result<()> {
        if let Some(write) = &mut self.write {
            let subscription = json!({
                "req_id": req_id,
                "type": "dataref_subscribe_values",
                "params": {
                    "datarefs": [
                        {
                            "id": dataref_id,
                            "index": index
                        }
                    ]
                }
            });

            let message = serde_json::to_string(&subscription)
                .context("Erro ao serializar mensagem de subscrição")?;

            write.send(Message::Text(message))
                .await
                .context("Erro ao enviar mensagem de subscrição pelo WebSocket")?;

            println!("Subscrição enviada: DataRef ID {}", dataref_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("WebSocket não conectado. Não é possível subscrever."))
        }
    }

    /// Processa mensagens recebidas
    pub async fn process_messages(&mut self) {
        if let Some(read) = &mut self.read {
            while let Some(Ok(message)) = read.next().await {
                match message {
                    Message::Text(text) => {
                        println!("Mensagem recebida: {}", text);
                    }
                    _ => {
                        println!("Mensagem desconhecida recebida.");
                    }
                }
            }
        } else {
            println!("WebSocket não conectado. Não há mensagens para processar.");
        }
    }
}
