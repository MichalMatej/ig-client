/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 19/10/25
******************************************************************************/
use crate::prelude::{Account, Activity, MarketDetails};
use crate::presentation::account::{
    AccountTransaction, ActivityMetadata, Position, TransactionMetadata, WorkingOrder,
};
use crate::presentation::instrument::InstrumentType;
use crate::presentation::market::{
    Category, CategoryInstrument, CategoryInstrumentsMetadata, HistoricalPrice, MarketData,
    MarketNavigationNode, MarketNode, PriceAllowance,
};
use crate::presentation::order::{Direction, Status};
use crate::utils::parsing::{deserialize_null_as_empty_vec, deserialize_nullable_status};
use chrono::{DateTime, Utc};
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Database entry response for market instruments
#[derive(
    DebugPretty, DisplaySimple, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default,
)]
pub struct DBEntryResponse {
    /// The trading symbol identifier
    pub symbol: String,
    /// The Epic identifier used by the exchange
    pub epic: String,
    /// Human-readable name of the instrument
    pub name: String,
    /// Instrument type classification
    pub instrument_type: InstrumentType,
    /// The exchange where this instrument is traded
    pub exchange: String,
    /// Expiration date and time for the instrument
    pub expiry: String,
    /// Timestamp of the last update to this record
    pub last_update: DateTime<Utc>,
}

impl From<MarketNode> for DBEntryResponse {
    fn from(value: MarketNode) -> Self {
        let mut entry = DBEntryResponse::default();
        if !value.markets.is_empty() {
            let market = &value.markets[0];
            entry.symbol = market
                .epic
                .split('.')
                .nth(2)
                .unwrap_or_default()
                .to_string();
            entry.epic = market.epic.clone();
            entry.name = market.instrument_name.clone();
            entry.instrument_type = market.instrument_type;
            entry.exchange = "IG".to_string();
            entry.expiry = market.expiry.clone();
            entry.last_update = Utc::now();
        }
        entry
    }
}

impl From<MarketData> for DBEntryResponse {
    fn from(market: MarketData) -> Self {
        DBEntryResponse {
            symbol: market
                .epic
                .split('.')
                .nth(2)
                .unwrap_or_default()
                .to_string(),
            epic: market.epic.clone(),
            name: market.instrument_name.clone(),
            instrument_type: market.instrument_type,
            exchange: "IG".to_string(),
            expiry: market.expiry.clone(),
            last_update: Utc::now(),
        }
    }
}

impl From<&MarketNode> for DBEntryResponse {
    fn from(value: &MarketNode) -> Self {
        DBEntryResponse::from(value.clone())
    }
}

impl From<&MarketData> for DBEntryResponse {
    fn from(market: &MarketData) -> Self {
        DBEntryResponse::from(market.clone())
    }
}

/// Response containing multiple market details
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct MultipleMarketDetailsResponse {
    /// List of market details
    #[serde(rename = "marketDetails")]
    pub market_details: Vec<MarketDetails>,
}

impl MultipleMarketDetailsResponse {
    /// Returns the number of market details in the response
    ///
    /// # Returns
    /// Number of market details
    #[must_use]
    pub fn len(&self) -> usize {
        self.market_details.len()
    }

    /// Returns true if the response contains no market details
    ///
    /// # Returns
    /// True if empty, false otherwise
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.market_details.is_empty()
    }

    /// Returns a reference to the market details vector
    ///
    /// # Returns
    /// Reference to the vector of market details
    #[must_use]
    pub fn market_details(&self) -> &Vec<MarketDetails> {
        &self.market_details
    }

    /// Returns an iterator over the market details
    ///
    /// # Returns
    /// Iterator over market details
    pub fn iter(&self) -> impl Iterator<Item = &MarketDetails> {
        self.market_details.iter()
    }
}

