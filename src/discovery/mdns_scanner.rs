#![allow(dead_code)]
use crate::storage::drift::ScannedDevice;
use anyhow::Result;
use simple_mdns::async_discovery::ServiceDiscovery;
use simple_mdns::InstanceInformation;
use std::time::Duration;

pub async fn run_mdns_scan() -> Result<Vec<ScannedDevice>> {
    println!("🌐 [L3] Starting mDNS discovery...");

    let instance_info = InstanceInformation::new("null".to_string());
    let discovery = ServiceDiscovery::new(instance_info, "_http._tcp.local", 60)?;

    // Allow time for devices to respond to the multicast query
    tokio::time::sleep(Duration::from_secs(3)).await;

    let services = discovery.get_known_services().await;
    let mut devices = Vec::new();

    for service in services {
        // Find the first IPv4 address if available
        let ip_addr = service.ip_addresses.iter().find(|ip| ip.is_ipv4());
        
        if let Some(std::net::IpAddr::V4(ipv4)) = ip_addr {
            devices.push(ScannedDevice {
                mac_address: "".to_string(), // mDNS doesn't resolve MAC directly
                ip_address: ipv4.to_string(),
                service_name: Some("_http._tcp.local".to_string()),
            });
        }
    }

    Ok(devices)
}
