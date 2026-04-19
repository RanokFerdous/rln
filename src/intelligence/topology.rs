//! LLDP topology parsing and switch hierarchy mapping.
//!
//! In a production deployment, `run_lldp_scan` would capture raw Ethernet frames
//! using `pnet` and parse them with `packet-dissector-lldp` to discover which devices
//! are connected to which switch port.
//!
//! The current implementation provides a deterministic mock that demonstrates the
//! expected data shape for UI development and integration testing.
use pnet::datalink::{self, Channel, NetworkInterface};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::Packet;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// A device discovered via Link Layer Discovery Protocol (LLDP).
#[derive(Debug, Clone)]
pub struct LldpDevice {
    /// The device's hardware (MAC) address.
    pub mac_address: String,
    /// The device's IPv4 address at time of discovery.
    pub ip_address: String,
    /// The LLDP system name, if advertised.
    pub hostname: Option<String>,
}

/// Represents a single network switch and the devices it reports as connected.
#[derive(Debug, Clone, Default)]
pub struct SwitchTopology {
    /// The LLDP system name of the switch.
    pub switch_name: String,
    /// The specific port on the switch where the RLN node received the LLDP frame.
    pub port_id: String,
    /// All devices that reported being connected via this switch.
    pub devices: Vec<LldpDevice>,
}

/// Scans for live LLDP advertisements on the given interface for a specified duration.
/// Parses the 802.1AB TLVs to extract the switch name and port ID.
pub async fn run_lldp_scan(iface: &NetworkInterface, listen_duration: Duration) -> HashMap<String, SwitchTopology> {
    let mut topology = HashMap::new();
    let config = datalink::Config {
        read_timeout: Some(Duration::from_millis(500)),
        ..Default::default()
    };

    let mut rx = match datalink::channel(iface, config) {
        Ok(Channel::Ethernet(_, rx)) => rx,
        _ => {
            eprintln!("[LLDP] Failed to open ethernet channel on interface.");
            return topology;
        }
    };

    let start_time = Instant::now();

    // Run blocking pnet loop in spawn_blocking
    let results = tokio::task::spawn_blocking(move || {
        let mut local_map: HashMap<String, SwitchTopology> = HashMap::new();

        while start_time.elapsed() < listen_duration {
            if let Ok(packet) = rx.next() {
                if let Some(eth) = EthernetPacket::new(packet) {
                    if eth.get_ethertype() == EtherTypes::Lldp {
                        let payload = eth.payload();
                        let (sys_name, port_id) = parse_lldp_payload(payload);

                        let switch = sys_name.unwrap_or_else(|| "Unknown Switch".to_string());
                        let port = port_id.unwrap_or_else(|| "Unknown Port".to_string());
                        
                        local_map.entry(switch.clone()).or_insert(SwitchTopology {
                            switch_name: switch,
                            port_id: port,
                            devices: Vec::new(),
                        });
                    }
                }
            }
        }
        local_map
    }).await.unwrap_or_default();

    // Extend topology with the captured results
    topology.extend(results);
    topology
}

/// Helper function to parse raw LLDP payload bytes into (System Name, Port ID).
pub fn parse_lldp_payload(payload: &[u8]) -> (Option<String>, Option<String>) {
    let mut sys_name = None;
    let mut port_id = None;

    let mut i = 0;
    while i + 2 <= payload.len() {
        let header = u16::from_be_bytes([payload[i], payload[i+1]]);
        let t_type = (header >> 9) & 0x7F;
        let t_len = (header & 0x1FF) as usize;

        i += 2;
        if i + t_len > payload.len() { break; }

        match t_type {
            0 => break, // End of LLDPDU
            2 => { // Port ID
                if t_len > 1 {
                    // Skip subtype byte, just take the raw string/bytes
                    port_id = Some(String::from_utf8_lossy(&payload[i+1..i+t_len]).into_owned());
                }
            }
            5 => { // System Name
                sys_name = Some(String::from_utf8_lossy(&payload[i..i+t_len]).into_owned());
            }
            _ => {}
        }
        i += t_len;
    }

    (sys_name, port_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lldp_payload_valid() {
        let mut payload = Vec::new();
        
        // Port ID TLV (Type 2, Length 5) -> header 0x0405. Data: subtype 0x07, "eth0"
        payload.extend_from_slice(&[0x04, 0x05, 0x07, b'e', b't', b'h', b'0']);
        
        // System Name TLV (Type 5, Length 6) -> header 0x0A06. Data: "Switch"
        payload.extend_from_slice(&[0x0A, 0x06, b'S', b'w', b'i', b't', b'c', b'h']);
        
        // End of LLDPDU TLV (Type 0, Length 0) -> header 0x0000.
        payload.extend_from_slice(&[0x00, 0x00]);

        let (sys_name, port_id) = parse_lldp_payload(&payload);
        
        assert_eq!(sys_name.as_deref(), Some("Switch"));
        assert_eq!(port_id.as_deref(), Some("eth0"));
    }

    #[test]
    fn test_parse_lldp_payload_incomplete() {
        let mut payload = Vec::new();
        
        // Port ID TLV (Type 2, Length 10) but string ends early
        payload.extend_from_slice(&[0x04, 0x0A, 0x07, b'e', b't', b'h', b'0']);

        // Should return cleanly without panicking
        let (sys_name, port_id) = parse_lldp_payload(&payload);
        
        // Since length 10 wasn't satisfied, it should break and return None
        assert_eq!(sys_name, None);
        assert_eq!(port_id, None);
    }
}