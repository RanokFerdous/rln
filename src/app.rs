use crate::storage::drift::DriftEvent;

pub struct App {
    pub is_running: bool,
    pub logs: Vec<String>,
    pub known_devices: usize,
    pub active_drift_events: Vec<DriftEvent>,
}

impl App {
    pub fn new(known_devices: usize) -> Self {
        Self {
            is_running: true,
            logs: vec!["[SYSTEM] RLN v2.0 Initialized...".to_string()],
            known_devices,
            active_drift_events: Vec::new(),
        }
    }

    /// Handles shutting down the application securely
    pub fn quit(&mut self) {
        self.is_running = false;
    }

    /// Adds a log message to the UI
    pub fn add_log(&mut self, message: String) {
        self.logs.push(message);
        if self.logs.len() > 50 {
            self.logs.remove(0); // Keep logs bounded
        }
    }
}
