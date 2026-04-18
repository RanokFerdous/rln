#![allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LldpDevice {
    pub mac_address: String,
    pub ip_address: String,
    pub hostname: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SwitchTopology {
    pub switch_name: String,
    pub port_id: String,
    pub devices: Vec<LldpDevice>,
}

pub async fn run_lldp_scan() -> HashMap<String, SwitchTopology> {
    // Simulating packet-dissector-lldp parsing logic
    let mut topology = HashMap::new();

    topology.insert(
        "Core-Switch-01".to_string(),
        SwitchTopology {
            switch_name: "Core-Switch-01".to_string(),
            port_id: "GigabitEthernet1/0/1".to_string(),
            devices: vec![
                LldpDevice {
                    mac_address: "00:1B:44:11:3A:B7".to_string(),
                    ip_address: "192.168.1.10".to_string(),
                    hostname: Some("Router".to_string()),
                },
                LldpDevice {
                    mac_address: "AA:BB:CC:DD:EE:FF".to_string(),
                    ip_address: "192.168.1.105".to_string(),
                    hostname: Some("Desktop-PC".to_string()),
                },
            ],
        },
    );

    topology.insert(
        "Access-Switch-02".to_string(),
        SwitchTopology {
            switch_name: "Access-Switch-02".to_string(),
            port_id: "FastEthernet0/1".to_string(),
            devices: vec![
                LldpDevice {
                    mac_address: "11:22:33:44:55:66".to_string(),
                    ip_address: "192.168.1.50".to_string(),
                    hostname: Some("Brother-Printer".to_string()),
                },
            ],
        },
    );

    topology
}