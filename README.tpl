<div style="text-align: center;">
<img src="https://raw.githubusercontent.com/joaquinbejar/ig-client/refs/heads/main/doc/images/logo.png" alt="ig-client" style="max-width: 400px; width: 50%;">
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

{{readme}}

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

## Contribution 

We welcome contributions to this project! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the project still builds and all tests pass.
4. Commit your changes and push your branch to your forked repository.
5. Submit a pull request to the main repository.

### **Contact Information**

If you have any questions, issues, or would like to provide feedback, please feel free to contact the project maintainer:

- **Author**: Joaquín Béjar García
- **Email**: jb@taunais.com
- **Telegram**: [@joaquin_bejar](https://t.me/joaquin_bejar)
- **Repository**: <https://github.com/joaquinbejar/ig-client>
- **Documentation**: <https://docs.rs/ig-client>

We appreciate your interest and look forward to your contributions!

## ✍️ License

Licensed under MIT license

## Disclaimer

This software is not officially associated with IG Markets. Trading financial instruments carries risk, and this library is provided as-is without any guarantees. Always test thoroughly with a demo account before using in a live trading environment.