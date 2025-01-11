#[derive(Debug)]
pub struct BatteryPushButton {
    pub name: String,
    pub state: Option<bool>,
}

impl BatteryPushButton {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: None,
        }
    }

    pub fn get_state(&self) -> Option<bool> {
        self.state
    }
}