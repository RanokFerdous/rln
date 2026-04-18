use lan_asin::intelligence::topology::run_lldp_scan;

#[tokio::test]
async fn test_topology_mock_structure() {
    let topology = run_lldp_scan().await;

    // We expect 2 mock switches defined in the function
    assert_eq!(topology.len(), 2, "Expected exactly 2 mock switches");

    // Verify Core-Switch-01 exists and has devices
    let core_switch = topology.get("Core-Switch-01").expect("Missing Core-Switch-01");
    assert_eq!(core_switch.switch_name, "Core-Switch-01");
    assert_eq!(core_switch.port_id, "GigabitEthernet1/0/1");
    assert_eq!(core_switch.devices.len(), 2, "Core switch should have 2 connected devices");

    // Verify Access-Switch-02 exists
    let access_switch = topology.get("Access-Switch-02").expect("Missing Access-Switch-02");
    assert_eq!(access_switch.switch_name, "Access-Switch-02");
    assert_eq!(access_switch.devices.len(), 1, "Access switch should have 1 connected device");

    // Check specific device on Access switch
    let printer = &access_switch.devices[0];
    assert_eq!(printer.mac_address, "11:22:33:44:55:66");
    assert_eq!(printer.ip_address, "192.168.1.50");
    assert_eq!(printer.hostname.as_deref(), Some("Brother-Printer"));
}
