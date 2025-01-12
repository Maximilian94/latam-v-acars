use serde::Deserialize;
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
use std::collections::HashMap;

use crate::battery::BatteryState;

type WebSocketWrite = SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>, Message>;
type WebSocketRead = SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>>;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketResponse {
    #[serde(rename = "result")]
    ResultMessage {
        req_id: u64,
        success: bool,
    },
    #[serde(rename = "dataref_update_values")]
    DataRefUpdate {
        data: HashMap<String, Value>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Value {
    Single(f64),
    Array(Vec<f64>),
}

pub struct XPlaneClient {
    write: Option<WebSocketWrite>,
    read: Option<WebSocketRead>,
}

impl XPlaneClient {
    pub fn new() -> Self {
        Self {
            write: None,
            read: None,
        }
    }

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

    pub async fn process_messages(&mut self) {
        if let Some(read) = &mut self.read {
            while let Some(Ok(message)) = read.next().await {
                match message {
                    Message::Text(text) => {
                        match serde_json::from_str::<WebSocketResponse>(&text) {
                            Ok(WebSocketResponse::ResultMessage { req_id, success }) => {
                                println!("Mensagem de resultado recebida: req_id={}, success={}", req_id, success);
                            }
                            Ok(WebSocketResponse::DataRefUpdate { data }) => {
                                println!("Mensagem de atualização recebida: {:?}", data);
                                for (id, value) in data {
                                    match value {
                                        Value::Single(v) => {
                                            println!("ID: {}, Valor único: {}", id, v);
                                            if id == "2158853277856" {
                                                let battery_state = convert_battery_state(v);
                                                println!("Estado da bateria convertido: {:?}", battery_state);
                                            }
                                        }
                                        Value::Array(values) => {
                                            println!("ID: {}, Valores do array: {:?}", id, values);
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                println!("Erro ao deserializar mensagem: {}", err);
                            }
                        }
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

pub fn convert_battery_state(value: f64) -> BatteryState {
    match value {
        1.0 => BatteryState::Auto,
        0.0 => BatteryState::Off,
        _ => BatteryState::Unknown,
    }
}