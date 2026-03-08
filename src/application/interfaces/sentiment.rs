/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 8/3/26
******************************************************************************/

//! Client sentiment service interface for IG Markets API
//!
//! This module defines the interface for retrieving client sentiment data,
//! which shows the percentage of IG clients who are long vs short on instruments.

use crate::error::AppError;
use crate::model::responses::{ClientSentimentResponse, MarketSentiment};
use async_trait::async_trait;

/// Service for retrieving client sentiment data from the IG Markets API
///
/// Client sentiment shows the percentage of IG clients holding long versus
/// short positions on a given instrument, providing insight into market positioning.
#[async_trait]
pub trait SentimentService: Send + Sync {
    /// Returns client sentiment for multiple markets
    ///
    /// # Arguments
    /// * `market_ids` - List of market IDs to get sentiment for
    ///
    /// # Returns
    /// * `Ok(ClientSentimentResponse)` - Sentiment data for the requested markets
    /// * `Err(AppError)` - If the request fails
    async fn get_client_sentiment(
        &self,
        market_ids: &[String],
    ) -> Result<ClientSentimentResponse, AppError>;

    /// Returns client sentiment for a single market
    ///
    /// # Arguments
    /// * `market_id` - The market ID to get sentiment for
    ///
    /// # Returns
    /// * `Ok(MarketSentiment)` - Sentiment data for the market
    /// * `Err(AppError)` - If the request fails
    async fn get_client_sentiment_by_market(
        &self,
        market_id: &str,
    ) -> Result<MarketSentiment, AppError>;

    /// Returns client sentiment for related markets
    ///
    /// Returns a list of related (also traded) client sentiment for
    /// the given instrument's market.
    ///
    /// # Arguments
    /// * `market_id` - The market ID to get related sentiments for
    ///
    /// # Returns
    /// * `Ok(ClientSentimentResponse)` - Sentiment data for related markets
    /// * `Err(AppError)` - If the request fails
    async fn get_related_sentiment(
        &self,
        market_id: &str,
    ) -> Result<ClientSentimentResponse, AppError>;
}
