use std::collections::HashMap;
use crate::xplane_rest::DataRef;

pub trait DataRefBehavior: Send + Sync {
    fn get_object_to_subscribe(&self) -> serde_json::Value;
    fn process_socket_response(&self, response: serde_json::Value);
    fn set_id(&mut self, id: u64);
}

pub struct AirbusFBWBatOHPArray {
    pub id: Option<u64>,       // O ID é parte da estrutura
    pub name: String,          // Nome do DataRef
}

impl AirbusFBWBatOHPArray {
    pub fn new() -> Self {
        Self {
            id: None, // Inicialmente, sem ID
            name: "AirbusFBW/BatOHPArray".to_string(),
        }
    }
}

impl DataRefBehavior for AirbusFBWBatOHPArray {
    fn get_object_to_subscribe(&self) -> serde_json::Value {
        // Verifica se o ID está presente
        if let Some(id) = self.id {
            serde_json::json!({
                "id": id,
                "name": self.name,
                "index": [0, 1],
            })
        } else {
            // Se o ID estiver ausente, envia uma mensagem indicando o problema
            serde_json::json!({
                "error": "ID não definido para o DataRef",
                "name": self.name
            })
        }
    }

    fn process_socket_response(&self, response: serde_json::Value) {
        println!(
            "Processando resposta para '{}': {:?}",
            self.name, response
        );
    }

    fn set_id(&mut self, id: u64) {
        self.id = Some(id);
    }
}

pub struct DataRefRegistry {
    datarefs: HashMap<String, Box<dyn DataRefBehavior + Send + Sync>>,
}

impl DataRefRegistry {
    pub fn new() -> Self {
        let mut datarefs = HashMap::new();
        
        let airbus_fbw_bat_ohp_array = AirbusFBWBatOHPArray::new();

        datarefs.insert("AirbusFBW/BatOHPArray".to_string(), Box::new(airbus_fbw_bat_ohp_array) as Box<dyn DataRefBehavior + Send + Sync>);

        Self {
            datarefs: datarefs,
        }
    }

    pub fn update_dataref_id(&mut self, datarefs: Vec<DataRef>) {
        for dataref in datarefs {
            if let Some(obj) = self.datarefs.get_mut(&dataref.name) {
                // Faz o log do objeto encontrado
                println!("Objeto encontrado para '{}': {:?}", dataref.name, obj.get_object_to_subscribe());

                obj.set_id(dataref.id);

                println!("Objeto encontrado para '{}': {:?}", dataref.name, obj.get_object_to_subscribe());
            } else {
                // Se não encontrado, exibe um aviso
                // println!("Aviso: DataRef '{}' não encontrado no registro!", dataref.name);
            }
        }
    }
}