/// Model for historical prices
#[derive(DebugPretty, Clone, Serialize, Deserialize)]
pub struct HistoricalPricesResponse {
    /// List of historical price points
    pub prices: Vec<HistoricalPrice>,
    /// Type of the instrument
    #[serde(rename = "instrumentType")]
    pub instrument_type: InstrumentType,
    /// API usage allowance information
    #[serde(rename = "allowance", skip_serializing_if = "Option::is_none", default)]
    pub allowance: Option<PriceAllowance>,
}

impl HistoricalPricesResponse {
    /// Returns the number of price points in the response
    ///
    /// # Returns
    /// Number of price points
    #[must_use]
    pub fn len(&self) -> usize {
        self.prices.len()
    }

    /// Returns true if the response contains no price points
    ///
    /// # Returns
    /// True if empty, false otherwise
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.prices.is_empty()
    }

    /// Returns a reference to the prices vector
    ///
    /// # Returns
    /// Reference to the vector of historical prices
    #[must_use]
    pub fn prices(&self) -> &Vec<HistoricalPrice> {
        &self.prices
    }

    /// Returns an iterator over the prices
    ///
    /// # Returns
    /// Iterator over historical prices
    pub fn iter(&self) -> impl Iterator<Item = &HistoricalPrice> {
        self.prices.iter()
    }
}

/// Model for market search results
#[derive(DebugPretty, Clone, Serialize, Deserialize)]
pub struct MarketSearchResponse {
    /// List of markets matching the search criteria
    pub markets: Vec<MarketData>,
}

impl MarketSearchResponse {
    /// Returns the number of markets in the response
    ///
    /// # Returns
    /// Number of markets
    #[must_use]
    pub fn len(&self) -> usize {
        self.markets.len()
    }

    /// Returns true if the response contains no markets
    ///
    /// # Returns
    /// True if empty, false otherwise
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.markets.is_empty()
    }

    /// Returns a reference to the markets vector
    ///
    /// # Returns
    /// Reference to the vector of markets
    #[must_use]
    pub fn markets(&self) -> &Vec<MarketData> {
        &self.markets
    }

    /// Returns an iterator over the markets
    ///
    /// # Returns
    /// Iterator over markets
    pub fn iter(&self) -> impl Iterator<Item = &MarketData> {
        self.markets.iter()
    }
}

/// Response model for market navigation
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize)]
pub struct MarketNavigationResponse {
    /// List of navigation nodes at the current level
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub nodes: Vec<MarketNavigationNode>,
    /// List of markets at the current level
    #[serde(default, deserialize_with = "deserialize_null_as_empty_vec")]
    pub markets: Vec<MarketData>,
}

/// Response containing all categories of instruments enabled for the IG account
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize, Default)]
pub struct CategoriesResponse {
    /// List of categories
    pub categories: Vec<Category>,
}

impl CategoriesResponse {
    /// Returns the number of categories in the response
    ///
    /// # Returns
    /// Number of categories
    #[must_use]
    pub fn len(&self) -> usize {
        self.categories.len()
    }

    /// Returns true if the response contains no categories
    ///
    /// # Returns
    /// True if empty, false otherwise
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.categories.is_empty()
    }

    /// Returns a reference to the categories vector
    ///
    /// # Returns
    /// Reference to the vector of categories
    #[must_use]
    pub fn categories(&self) -> &Vec<Category> {
        &self.categories
    }

    /// Returns an iterator over the categories
    ///
    /// # Returns
    /// Iterator over categories
    pub fn iter(&self) -> impl Iterator<Item = &Category> {
        self.categories.iter()
    }
}

/// Response containing instruments for a specific category
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize, Default)]
pub struct CategoryInstrumentsResponse {
    /// List of instruments in the category
    pub instruments: Vec<CategoryInstrument>,
    /// Paging metadata
    pub metadata: Option<CategoryInstrumentsMetadata>,
}

impl CategoryInstrumentsResponse {
    /// Returns the number of instruments in the response
    ///
    /// # Returns
    /// Number of instruments
    #[must_use]
    pub fn len(&self) -> usize {
        self.instruments.len()
    }

