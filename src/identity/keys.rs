use anyhow::{Context, Result};
use ed25519_dalek::{SecretKey, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
use std::path::Path;

pub struct NodeIdentity {
    pub signing_key: SigningKey, // Private key (keep secret!)
    pub peer_id: VerifyingKey,   // Public key (safe to share)
}

impl NodeIdentity {
    /// Loads the node's identity from disk, or generates a new one if it doesn't exist.
    pub fn load_or_generate<P: AsRef<Path>>(key_path: P) -> Result<Self> {
        let path = key_path.as_ref();

        if path.exists() {
            // Load existing key
            let key_bytes = fs::read(path).context("Failed to read identity file")?;

            // Ensure the file is exactly 32 bytes (the size of an Ed25519 secret key)
            let secret_bytes: [u8; 32] = key_bytes
                .as_slice()
                .try_into()
                .context("Invalid key file size. Expected 32 bytes.")?;

            let signing_key = SigningKey::from_bytes(&secret_bytes);
            let peer_id = signing_key.verifying_key();

            Ok(Self {
                signing_key,
                peer_id,
            })
        } else {
            // Generate a brand new keypair
            let mut csprng = OsRng;
            let signing_key = SigningKey::generate(&mut csprng);
            let peer_id = signing_key.verifying_key();

            // Save the secret bytes to disk
            fs::write(path, signing_key.to_bytes()).context("Failed to write identity file")?;

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
                signing_key,
                peer_id,
            })
        }
    }

    /// Returns the Peer ID as a human-readable hex string for logs and TUI.
    pub fn peer_id_hex(&self) -> String {
        hex::encode(self.peer_id.as_bytes())
    }

    /// Returns the raw 32 bytes of the private key to feed into our P2P networking stack.
    pub fn secret_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }
}
