use std::sync::Arc;

use crate::domain::systems::electrical::battery::{BatteryPushButton, BatteryState};
use crate::infrastructure::{xplane_ws, xplane_rest};
use tokio::join;
use futures_util::StreamExt;
use tokio::sync::oneshot;
use crate::infrastructure::data_ref_registry::DataRefRegistry;

pub struct ConnectToXPlaneUseCase {

}

impl ConnectToXPlaneUseCase {
    pub fn new() -> Self {
        Self {
        }
    }

    pub async fn execute(&mut self) -> Result<(), String> {
        let mut dataref_registry = DataRefRegistry::new();

        // Busca os DataRefs e atualiza o registro
        let datarefs = xplane_rest::fetch_datarefs()
            .await
            .map_err(|e| e.to_string())?;

        dataref_registry.update_dataref_id(datarefs);

        //////////////////////////////////////////////////////////////////////////////////////////////////////////
        let mut xplane_client = xplane_ws::XPlaneClient::new();
        xplane_client
            .connect("ws://localhost:8086/api/v1")
            .await
            .map_err(|e| e.to_string())?;

        println!("Conexão ao WebSocket concluída!");

        if let Some(dataref) = dataref_registry
            .datarefs
            .get("AirbusFBW/BatOHPArray")
        {
            // Obtém o JSON para subscrição
            let subscription_object = dataref.get_object_to_subscribe();

            // Cria o array com o objeto de subscrição
            let datarefs = vec![subscription_object];

            // Chama o método subscribe com o array de DataRefs
            xplane_client
                .subscribe(datarefs, 9998)
                .await
                .map_err(|e| e.to_string())?;
        } else {
            println!("DataRef 'AirbusFBW/BatOHPArray' não encontrado no registro.");
        }

        // Processar mensagens recebidas do WebSocket
        println!("Iniciando processamento de mensagens...");
        xplane_client.process_messages(&dataref_registry).await;

        Ok(())
    }
}
