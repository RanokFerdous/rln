#![allow(deprecated)]
use anyhow::{Context, Result};
use iroh::base::key::SecretKey;
use iroh::net::{endpoint::get_remote_node_id, Endpoint};
use std::sync::Arc;

/// A custom ALPN (Application-Layer Protocol Negotiation) for RLN.
/// This ensures our P2P nodes only accept traffic meant for this specific application.
pub const RLN_ALPN: &[u8] = b"rln/v2.0";

pub struct P2pNode {
    pub endpoint: Endpoint,
}

impl P2pNode {
    /// Initializes the Iroh MagicEndpoint using our previously generated Ed25519 secret key.
    pub async fn new(secret_key_bytes: &[u8; 32]) -> Result<Self> {
        // Convert our raw 32-byte secret into Iroh's native key format
        let secret_key = SecretKey::from_bytes(secret_key_bytes);

        // Build the Iroh Endpoint
        // We bind to an IPv4/IPv6 wildcard port. Iroh handles the NAT traversal.
        let endpoint = Endpoint::builder()
            .secret_key(secret_key)
            .alpns(vec![RLN_ALPN.to_vec()])
            .bind()
            .await
            .context("Failed to bind Iroh Endpoint")?;

        Ok(Self { endpoint })
    }

    /// Starts a background loop to listen for incoming, authenticated RLN connections.
    pub async fn listen_for_peers(self: Arc<Self>) -> Result<()> {
        let node_id = self.endpoint.node_id();
        println!("🚀 [P2P] Node online. Listening for incoming QUIC connections...");
        println!(
            "📡 [P2P] Share this Node ID with peers to connect: {}",
            node_id
        );

        // This is the main connection acceptance loop
        while let Some(incoming) = self.endpoint.accept().await {
            let connecting = match incoming.accept() {
                Ok(conn) => conn,
                Err(e) => {
                    eprintln!("⚠️ [P2P] Rejected incoming connection: {}", e);
                    continue;
                }
            };

            // Spawn a new task to handle the individual connection securely
            tokio::spawn(async move {
                match connecting.await {
                    Ok(connection) => {
                        // Because of Iroh's architecture, the connection is already mutually authenticated
                        // via Ed25519 signatures at this point!
                        let peer_id = get_remote_node_id(&connection).expect("Peer should have an ID");
                        println!(
                            "🤝 [P2P] Secure connection established with Peer: {}",
                            peer_id
                        );

                        // In Phase 4, we will accept bidirectional streams here to transfer files.
                    }
                    Err(e) => {
                        eprintln!("❌ [P2P] Handshake failed: {}", e);
                    }
                }
            });
        }

        Ok(())
    }
}
