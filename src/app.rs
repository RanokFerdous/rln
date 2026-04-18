use crate::storage::drift::DriftEvent;
use crate::intelligence::topology::SwitchTopology;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TransferState {
    pub filename: String,
    pub peer_id: String,
    pub progress_pct: u8,
    pub speed_mbps: f64,
}

pub struct App {
    pub is_running: bool,
    pub logs: Vec<String>,
    pub known_devices: usize,
    pub active_drift_events: Vec<DriftEvent>,
    pub active_transfers: Vec<TransferState>,
    pub topology: HashMap<String, SwitchTopology>,
}

impl App {
    pub fn new(known_devices: usize) -> Self {
        Self {
            is_running: true,
            logs: vec!["[SYSTEM] RLN v2.0 Initialized...".to_string()],
            known_devices,
            active_drift_events: Vec::new(),
            active_transfers: Vec::new(),
            topology: HashMap::new(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_log_bounding() {
        let mut app = App::new(0);
        app.logs.clear(); // Clear initial logs for predictability

        // Add 60 logs
        for i in 0..60 {
            app.add_log(format!("Log message {}", i));
        }

        // Ensure we only have 50 logs
        assert_eq!(app.logs.len(), 50);

        // Ensure the oldest logs (0-9) were removed, and the first log is now 10
        assert_eq!(app.logs[0], "Log message 10");
        assert_eq!(app.logs.last().unwrap(), "Log message 59");
    }
}