    /// Returns true if the response contains no instruments
    ///
    /// # Returns
    /// True if empty, false otherwise
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.instruments.is_empty()
    }

    /// Returns a reference to the instruments vector
    ///
    /// # Returns
    /// Reference to the vector of instruments
    #[must_use]
    pub fn instruments(&self) -> &Vec<CategoryInstrument> {
        &self.instruments
    }

    /// Returns an iterator over the instruments
    ///
    /// # Returns
    /// Iterator over instruments
    pub fn iter(&self) -> impl Iterator<Item = &CategoryInstrument> {
        self.instruments.iter()
    }
}

/// Response containing user accounts
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize, Default)]
pub struct AccountsResponse {
    /// List of accounts owned by the user
    pub accounts: Vec<Account>,
}

/// Open positions
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize, Default)]
pub struct PositionsResponse {
    /// List of open positions
    pub positions: Vec<Position>,
}

impl PositionsResponse {
    /// Compact positions by epic, combining positions with the same epic
    ///
    /// This method takes a vector of positions and returns a new vector where
    /// positions with the same epic have been combined into a single position.
    ///
    /// # Arguments
    /// * `positions` - A vector of positions to compact
    ///
    /// # Returns
    /// A vector of positions with unique epics
    pub fn compact_by_epic(positions: Vec<Position>) -> Vec<Position> {
        let mut epic_map: HashMap<String, Position> = std::collections::HashMap::new();

        for position in positions {
            let epic = position.market.epic.clone();
            epic_map
                .entry(epic)
                .and_modify(|existing| {
                    *existing = existing.clone() + position.clone();
                })
                .or_insert(position);
        }

        epic_map.into_values().collect()
    }
}

/// Working orders
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize)]
pub struct WorkingOrdersResponse {
    /// List of pending working orders
    #[serde(rename = "workingOrders")]
    pub working_orders: Vec<WorkingOrder>,
}

/// Account activity
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize)]
pub struct AccountActivityResponse {
    /// List of activities on the account
    pub activities: Vec<Activity>,
    /// Metadata about pagination
    pub metadata: Option<ActivityMetadata>,
}

/// Transaction history
#[derive(DebugPretty, DisplaySimple, Clone, Deserialize, Serialize)]
pub struct TransactionHistoryResponse {
    /// List of account transactions
    pub transactions: Vec<AccountTransaction>,
    /// Metadata about the transaction list
    pub metadata: TransactionMetadata,
}

