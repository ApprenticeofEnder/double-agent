use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,
}

// What do I actually want to do
// 1. Create global capabilities
// 2. Create per-agent capabilities
// 3. Sync with a watchdog?

fn main() {
    println!("Hello, world!");
}
