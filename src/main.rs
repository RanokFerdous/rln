mod discovery;
mod identity;
mod storage;
mod transfer;

use anyhow::Result;
use discovery::l2_scanner;
use discovery::mdns_scanner;
use identity::keys::NodeIdentity;
use std::path::Path;
use std::sync::Arc;
use storage::db::Database;
use storage::drift::{calculate_drift, DriftEvent};
use transfer::stream::P2pNode;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🛰️ Starting RLN v2.0 Initialization...");

    // 1. Initialize Storage & Fetch History
    let db = Database::new("data/rln_state.db")?;
    let historical_snapshots = db.get_all_snapshots()?;
    println!(
        "✅ Database ready. Loaded {} known devices.",
        historical_snapshots.len()
    );

    // 2. Cryptographic Bootstrapping
    let key_path = "data/identity.key";
    let is_new_key = !Path::new(key_path).exists();

    let identity = NodeIdentity::load_or_generate(key_path)?;

    if is_new_key {
        println!("🔑 Generated NEW Cryptographic Identity.");
    } else {
        println!("🔑 Loaded existing Cryptographic Identity.");
    }
    println!("🛡️  Peer ID: {}", identity.peer_id_hex());

    // 3. Initialize P2P Networking (New in Milestone 2.2)
    // We pass the raw secret bytes from our identity into the Iroh node
    let p2p_node = Arc::new(P2pNode::new(&identity.secret_bytes()).await?);

    // Spawn the P2P listener in the background so it doesn't block our UI or scanners
    let _p2p_task = tokio::spawn({
        let node = Arc::clone(&p2p_node);
        async move {
            if let Err(e) = node.listen_for_peers().await {
                eprintln!("❌ P2P Listener crashed: {}", e);
            }
        }
    });

    // Let the P2P listener print its startup logs before the scanners take over the console
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    println!("--------------------------------------------------");

    // 3. Setup Network Interface
    let iface = l2_scanner::get_active_interface()?;
    if let Err(e) = l2_scanner::verify_privileges(&iface) {
        eprintln!("🔒 Security Alert: {}", e);
        return Ok(());
    }

    // 4. Launch Discovery Engines Concurrently
    println!("🚀 Launching Discovery Engines on {}...\n", iface.name);

    let arp_task = tokio::spawn({
        let iface = iface.clone();
        async move { l2_scanner::run_arp_sweep(&iface).await }
    });

    let mdns_task = tokio::spawn(async move { mdns_scanner::run_mdns_discoveryy().await });

    // Wait for tasks to finish and unwrap the thread/result layers
    let (arp_res, mdns_res) = tokio::join!(arp_task, mdns_task);

    // Combine the results into a single scan list
    let mut current_scan = arp_res??;
    let mut mdns_scan = mdns_res??;
    current_scan.append(&mut mdns_scan);

    // 5. Execute the Drift Engine
    println!("\n🧠 Analyzing Network Drift...");
    let drift_events = calculate_drift(&historical_snapshots, &current_scan);

    if drift_events.is_empty() {
        println!("  -> No devices found. Is the network down?");
    }

    // 6. Process and Display Events
    for event in drift_events {
        match event {
            DriftEvent::NoChange { mac } => {
                println!("  🟢 [STABLE]   MAC: {} is unchanged.", mac);
            }
            DriftEvent::NewDevice { mac, ip } => {
                println!("  🚨 [NEW]      MAC: {} appeared at IP: {}", mac, ip);
                // Save new device to DB so it doesn't trigger as "NEW" next time
                db.upsert_device(&mac, &ip, None)?;
            }
            DriftEvent::IpChanged {
                mac,
                old_ip,
                new_ip,
            } => {
                println!(
                    "  ⚠️ [DRIFT]    MAC: {} changed IP from {} to {}",
                    mac, old_ip, new_ip
                );
                // Update DB with new IP
                db.upsert_device(&mac, &new_ip, None)?;
            }
            DriftEvent::DeviceOffline { mac, last_ip } => {
                println!(
                    "  👻 [OFFLINE]  MAC: {} (last seen at {}) is missing.",
                    mac, last_ip
                );
            }
        }
    }

    println!("\n✅ Network scan and drift analysis complete.");
    // Keep the main thread alive temporarily for testing so the P2P node can listen
    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    Ok(())
}