/// Response to order creation
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct CreateOrderResponse {
    /// Client-generated reference for the deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

/// Response to closing a position
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct ClosePositionResponse {
    /// Client-generated reference for the closing deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

/// Response to updating a position
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct UpdatePositionResponse {
    /// Client-generated reference for the update deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

/// Response to working order creation
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct CreateWorkingOrderResponse {
    /// Client-generated reference for the deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
}

/// Details of a confirmed order
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct OrderConfirmationResponse {
    /// Date and time of the confirmation
    pub date: String,
    /// Status of the order (accepted, rejected, etc.)
    /// This can be null in some responses (e.g., when market is closed)
    #[serde(deserialize_with = "deserialize_nullable_status")]
    pub status: Status,
    /// Reason for rejection if applicable
    pub reason: Option<String>,
    /// Unique identifier for the deal
    #[serde(rename = "dealId")]
    pub deal_id: Option<String>,
    /// Client-generated reference for the deal
    #[serde(rename = "dealReference")]
    pub deal_reference: String,
    /// Status of the deal
    #[serde(rename = "dealStatus")]
    pub deal_status: Option<String>,
    /// Instrument EPIC identifier
    pub epic: Option<String>,
    /// Expiry date for the order
    #[serde(rename = "expiry")]
    pub expiry: Option<String>,
    /// Whether a guaranteed stop was used
    #[serde(rename = "guaranteedStop")]
    pub guaranteed_stop: Option<bool>,
    /// Price level of the order
    #[serde(rename = "level")]
    pub level: Option<f64>,
    /// Distance for take profit
    #[serde(rename = "limitDistance")]
    pub limit_distance: Option<f64>,
    /// Price level for take profit
    #[serde(rename = "limitLevel")]
    pub limit_level: Option<f64>,
    /// Size/quantity of the order
    pub size: Option<f64>,
    /// Distance for stop loss
    #[serde(rename = "stopDistance")]
    pub stop_distance: Option<f64>,
    /// Price level for stop loss
    #[serde(rename = "stopLevel")]
    pub stop_level: Option<f64>,
    /// Whether a trailing stop was used
    #[serde(rename = "trailingStop")]
    pub trailing_stop: Option<bool>,
    /// Direction of the order (buy or sell)
    pub direction: Option<Direction>,
}

impl std::fmt::Display for MultipleMarketDetailsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use prettytable::format;
        use prettytable::{Cell, Row, Table};

        let mut table = Table::new();

        // Set table format
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        // Add header
        table.add_row(Row::new(vec![
            Cell::new("INSTRUMENT NAME"),
            Cell::new("EPIC"),
            Cell::new("BID"),
            Cell::new("OFFER"),
            Cell::new("MID"),
            Cell::new("SPREAD"),
            Cell::new("EXPIRY"),
            Cell::new("HIGH/LOW"),
        ]));

        // Sort by instrument name
        let mut sorted_details = self.market_details.clone();
        sorted_details.sort_by(|a, b| {
            a.instrument
                .name
                .to_lowercase()
                .cmp(&b.instrument.name.to_lowercase())
        });

        // Add rows
        for details in &sorted_details {
            let bid = details
                .snapshot
                .bid
                .map(|b| format!("{:.2}", b))
                .unwrap_or_else(|| "-".to_string());

            let offer = details
                .snapshot
                .offer
                .map(|o| format!("{:.2}", o))
                .unwrap_or_else(|| "-".to_string());

            let mid = match (details.snapshot.bid, details.snapshot.offer) {
                (Some(b), Some(o)) => format!("{:.2}", (b + o) / 2.0),
                _ => "-".to_string(),
            };

            let spread = match (details.snapshot.bid, details.snapshot.offer) {
                (Some(b), Some(o)) => format!("{:.2}", o - b),
                _ => "-".to_string(),
            };

            // Use expiry directly (shorter than last_dealing_date)
            let expiry = details
                .instrument
                .expiry_details
                .as_ref()
                .map(|ed| {
                    // Extract just the date part (YYYY-MM-DD)
                    ed.last_dealing_date
                        .split('T')
                        .next()
                        .unwrap_or(&ed.last_dealing_date)
                        .to_string()
                })
                .unwrap_or_else(|| {
                    details
                        .instrument
                        .expiry
                        .split('T')
                        .next()
                        .unwrap_or(&details.instrument.expiry)
                        .to_string()
                });

            let high_low = format!(
                "{}/{}",
                details
                    .snapshot
                    .high
                    .map(|h| format!("{:.2}", h))
                    .unwrap_or_else(|| "-".to_string()),
                details
                    .snapshot
                    .low
                    .map(|l| format!("{:.2}", l))
                    .unwrap_or_else(|| "-".to_string())
            );

            // Truncate long names to make room for EPIC
            let name = if details.instrument.name.len() > 30 {
                format!("{}...", &details.instrument.name[0..27])
            } else {
                details.instrument.name.clone()
            };

            // Don't truncate EPIC - show it complete
            let epic = details.instrument.epic.clone();

            table.add_row(Row::new(vec![
                Cell::new(&name),
                Cell::new(&epic),
                Cell::new(&bid),
                Cell::new(&offer),
                Cell::new(&mid),
                Cell::new(&spread),
                Cell::new(&expiry),
                Cell::new(&high_low),
            ]));
        }

        write!(f, "{}", table)
    }
}

impl std::fmt::Display for HistoricalPricesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use prettytable::format;
        use prettytable::{Cell, Row, Table};

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        // Add header
        table.add_row(Row::new(vec![
            Cell::new("SNAPSHOT TIME"),
            Cell::new("OPEN BID"),
            Cell::new("OPEN ASK"),
            Cell::new("HIGH BID"),
            Cell::new("HIGH ASK"),
            Cell::new("LOW BID"),
            Cell::new("LOW ASK"),
            Cell::new("CLOSE BID"),
            Cell::new("CLOSE ASK"),
            Cell::new("VOLUME"),
        ]));

        // Add rows
        for price in &self.prices {
            let open_bid = price
                .open_price
                .bid
                .map(|v| format!("{:.4}", v))
                .unwrap_or_else(|| "-".to_string());

            let open_ask = price
                .open_price
                .ask
                .map(|v| format!("{:.4}", v))
                .unwrap_or_else(|| "-".to_string());

            let high_bid = price
                .high_price
                .bid
                .map(|v| format!("{:.4}", v))
                .unwrap_or_else(|| "-".to_string());

            let high_ask = price
                .high_price
                .ask
                .map(|v| format!("{:.4}", v))
                .unwrap_or_else(|| "-".to_string());

            let low_bid = price
                .low_price
                .bid
                .map(|v| format!("{:.4}", v))
                .unwrap_or_else(|| "-".to_string());

            let low_ask = price
                .low_price
                .ask
                .map(|v| format!("{:.4}", v))
                .unwrap_or_else(|| "-".to_string());

            let close_bid = price
                .close_price
                .bid
                .map(|v| format!("{:.4}", v))
                .unwrap_or_else(|| "-".to_string());

            let close_ask = price
                .close_price
                .ask
                .map(|v| format!("{:.4}", v))
                .unwrap_or_else(|| "-".to_string());

            let volume = price
                .last_traded_volume
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string());

            table.add_row(Row::new(vec![
                Cell::new(&price.snapshot_time),
                Cell::new(&open_bid),
                Cell::new(&open_ask),
                Cell::new(&high_bid),
                Cell::new(&high_ask),
                Cell::new(&low_bid),
                Cell::new(&low_ask),
                Cell::new(&close_bid),
                Cell::new(&close_ask),
                Cell::new(&volume),
            ]));
        }

        // Add summary footer
        writeln!(f, "{}", table)?;
        writeln!(f, "\nSummary:")?;
        writeln!(f, "  Total price points: {}", self.prices.len())?;
        writeln!(f, "  Instrument type: {:?}", self.instrument_type)?;

        if let Some(allowance) = &self.allowance {
            writeln!(
                f,
                "  Remaining allowance: {}",
                allowance.remaining_allowance
            )?;
            writeln!(f, "  Total allowance: {}", allowance.total_allowance)?;
        }

        Ok(())
    }
}

