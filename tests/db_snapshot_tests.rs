use lan_asin::storage::db::Database;

#[test]
fn test_database_initialization() {
    // Should initialize successfully in-memory
    let db = Database::new(":memory:");
    assert!(db.is_ok(), "Database failed to initialize");
}

#[test]
fn test_upsert_and_retrieve_devices() {
    let db = Database::new(":memory:").unwrap();

    // Verify it starts empty
    let initial = db.get_all_snapshots().unwrap();
    assert_eq!(initial.len(), 0);

    // Insert a new device
    db.upsert_device("00:1A:2B:3C:4D:5E", "192.168.1.10", Some("Router"))
        .unwrap();
    
    let snapshots = db.get_all_snapshots().unwrap();
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].mac_address, "00:1A:2B:3C:4D:5E");
    assert_eq!(snapshots[0].ip_address, "192.168.1.10");
    assert_eq!(snapshots[0].service_name.as_deref(), Some("Router"));

    // Upsert the SAME device with a new IP and Service Name
    db.upsert_device("00:1A:2B:3C:4D:5E", "192.168.1.50", Some("Gateway"))
        .unwrap();

    let updated = db.get_all_snapshots().unwrap();
    assert_eq!(updated.len(), 1, "Upsert should not duplicate rows");
    assert_eq!(updated[0].ip_address, "192.168.1.50");
    assert_eq!(updated[0].service_name.as_deref(), Some("Gateway"));

    // Insert a SECOND device
    db.upsert_device("11:22:33:44:55:66", "10.0.0.1", None).unwrap();
    let final_snapshots = db.get_all_snapshots().unwrap();
    assert_eq!(final_snapshots.len(), 2);
}
