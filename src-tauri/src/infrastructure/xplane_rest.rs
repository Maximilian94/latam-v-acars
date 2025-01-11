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
