use tokio::sync::watch;
use tokio_stream::wrappers::WatchStream;
use tokio_stream::StreamExt;
use std::sync::Arc;
use tokio::task;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BatteryState {
    Unknown, // Estado inicial
    Auto,
    Off,
}

pub struct BatteryPushButton {
    pub name: String,
    sender: watch::Sender<BatteryState>,
}

impl Clone for BatteryPushButton {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            sender: self.sender.clone(),
        }
    }
}

impl BatteryPushButton {
    // Método de criação
    pub fn new(name: &str, initial_state: BatteryState) -> Self {
        let (sender, _receiver) = watch::channel(initial_state);
        Self {
            name: name.to_string(),
            sender,
        }
    }

    // Método para atualizar o estado
    pub fn set_state(&self, new_state: BatteryState) {
        println!("Alterando estado para: {:?}", new_state);
        let _ = self.sender.send(new_state);
    }

    // Método para obter um stream de atualizações
    pub fn subscribe(&self) -> impl tokio_stream::Stream<Item = BatteryState> + '_ {
        let receiver = self.sender.subscribe();
        WatchStream::new(receiver)
    }

    // Método para obter o estado atual
    pub fn get_state(&self) -> BatteryState {
        *self.sender.borrow()
    }
}
