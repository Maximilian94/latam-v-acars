use std::sync::Arc;

use crate::domain::systems::electrical::battery::{BatteryPushButton, BatteryState};
use crate::infrastructure::{xplane_ws, xplane_rest};
use tokio::join;
use futures_util::StreamExt;
use tokio::sync::oneshot;

pub struct ConnectToXPlaneUseCase {
    pub battery_1: BatteryPushButton,
    pub battery_2: BatteryPushButton,
}

impl ConnectToXPlaneUseCase {
    pub fn new() -> Self {
        Self {
            battery_1: BatteryPushButton::new("Battery 1", BatteryState::Unknown),
            battery_2: BatteryPushButton::new("Battery 2", BatteryState::Unknown),
        }
    }

    pub async fn execute(&mut self) -> Result<(), String> {
        let mut xplane_client = xplane_ws::XPlaneClient::new();

        let (ws_result, datarefs_result) = join!(
            xplane_client.connect("ws://localhost:8086/api/v1"),
            xplane_rest::fetch_datarefs()
        );

        let datarefs = datarefs_result.map_err(|e| e.to_string())?;

        let battery_dataref = datarefs
            .iter() // Itera sobre &DataRef
            .find(|d| d.name == "AirbusFBW/BatOHPArray") // Verifica o campo name
            .ok_or("DataRef da bateria não encontrado")?;

        println!("DataRef da bateria encontrado: ID {}", battery_dataref.id);

        xplane_client
            .subscribe(battery_dataref.id, Some(serde_json::json!(0)), 9998)
            .await
            .map_err(|e| e.to_string())?;

            // Processar mensagens recebidas do WebSocket
        println!("Iniciando processamento de mensagens...");
        xplane_client.process_messages().await;

        let (tx, rx) = oneshot::channel();

        let battery1_clone = self.battery_1.clone();
        let subscriber1 = tokio::spawn(async move {
            let mut stream = battery1_clone.subscribe();
            let _ = tx.send(());

            while let Some(state) = stream.next().await {
                println!("Subscriber 1 recebeu atualização: {:?}", state);
            }
            println!("Subscriber 1: Stream encerrado.");
        });

        let _ = rx.await;

        self.battery_1.set_state(BatteryState::Auto);

        let ws_state = ws_result.map_err(|e| e.to_string())?;

        println!("Lista de DataRefs disponíveis:");
        // for dataref in &datarefs {
        //     println!(
        //         "- {} (Writable: {}) - Type: {}",
        //         dataref.name, dataref.is_writable, dataref.value_type
        //     );
        // }

        Ok(ws_state)
    }
}