impl std::fmt::Display for MarketSearchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use prettytable::format;
        use prettytable::{Cell, Row, Table};

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);

        // Add header
        table.add_row(Row::new(vec![
            Cell::new("INSTRUMENT NAME"),
            Cell::new("EPIC"),
            Cell::new("BID"),
            Cell::new("OFFER"),
            Cell::new("MID"),
            Cell::new("SPREAD"),
            Cell::new("EXPIRY"),
            Cell::new("TYPE"),
        ]));

        // Sort by instrument name
        let mut sorted_markets = self.markets.clone();
        sorted_markets.sort_by(|a, b| {
            a.instrument_name
                .to_lowercase()
                .cmp(&b.instrument_name.to_lowercase())
        });

        // Add rows
        for market in &sorted_markets {
            let bid = market
                .bid
                .map(|b| format!("{:.4}", b))
                .unwrap_or_else(|| "-".to_string());

            let offer = market
                .offer
                .map(|o| format!("{:.4}", o))
                .unwrap_or_else(|| "-".to_string());

            let mid = match (market.bid, market.offer) {
                (Some(b), Some(o)) => format!("{:.4}", (b + o) / 2.0),
                _ => "-".to_string(),
            };

            let spread = match (market.bid, market.offer) {
                (Some(b), Some(o)) => format!("{:.4}", o - b),
                _ => "-".to_string(),
            };

            // Truncate long names
            let name = if market.instrument_name.len() > 30 {
                format!("{}...", &market.instrument_name[0..27])
            } else {
                market.instrument_name.clone()
            };

            // Extract date from expiry
            let expiry = market
                .expiry
                .split('T')
                .next()
                .unwrap_or(&market.expiry)
                .to_string();

            let instrument_type = format!("{:?}", market.instrument_type);

            table.add_row(Row::new(vec![
                Cell::new(&name),
                Cell::new(&market.epic),
                Cell::new(&bid),
                Cell::new(&offer),
                Cell::new(&mid),
                Cell::new(&spread),
                Cell::new(&expiry),
                Cell::new(&instrument_type),
            ]));
        }

        writeln!(f, "{}", table)?;
        writeln!(f, "\nTotal markets found: {}", self.markets.len())?;

        Ok(())
    }
}

