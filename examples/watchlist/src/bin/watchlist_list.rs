/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 8/3/26
******************************************************************************/

//! Example: List all watchlists
//!
//! This example demonstrates how to retrieve all watchlists for the active account.
//!
//! Run with:
//! ```bash
//! cargo run --bin watchlist_list
//! ```

use ig_client::application::client::Client;
use ig_client::application::interfaces::watchlist::WatchlistService;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("Starting watchlist list example");

    let client = Client::new();

    let watchlists = client.get_watchlists().await?;

    println!("\n=== Watchlists ===\n");
    println!(
        "{:<40} {:<20} {:<10} {:<10}",
        "NAME", "ID", "EDITABLE", "DELETEABLE"
    );
    println!("{}", "-".repeat(80));

    for watchlist in &watchlists.watchlists {
        println!(
            "{:<40} {:<20} {:<10} {:<10}",
            watchlist.name,
            watchlist.id,
            if watchlist.editable { "Yes" } else { "No" },
            if watchlist.deleteable { "Yes" } else { "No" }
        );
    }

    println!("\nTotal watchlists: {}", watchlists.watchlists.len());

    Ok(())
}
