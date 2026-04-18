use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Renders the main dashboard UI based on the current App state
pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(8),     // Network Topology
            Constraint::Length(6),  // Active Transfers
            Constraint::Length(8),  // Logs
        ])
        .split(f.area());

    // 1. Header
    let header_text = format!(
        " 🛰️  RLN v2.0 Dashboard | Known Devices: {} | Status: Active",
        app.known_devices
    );
    let header = Paragraph::new(header_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).title(" RLN Orchestrator "));
    f.render_widget(header, chunks[0]);

    // 2. Topology
    let mut topo_items = Vec::new();
    if app.topology.is_empty() {
        topo_items.push(ListItem::new("Scanning LLDP Topology..."));
    } else {
        for (switch_name, topo) in &app.topology {
            topo_items.push(ListItem::new(Line::from(vec![
                Span::styled("🔌 Switch: ", Style::default().fg(Color::Blue)),
                Span::styled(format!("{} ", switch_name), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                Span::styled(format!("(Port: {})", topo.port_id), Style::default().fg(Color::DarkGray)),
            ])));
            for device in &topo.devices {
                topo_items.push(ListItem::new(Line::from(format!(
                    "  ├─ {} ({}) - {}",
                    device.hostname.as_deref().unwrap_or("Unknown"),
                    device.ip_address,
                    device.mac_address,
                ))));
            }
        }
    }
    
    // Also include Drift Events quickly if there are any
    if !app.active_drift_events.is_empty() {
        topo_items.push(ListItem::new(""));
        topo_items.push(ListItem::new(Line::from(Span::styled("Drift Alerts:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))));
        for event in &app.active_drift_events {
            let log_str = format!("{:?}", event); // Keep it simple for now
            topo_items.push(ListItem::new(format!("  ! {}", log_str)));
        }
    }

    let topo_list = List::new(topo_items).block(
        Block::default().borders(Borders::ALL).title(" Network Topology (LLDP) ")
    );
    f.render_widget(topo_list, chunks[1]);

    // 3. Active Transfers
    let mut transfer_items = Vec::new();
    if app.active_transfers.is_empty() {
        transfer_items.push(ListItem::new("No active transfers."));
    } else {
        for t in &app.active_transfers {
            let line = Line::from(vec![
                Span::styled("📦 ", Style::default()),
                Span::styled(format!("{} ", t.filename), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(format!("-> {} ", t.peer_id), Style::default().fg(Color::Gray)),
                Span::styled(format!("[{}%] ", t.progress_pct), Style::default().fg(Color::Cyan)),
                Span::styled(format!("({:.1} MB/s)", t.speed_mbps), Style::default().fg(Color::DarkGray)),
            ]);
            transfer_items.push(ListItem::new(line));
        }
    }
    
    let transfers_list = List::new(transfer_items).block(
        Block::default().borders(Borders::ALL).title(" Active P2P Transfers ")
    );
    f.render_widget(transfers_list, chunks[2]);

    // 4. Logs
    let log_items: Vec<ListItem> = app.logs.iter().map(|log| ListItem::new(log.clone())).collect();
    let logs_list = List::new(log_items).block(
        Block::default().borders(Borders::ALL).title(" System Logs ")
    );
    f.render_widget(logs_list, chunks[3]);
}