// ============================================================================
// WATCHLIST RESPONSES
// ============================================================================

/// Response containing all watchlists for the active account
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistsResponse {
    /// List of watchlists
    pub watchlists: Vec<Watchlist>,
}

/// A watchlist containing instruments
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct Watchlist {
    /// Watchlist identifier
    pub id: String,
    /// Watchlist name
    pub name: String,
    /// Whether the watchlist can be edited
    pub editable: bool,
    /// Whether the watchlist can be deleted
    pub deleteable: bool,
    /// Whether this is a default system watchlist
    #[serde(rename = "defaultSystemWatchlist")]
    pub default_system_watchlist: bool,
}

/// Response when creating a new watchlist
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct CreateWatchlistResponse {
    /// The ID of the created watchlist
    #[serde(rename = "watchlistId")]
    pub watchlist_id: String,
    /// Status of the operation
    pub status: String,
}

/// Response containing markets in a watchlist
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct WatchlistMarketsResponse {
    /// List of markets in the watchlist
    pub markets: Vec<MarketData>,
}

/// Generic status response for operations
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct StatusResponse {
    /// Status of the operation (e.g., "SUCCESS")
    pub status: String,
}

// ============================================================================
// CLIENT SENTIMENT RESPONSES
// ============================================================================

/// Response containing client sentiment for multiple markets
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct ClientSentimentResponse {
    /// List of client sentiments
    #[serde(rename = "clientSentiments")]
    pub client_sentiments: Vec<MarketSentiment>,
}

/// Client sentiment data for a single market
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct MarketSentiment {
    /// Market identifier
    #[serde(rename = "marketId")]
    pub market_id: String,
    /// Percentage of clients with long positions
    #[serde(rename = "longPositionPercentage")]
    pub long_position_percentage: f64,
    /// Percentage of clients with short positions
    #[serde(rename = "shortPositionPercentage")]
    pub short_position_percentage: f64,
}

// ============================================================================
// INDICATIVE COSTS RESPONSES
// ============================================================================

/// Response containing indicative costs and charges
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct IndicativeCostsResponse {
    /// Reference for the indicative quote
    #[serde(rename = "indicativeQuoteReference")]
    pub indicative_quote_reference: String,
    /// Costs and charges breakdown
    #[serde(rename = "costsAndCharges")]
    pub costs_and_charges: CostsAndCharges,
}

/// Breakdown of costs and charges
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct CostsAndCharges {
    /// Total cost percentage
    #[serde(rename = "totalCostPercentage")]
    pub total_cost_percentage: Option<f64>,
    /// Total cost amount
    #[serde(rename = "totalCostAmount")]
    pub total_cost_amount: Option<f64>,
    /// Currency
    pub currency: Option<String>,
    /// One-off costs
    #[serde(rename = "oneOffCosts")]
    pub one_off_costs: Option<CostBreakdown>,
    /// Ongoing costs
    #[serde(rename = "ongoingCosts")]
    pub ongoing_costs: Option<CostBreakdown>,
    /// Transaction costs
    #[serde(rename = "transactionCosts")]
    pub transaction_costs: Option<CostBreakdown>,
    /// Incidental costs
    #[serde(rename = "incidentalCosts")]
    pub incidental_costs: Option<CostBreakdown>,
}

