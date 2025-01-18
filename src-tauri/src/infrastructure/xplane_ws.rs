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
use crate::infrastructure::data_ref_registry::DataRefRegistry;

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

#[derive(Debug, Deserialize, Serialize)]
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

    pub async fn subscribe(
        &mut self,
        datarefs: Vec<serde_json::Value>, // Lista de DataRefs como parâmetro
        req_id: u64,
    ) -> Result<()> {
        if let Some(write) = &mut self.write {
            // Constrói o objeto de subscrição
            let subscription = json!({
                "req_id": req_id,
                "type": "dataref_subscribe_values",
                "params": {
                    "datarefs": datarefs // Usa os DataRefs passados como argumento
                }
            });
    
            // Serializa a mensagem para enviar via WebSocket
            let message = serde_json::to_string(&subscription)
                .context("Erro ao serializar mensagem de subscrição")?;
    
            // Envia a mensagem
            write.send(Message::Text(message))
                .await
                .context("Erro ao enviar mensagem de subscrição pelo WebSocket")?;
    
            println!("Subscrição enviada: {:?}", subscription);
            Ok(())
        } else {
            Err(anyhow::anyhow!("WebSocket não conectado. Não é possível subscrever."))
        }
    }

    pub async fn process_messages(&mut self, dataref_registry: &DataRefRegistry) {
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
                                for (id_str, value) in data {
                                    // Converte o ID para u64
                                    if let Ok(id) = id_str.parse::<u64>() {
                                        if let Some(dataref_name) = dataref_registry.id_to_name.get(&id) {
                                            if let Some(dataref) = dataref_registry.datarefs.get(dataref_name) {
                                                // Invoca o método de processamento do dataref
                                                dataref.process_socket_response(serde_json::json!({ "id": id, "value": value }));
                                            } else {
                                                println!("DataRef não encontrado para o nome '{}'", dataref_name);
                                            }
                                        } else {
                                            println!("ID '{}' não mapeado para nenhum DataRef", id);
                                        }
                                    } else {
                                        println!("Erro ao converter ID: {}", id_str);
                                    }
                                }
                            }
                            Err(err) => {
                                println!("Erro ao deserializar mensagem: {}", err);
                            }
                        }
                    }
                    _ => {
                        println!("Mensagem desconhecida recebida: {}", message);
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