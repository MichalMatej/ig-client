<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/joaquinbejar/ig-client/refs/heads/main/doc/images/logo.png" alt="ig-client" style="width: 80%; height: 80%;">
</div>

[![Dual License](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/ig-client.svg)](https://crates.io/crates/ig-client)
[![Downloads](https://img.shields.io/crates/d/ig-client.svg)](https://crates.io/crates/ig-client)
[![Stars](https://img.shields.io/github/stars/joaquinbejar/ig-client.svg)](https://github.com/joaquinbejar/ig-client/stargazers)
[![Issues](https://img.shields.io/github/issues/joaquinbejar/ig-client.svg)](https://github.com/joaquinbejar/ig-client/issues)
[![PRs](https://img.shields.io/github/issues-pr/joaquinbejar/ig-client.svg)](https://github.com/joaquinbejar/ig-client/pulls)
[![Build Status](https://img.shields.io/github/workflow/status/joaquinbejar/ig-client/CI)](https://github.com/joaquinbejar/ig-client/actions)
[![Coverage](https://img.shields.io/codecov/c/github/joaquinbejar/ig-client)](https://codecov.io/gh/joaquinbejar/ig-client)
[![Dependencies](https://img.shields.io/librariesio/github/joaquinbejar/ig-client)](https://libraries.io/github/joaquinbejar/ig-client)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/ig-client)
[![Wiki](https://img.shields.io/badge/wiki-latest-blue.svg)](https://deepwiki.com/joaquinbejar/ig-client)

## IG Markets API Client for Rust

A comprehensive Rust client for interacting with the IG Markets trading API. This library provides a type-safe and ergonomic way to access IG Markets' REST and WebSocket APIs for trading and market data retrieval.

### Overview

The IG Markets API Client for Rust is designed to provide a reliable and efficient interface to the IG Markets trading platform. It handles authentication, session management, and all API interactions while providing a clean, idiomatic Rust interface for developers.

### Features

- **Authentication**: Secure authentication with the IG Markets API including session refresh and account switching
- **Account Management**: Access account information, balances, preferences, and activity history
- **Market Data**: Retrieve market data, prices, instrument details, and historical prices
- **Order Management**: Create, modify, and close positions and orders with various order types
- **Working Orders**: Create, update, and manage working orders with support for limit and stop orders
- **Watchlists**: Full CRUD operations for watchlists including adding/removing instruments
- **Client Sentiment**: Access client sentiment data for single or multiple markets
- **Indicative Costs**: Retrieve indicative costs and charges for opening, closing, or editing positions
- **Transaction History**: Access detailed transaction and activity history
- **WebSocket Support**: Real-time market data streaming via WebSocket connections
- **Advanced Rate Limiting**: Sophisticated rate limiting with automatic backoff, concurrent request management, and explicit rate limit error handling
- **Fully Documented**: Comprehensive documentation for all components and methods
- **Error Handling**: Robust error handling and reporting with detailed error types
- **Type Safety**: Strong type checking for API requests and responses
- **Async Support**: Built with async/await for efficient non-blocking operations
- **Concurrency Management**: Built-in semaphores and thread-safe primitives for handling concurrent API requests
- **Configurable**: Flexible configuration options for different environments (demo/live)
- **Persistence**: Optional database integration for storing historical data
- **Database Support**: Integration with SQLx for storing and retrieving transaction data
- **Serialization Utilities**: Custom serialization helpers for handling IG Markets API responses

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ig-client = "0.11.0"
tokio = { version = "1", features = ["full"] }  # For async runtime
dotenv = "0.15"                                 # For environment variable loading
tracing = "0.1"                                # For logging
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres"] }  # Optional for database support
```

#### Requirements

- Rust 1.56 or later (for async/await support)
- An IG Markets account (demo or live)
- API credentials from IG Markets
- PostgreSQL database (optional, for data persistence)

### Configuration

Create a `.env` file in your project root with the following variables:

```
IG_USERNAME=your_username
IG_PASSWORD=your_password
IG_API_KEY=your_api_key
IG_ACCOUNT_ID=your_account_id
IG_BASE_URL=https://demo-api.ig.com/gateway/deal  # Use demo or live as needed
IG_TIMEOUT=30  # HTTP request timeout in seconds
IG_WS_URL=wss://demo-apd.marketdatasystems.com  # WebSocket URL
IG_WS_RECONNECT=5  # WebSocket reconnect interval in seconds
DATABASE_URL=postgres://user:password@localhost/ig_db  # Optional for data persistence
IG_DB_MAX_CONN=5  # Maximum database connections
TX_LOOP_INTERVAL_HOURS=1  # Transaction loop interval in hours
TX_PAGE_SIZE=20  # Transaction page size
TX_DAYS_BACK=7  # Number of days to look back for transactions
```

### Usage Examples

#### Complete Example Application

Here's a complete example showing how to set up the client, authenticate, and perform various operations:

```rust
use ig_client::application::services::account_service::{AccountService, IgAccountService};
use ig_client::application::services::market_service::{IgMarketService, MarketService};
use ig_client::application::services::order_service::{IgOrderService, OrderService};
use ig_client::application::models::order::{CreateOrderRequest, Direction};
use ig_client::config::Config;
use ig_client::session::auth::{IgAuth, IgAuthenticator};
use std::sync::Arc;
use dotenv::dotenv;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), ig_client::error::AppError> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Load environment variables
    dotenv().ok();
    info!("Environment variables loaded");

    // Create configuration
    let config = Arc::new(Config::new());
    info!("Configuration created");

    // Authenticate
    let auth = IgAuth::new(&config);
    let session = auth.login().await?;
    info!("Authentication successful");

    // Create services
    let account_service = IgAccountService::new(config.clone());
    let market_service = IgMarketService::new(config.clone());
    // Get account information
    let account_info = account_service.get_accounts(&session).await?;
    info!("Account information retrieved: {} accounts", account_info.accounts.len());

    // Switch to a different account if needed
    if account_info.accounts.len() > 1 {
        let target_account = &account_info.accounts[1];
        let updated_session = auth.switch_account(&session, &target_account.account_id, Some(true)).await?;
        info!("Switched to account: {}", updated_session.account_id);
    }

    // Search for a market
    let search_term = "US 500";
    let search_results = market_service.search_markets(&session, search_term).await?;
    info!("Found {} markets matching '{}'", search_results.markets.len(), search_term);

    // Get market details for the first result
    if let Some(market) = search_results.markets.first() {
        let epic = &market.epic;
        let market_details = market_service.get_market_details(&session, epic).await?;
        info!("Market details for {}: {}", epic, market_details.instrument.name);

        // Get historical prices
        let prices = market_service.get_prices_by_date(
            &session,
            epic,
            "MINUTE",
            "1",
            "2023-01-01T00:00:00",
            "2023-01-02T00:00:00"
        ).await?;
        info!("Retrieved {} price points", prices.prices.len());

        // Check if the market is tradable
        if market_details.snapshot.market_status == "TRADEABLE" {
            // Create a market order
            let order_request = CreateOrderRequest::market(
                epic.clone(),
                Direction::Buy,
                1.0, // Size
            );

            let order_result = order_service.create_order(&session, &order_request).await?;
            info!("Order placed: deal reference = {}", order_result.deal_reference);

            // Get positions
            let positions = account_service.get_positions(&session).await?;
            info!("Current positions: {}", positions.positions.len());
        }
    }

    info!("Example completed successfully");
    Ok(())
}
```

#### Authentication

```rust
use ig_client::session::auth::IgAuth;
use ig_client::config::Config;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), ig_client::error::AppError> {
    // Load configuration from environment variables
    let config = Arc::new(Config::new());

    // Create authentication handler
    let auth = IgAuth::new(config.clone());

    // Authenticate and get a session
    let session = auth.authenticate().await?;

    info!("Successfully authenticated!");
    Ok(())
}
```

#### Getting Account Information

```rust
use ig_client::application::services::account_service::{AccountService, IgAccountService};
use std::sync::Arc;

// Create account service
let account_service = IgAccountService::new(config.clone());

// Get account information
let account_info = account_service.get_accounts(&session).await?;
info!("Accounts: {:?}", account_info);

// Get positions
let positions = account_service.get_positions(&session).await?;
info!("Open positions: {}", positions.positions.len());

// Get transaction history
let from_date = chrono::Utc::now() - chrono::Duration::days(7);
let to_date = chrono::Utc::now();
let transactions = account_service.get_transactions(&session, from_date, to_date).await?;
info!("Transactions in the last week: {}", transactions.transactions.len());
```

#### Market Data

```rust
use ig_client::application::services::market_service::{MarketService, IgMarketService};

// Create market service
let market_service = IgMarketService::new(config.clone());

// Search for markets
let search_result = market_service.search_markets(&session, "Apple").await?;
info!("Found {} markets matching 'Apple'", search_result.markets.len());

// Get market details
if let Some(market) = search_result.markets.first() {
    let details = market_service.get_market_details(&session, &market.epic).await?;
    info!("Market details for {}: {}", market.instrument_name, details.instrument.name);

    // Get historical prices
    let prices = market_service.get_prices(
        &session,
        &market.epic,
        "DAY",   // Resolution
        30,      // Number of data points
    ).await?;
    info!("Retrieved {} historical price points", prices.prices.len());
}
```

#### Placing and Managing Orders

```rust
use ig_client::application::services::order_service::{OrderService, IgOrderService};
use ig_client::application::models::order::{CreateOrderRequest, Direction, OrderType, TimeInForce};

// Create order service
let order_service = IgOrderService::new(config.clone());

// Create a market order
let market_order = CreateOrderRequest::market(
    "OP.D.OTCDAX1.021100P.IP".to_string(),  // EPIC
    Direction::Buy,                     // Direction
    1.0,                               // Size
    None,                              // Limit level
    None,                              // Stop level
);

// Place the order
let result = order_service.create_order(&session, &market_order).await?;
info!("Market order placed: {:?}", result);

// Create a limit order
let limit_order = CreateOrderRequest {
    epic: "OP.D.OTCDAX1.021100P.IP".to_string(),
    direction: Direction::Buy,
    size: 1.0,
    order_type: OrderType::Limit,
    level: Some(1.05),  // Limit price
    guaranteed_stop: false,
    time_in_force: TimeInForce::GoodTillDate,
    good_till_date: Some("2025-06-01T12:00:00".to_string()),
    stop_level: None,
    stop_distance: None,
    limit_level: None,
    limit_distance: None,
    deal_reference: Some("my-custom-reference".to_string()),
};

let result = order_service.create_order(&session, &limit_order).await?;
info!("Limit order placed: {:?}", result);

// Close a position
let positions = account_service.get_positions(&session).await?;
if let Some(position) = positions.positions.first() {
    let close_request = order_service.close_position(
        &session,
        &position.position.deal_id,
        position.position.direction.clone(),
        position.position.size,
    ).await?;
    info!("Position closed: {:?}", close_request);
}
```

#### WebSocket Streaming

```rust
use ig_client::application::services::market_listener::{MarketListener, MarketListenerCallback};
use ig_client::application::models::market::MarketData;
use std::sync::Arc;
use tokio::sync::mpsc;

// Create a channel to receive market updates
let (tx, mut rx) = mpsc::channel(100);

// Create a callback function to handle market updates
let callback: MarketListenerCallback = Arc::new(move |market_data: &MarketData| {
    let data = market_data.clone();
    let tx = tx.clone();
    tokio::spawn(async move {
        let _ = tx.send(data).await;
    });
    Ok(())
});

// Create and start the market listener
let listener = MarketListener::new(callback);
listener.connect(&session).await?;

// Subscribe to market updates
let epics = vec!["OP.D.OTCDAX1.021100P.IP", "CS.D.USDJPY.CFD.IP"];
listener.subscribe(&epics).await?;

// Process market updates
while let Some(market_data) = rx.recv().await {
    info!("Market update for {}: bid={}, offer={}",
             market_data.epic, market_data.bid.unwrap_or(0.0), market_data.offer.unwrap_or(0.0));
}
```

### Available Services

The library provides the following service traits for interacting with the IG Markets API:

#### AccountService
- `get_accounts()` - Get all accounts for the authenticated user
- `get_positions()` - Get all open positions
- `get_working_orders()` - Get all working orders
- `get_activity(from, to)` - Get account activity for a date range
- `get_activity_by_period(period_ms)` - Get activity for a period in milliseconds
- `get_transactions(from, to)` - Get transaction history
- `get_preferences()` - Get account preferences
- `update_preferences(trailing_stops_enabled)` - Update account preferences

#### MarketService
- `search_markets(term)` - Search for markets by keyword
- `get_market_details(epic)` - Get detailed market information
- `get_multiple_market_details(epics)` - Get details for multiple markets
- `get_historical_prices(epic, resolution, num_points)` - Get historical price data
- `get_historical_prices_by_date_range(epic, resolution, start, end)` - Get prices for date range
- `get_market_navigation()` - Get market navigation hierarchy
- `get_categories()` - Get market categories
- `get_instruments_by_category(category)` - Get instruments in a category

#### OrderService
- `create_order(request)` - Create a new market order
- `get_order_confirmation(deal_reference)` - Get order confirmation
- `update_position(deal_id, update)` - Update an existing position
- `close_position(request)` - Close a position
- `get_position(deal_id)` - Get a single position by deal ID
- `create_working_order(request)` - Create a working order
- `update_working_order(deal_id, update)` - Update an existing working order
- `delete_working_order(deal_id)` - Delete a working order

#### WatchlistService (New in 0.11.0)
- `get_watchlists()` - Get all watchlists
- `create_watchlist(name, epics)` - Create a new watchlist
- `get_watchlist(id)` - Get watchlist markets
- `delete_watchlist(id)` - Delete a watchlist
- `add_to_watchlist(id, epic)` - Add instrument to watchlist
- `remove_from_watchlist(id, epic)` - Remove instrument from watchlist

#### SentimentService (New in 0.11.0)
- `get_client_sentiment(market_ids)` - Get sentiment for multiple markets
- `get_client_sentiment_by_market(market_id)` - Get sentiment for a single market
- `get_related_sentiment(market_id)` - Get sentiment for related markets

#### CostsService (New in 0.11.0)
- `get_indicative_costs_open(request)` - Get costs for opening a position
- `get_indicative_costs_close(request)` - Get costs for closing a position
- `get_indicative_costs_edit(request)` - Get costs for editing a position
- `get_costs_history(from, to)` - Get historical costs
- `get_durable_medium(quote_reference)` - Get durable medium document

#### OperationsService (New in 0.11.0)
- `get_client_apps()` - Get API application details
- `disable_client_app()` - Disable current API key

### Documentation

Comprehensive documentation is available for all components of the library. The documentation includes detailed explanations of all modules, structs, and functions, along with examples of how to use them.

#### API Documentation

You can access the API documentation on [docs.rs](https://docs.rs/ig-client) or generate it locally with:

```bash
make doc-open
```

#### Architecture

The library is organized into several modules:

- **config**: Configuration handling and environment variable loading
- **session**: Authentication and session management
- **application**: Core business logic and services
  - **models**: Data structures for API requests and responses
  - **services**: Service implementations for different API areas
- **transport**: HTTP and WebSocket communication with the IG Markets API
- **utils**: Utility functions for parsing, logging, etc.
- **error**: Error types and handling

### Development

This project includes a comprehensive Makefile with commands for common development tasks.

#### Building

```bash
make build        # Debug build
make release      # Release build
```

#### Testing

```bash
make test         # Run all tests
```

#### Code Quality

```bash
make fmt          # Format code with rustfmt
make lint         # Run clippy linter
make doc          # Check documentation coverage
make check        # Run tests, format check, and linting
make pre-push     # Run all checks before pushing code
```

#### Documentation

```bash
make doc-open     # Generate and open documentation
```

#### Code Coverage

```bash
make coverage     # Generate code coverage report (XML)
make coverage-html # Generate HTML coverage report
make open-coverage # Open HTML coverage report
```

#### Benchmarking

```bash
make bench        # Run benchmarks
make bench-show   # Show benchmark results
```

#### Rate Limiting

The library includes a sophisticated rate limiting system to comply with IG Markets API restrictions:

- **Multiple Rate Limit Types**: Different limits for trading, non-trading, and historical data requests
- **Thread-Safe Implementation**: Uses `tokio::sync::Mutex` for safe concurrent access
- **Automatic Backoff**: Dynamically calculates wait times based on request history
- **Explicit Rate Limit Handling**: Detects and handles rate limit errors from the API
- **Global Semaphore**: Limits concurrent API requests to prevent overwhelming the API
- **Configurable Safety Margins**: Adjustable safety margins to stay below API limits
- **Rate Limit Error Recovery**: Automatic cooldown and recovery when rate limits are exceeded

Example of configuring rate limits:

```rust
// Create a configuration with custom rate limit settings
let config = Arc::new(Config::with_rate_limit_type(
    RateLimitType::NonTradingAccount,  // Type of rate limit to enforce
    0.8,                               // Safety margin (80% of actual limit)
));

// The rate limiter will automatically be used by all services
let http_client = IgHttpClientImpl::new(config.clone());
let auth = IgAuth::new(config.clone());

// When rate limits are exceeded, the system will automatically:
// 1. Detect the rate limit error from the API
// 2. Enforce a mandatory cooldown period
// 3. Gradually resume requests with appropriate delays
```

#### Continuous Integration

```bash
make workflow     # Run all CI workflow steps locally
```

### Project Structure

```
├── src/
│   ├── application/       # Core business logic
│   │   ├── interfaces/    # Service trait interfaces
│   │   │   ├── account.rs # Account service trait
│   │   │   ├── costs.rs   # Costs service trait (v0.11.0)
│   │   │   ├── market.rs  # Market service trait
│   │   │   ├── operations.rs # Operations service trait (v0.11.0)
│   │   │   ├── order.rs   # Order service trait
│   │   │   ├── sentiment.rs # Sentiment service trait (v0.11.0)
│   │   │   └── watchlist.rs # Watchlist service trait (v0.11.0)
│   │   ├── auth.rs        # Authentication handler
│   │   ├── client.rs      # Main API client
│   │   └── config.rs      # Configuration handling
│   ├── model/             # Data models
│   │   ├── requests.rs    # API request models
│   │   ├── responses.rs   # API response models
│   │   └── streaming.rs   # Streaming data models
│   ├── presentation/      # Presentation layer
│   │   ├── account.rs     # Account presentation
│   │   ├── market.rs      # Market presentation
│   │   └── order.rs       # Order presentation
│   ├── storage/           # Data persistence
│   │   └── config.rs      # Database configuration
│   ├── constants.rs       # Global constants
│   ├── error.rs           # Error types
│   └── utils/             # Utility functions
│       ├── parsing.rs     # Parsing utilities
│       └── retry.rs       # Retry utilities
├── examples/              # Example applications
│   ├── chart/             # Chart examples
│   ├── costs/             # Costs examples (v0.11.0)
│   ├── market/            # Market examples
│   ├── orders/            # Order examples
│   ├── positions/         # Position examples
│   ├── sentiment/         # Sentiment examples (v0.11.0)
│   ├── streaming/         # Streaming examples
│   ├── watchlist/         # Watchlist examples (v0.11.0)
│   └── other/             # Other examples
├── tests/                 # Tests
│   ├── integration/       # Integration tests
│   └── unit/              # Unit tests
└── Makefile              # Development commands
```

### Contributing

Contributions are welcome! Here's how you can contribute:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes and commit them: `git commit -m 'Add some feature'`
4. Run the tests: `make test`
5. Push to the branch: `git push origin feature/my-feature`
6. Submit a pull request

Please make sure your code passes all tests and linting checks before submitting a pull request.

## What's New in 0.11.0

This release adds comprehensive API coverage with the following new services:

### New Services
- **WatchlistService** - Full CRUD operations for watchlists
  - `get_watchlists()`, `create_watchlist()`, `get_watchlist()`, `delete_watchlist()`
  - `add_to_watchlist()`, `remove_from_watchlist()`
- **SentimentService** - Client sentiment data
  - `get_client_sentiment()`, `get_client_sentiment_by_market()`, `get_related_sentiment()`
- **CostsService** - Indicative costs and charges
  - `get_indicative_costs_open()`, `get_indicative_costs_close()`, `get_indicative_costs_edit()`
  - `get_costs_history()`, `get_durable_medium()`
- **OperationsService** - API application management
  - `get_client_apps()`, `disable_client_app()`

### Extended Services
- **AccountService** - Added `get_preferences()`, `update_preferences()`, `get_activity_by_period()`
- **OrderService** - Added `get_position()`, `update_working_order()`

### New Examples
- `examples/watchlist/` - Watchlist management examples
- `examples/sentiment/` - Client sentiment examples
- `examples/costs/` - Indicative costs examples
- Additional examples in `positions/`, `orders/`, and `other/`

## Contribution and Contact

We welcome contributions to this project! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the project still builds and all tests pass.
4. Commit your changes and push your branch to your forked repository.
5. Submit a pull request to the main repository.

If you have any questions, issues, or would like to provide feedback, please feel free to contact the project maintainer:

**Joaquin Bejar Garcia**
- Email: jb@taunais.com
- GitHub: [joaquinbejar](https://github.com/joaquinbejar)

We appreciate your interest and look forward to your contributions!

## ✍️ License

Licensed under MIT license

## Disclaimer

This software is not officially associated with IG Markets. Trading financial instruments carries risk, and this library is provided as-is without any guarantees. Always test thoroughly with a demo account before using in a live trading environment.
