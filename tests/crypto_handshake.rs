use lan_asin::identity::keys::NodeIdentity;
use std::env::temp_dir;
use std::fs;

#[test]
fn test_identity_generation_and_persistence() {
    let mut key_path = temp_dir();
    key_path.push("test_identity.key");

    // Clean up before test just in case
    let _ = fs::remove_file(&key_path);

    // 1. Generate new identity
    let identity1 = NodeIdentity::load_or_generate(&key_path).expect("Failed to generate identity");
    let peer_id_hex1 = identity1.peer_id_hex();

    assert!(key_path.exists(), "Key file should have been created");
    assert!(!peer_id_hex1.is_empty(), "Peer ID should not be empty");

    // 2. Load the existing identity
    let identity2 =
        NodeIdentity::load_or_generate(&key_path).expect("Failed to load existing identity");
    let peer_id_hex2 = identity2.peer_id_hex();

    // 3. Verify they match
    assert_eq!(
        peer_id_hex1, peer_id_hex2,
        "Loaded identity did not match the generated identity"
    );

    // Clean up after test
    let _ = fs::remove_file(&key_path);
}