/// Breakdown of a specific cost category
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct CostBreakdown {
    /// Percentage value
    pub percentage: Option<f64>,
    /// Monetary amount
    pub amount: Option<f64>,
}

/// Response containing historical costs
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct CostsHistoryResponse {
    /// List of historical costs
    pub costs: Vec<HistoricalCost>,
}

/// Historical cost entry
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct HistoricalCost {
    /// Date of the cost
    pub date: String,
    /// Deal reference
    #[serde(rename = "dealReference")]
    pub deal_reference: Option<String>,
    /// Epic of the instrument
    pub epic: Option<String>,
    /// Total cost amount
    #[serde(rename = "totalCost")]
    pub total_cost: Option<f64>,
    /// Currency
    pub currency: Option<String>,
}

/// Response containing a durable medium document
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct DurableMediumResponse {
    /// The durable medium document content (typically HTML or PDF)
    pub document: String,
}

// ============================================================================
// ACCOUNT PREFERENCES RESPONSES
// ============================================================================

/// Response containing account preferences
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct AccountPreferencesResponse {
    /// Whether trailing stops are enabled
    #[serde(rename = "trailingStopsEnabled")]
    pub trailing_stops_enabled: bool,
}

// ============================================================================
// OPERATIONS/APPLICATION RESPONSES
// ============================================================================

/// Response containing application details (a single application)
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationDetailsResponse {
    /// API key
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// Application name
    pub name: Option<String>,
    /// Application status
    pub status: String,
    /// Overall allowance for the account
    #[serde(rename = "allowanceAccountOverall")]
    pub allowance_account_overall: Option<i32>,
    /// Trading allowance for the account
    #[serde(rename = "allowanceAccountTrading")]
    pub allowance_account_trading: Option<i32>,
    /// Concurrent connections allowance
    #[serde(rename = "concurrentSubscriptionsLimit")]
    pub concurrent_subscriptions_limit: Option<i32>,
    /// Creation date
    #[serde(rename = "createdDate")]
    pub created_date: Option<String>,
}

/// Information about an API application
#[derive(DebugPretty, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationInfo {
    /// API key
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// Application name
    pub name: Option<String>,
    /// Application status
    pub status: String,
    /// Overall allowance for the account
    #[serde(rename = "allowanceAccountOverall")]
    pub allowance_account_overall: Option<i32>,
    /// Trading allowance for the account
    #[serde(rename = "allowanceAccountTrading")]
    pub allowance_account_trading: Option<i32>,
    /// Concurrent connections allowance
    #[serde(rename = "concurrentSubscriptionsLimit")]
    pub concurrent_subscriptions_limit: Option<i32>,
    /// Creation date
    #[serde(rename = "createdDate")]
    pub created_date: Option<String>,
}

// ============================================================================
// SINGLE POSITION RESPONSE
// ============================================================================

/// Response containing a single position
#[derive(DebugPretty, Clone, Serialize, Deserialize)]
pub struct SinglePositionResponse {
    /// Position details
    pub position: Position,
    /// Market data for the position
    pub market: MarketData,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::fs;

    #[test]
    fn test_deserialize_working_orders_from_file() -> Result<(), Box<dyn std::error::Error>> {
        // Load the JSON file
        let json_content = fs::read_to_string("Data/working_orders.json")?;

        // Parse as a generic JSON Value first to inspect the structure
        let json_value: Value = serde_json::from_str(&json_content)?;

        println!(
            "JSON structure:\n{}",
            serde_json::to_string_pretty(&json_value)?
        );

        // Attempt to deserialize into WorkingOrdersResponse
        let response: WorkingOrdersResponse = serde_json::from_str(&json_content)?;

        println!(
            "Successfully deserialized {} working orders",
            response.working_orders.len()
        );
        for (idx, order) in response.working_orders.iter().enumerate() {
            println!(
                "Order {}: epic={}, direction={:?}, size={}, level={}",
                idx + 1,
                order.working_order_data.epic,
                order.working_order_data.direction,
                order.working_order_data.order_size,
                order.working_order_data.order_level
            );
        }
        Ok(())
    }
}
