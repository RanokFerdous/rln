//! Cryptographic identity management for the RLN node.
//!
//! Uses `iroh`'s native Ed25519 key primitives ([`iroh::SecretKey`] /
//! [`iroh::PublicKey`]) — no external `ed25519-dalek` dependency required.
//! The iroh 0.98 crate re-exports these at the crate root, keeping our
//! identity layer tightly integrated with the P2P networking stack.
//!
//! On first boot, a 32-byte secret key is generated and written to
//! `data/identity.key` with `chmod 600` permissions. On subsequent runs
//! the same key is loaded so that the node's `EndpointId` remains stable.
use anyhow::{Context, Result};
use iroh::{PublicKey, SecretKey};
use std::fs;
use std::path::Path;

pub struct NodeIdentity {
    pub secret_key: SecretKey,
    pub peer_id: PublicKey,
}

impl NodeIdentity {
    /// Loads the node's identity from disk, or generates a new one if it doesn't exist.
    pub fn load_or_generate<P: AsRef<Path>>(key_path: P) -> Result<Self> {
        let path = key_path.as_ref();

        if path.exists() {
            // Load existing key
            let key_bytes = fs::read(path).context("Failed to read identity file")?;

            // Ensure the file is exactly 32 bytes
            let secret_bytes: [u8; 32] = key_bytes
                .as_slice()
                .try_into()
                .context("Invalid key file size. Expected 32 bytes.")?;

            let secret_key = SecretKey::from_bytes(&secret_bytes);
            let peer_id = secret_key.public();

            Ok(Self {
                secret_key,
                peer_id,
            })
        } else {
            // Generate a brand new keypair
            let secret_key = SecretKey::generate();
            let peer_id = secret_key.public();

            // Save the secret bytes to disk
            fs::write(path, secret_key.to_bytes()).context("Failed to write identity file")?;

            // Restrict file permissions to the current user only (Unix-only feature)
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(path)?.permissions();
                perms.set_mode(0o600); // Read/Write for owner, nothing for others
                fs::set_permissions(path, perms)
                    .context("Failed to set strict file permissions")?;
            }

            Ok(Self {
                secret_key,
                peer_id,
            })
        }
    }

    /// Returns the Peer ID as a human-readable hex string for logs and TUI.
    pub fn peer_id_hex(&self) -> String {
        self.peer_id.to_string()
    }

    /// Returns the raw 32 bytes of the private key to feed into our P2P networking stack.
    pub fn secret_bytes(&self) -> [u8; 32] {
        self.secret_key.to_bytes()
    }
}
