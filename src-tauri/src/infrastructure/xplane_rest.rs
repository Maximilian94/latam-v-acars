use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataRef {
    pub id: u64,
    pub name: String,
    pub is_writable: bool,
    pub value_type: String,
}

#[derive(Debug, Deserialize)]
pub struct DataRefResponse {
    pub data: Vec<DataRef>,
}

pub async fn fetch_datarefs() -> Result<Vec<DataRef>, anyhow::Error> {
    let url = "http://localhost:8086/api/v1/datarefs";
    let client = Client::new();

    print!("Enviando requisição para {}", url);

    let response = client.get(url).send().await.context("Erro ao enviar requisição para listar DataRefs")?;
    println!("Resposta recebida com status: {}", response.status());

    if response.status().is_success() {
        let datarefs_response: DataRefResponse = response.json().await.context("Erro ao desserializar resposta dos DataRefs")?;
        println!("DataRefs recebidos com sucesso!");
        Ok(datarefs_response.data)
    } else {
        let error_message = format!("Erro ao listar DataRefs: {}", response.status());
        println!("{}", error_message);
        anyhow::bail!(error_message);
    }
}



// use reqwest::Client;
// use serde::Deserialize;
// use anyhow::{Result, Context};

// #[derive(Debug, Deserialize)]
// pub struct DataRefsResponse {
//     pub datarefs: Vec<DataRef>,
// }

// #[derive(Debug, Deserialize)]
// pub struct DataRef {
//     pub id: u64,
//     pub is_writable: bool,
//     pub name: String,
//     pub value_type: String,
// }

// pub async fn list_datarefs() -> Result<Vec<DataRef>> {
//     println!("Iniciando list_datarefs...");

//     let url = "http://localhost:8086/api/v1/datarefs";
//     let client = Client::new();

//     println!("Enviando requisição para {}", url);
//     let response = client
//         .get(url)
//         .send()
//         .await
//         .context("Erro ao enviar requisição para listar datarefs")?;

//     println!("Resposta recebida com status: {}", response.status());
//     if response.status().is_success() {
//         let body = response
//             .text()
//             .await
//             .context("Erro ao obter corpo da resposta")?;
    
//         println!("Corpo da resposta: {}", &body[..200]); // Mostra os primeiros 200 caracteres do JSON
    
//         let datarefs_response: DataRefsResponse = serde_json::from_str(&body)
//             .context("Erro ao desserializar resposta dos datarefs")?;
    
//         let datarefs = datarefs_response.datarefs;
    
//         println!("DataRefs recebidos com sucesso!");
//         for dataref in &datarefs {
//             println!(
//                 "- ID: {} | Name: {} | Writable: {} | Type: {}",
//                 dataref.id,
//                 dataref.name,
//                 dataref.is_writable, // Corrigido para is_writable
//                 dataref.value_type
//             );
//         }
    
//         Ok(datarefs)
//     } else {
//         let error_message = format!("Erro ao listar datarefs: {}", response.status());
//         println!("{}", error_message);
//         anyhow::bail!(error_message);
//     }
// }
