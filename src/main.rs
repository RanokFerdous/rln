mod app;
mod discovery;
mod identity;
mod intelligence;
mod storage;
mod transfer;
mod tui;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start the TUI dashboard
    #[arg(short, long)]
    dashboard: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("🛰️ Rust-LAN-Navigator (RLN) v2.0");
    
    if args.dashboard {
        // Start TUI
    } else {
        // Run discovery and exit
    }
    
    Ok(())
}
