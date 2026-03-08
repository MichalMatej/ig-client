/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 8/3/26
******************************************************************************/

//! Watchlist service interface for IG Markets API
//!
//! This module defines the interface for managing watchlists, including
//! creating, retrieving, updating, and deleting watchlists and their contents.

use crate::error::AppError;
use crate::model::responses::{
    CreateWatchlistResponse, StatusResponse, WatchlistMarketsResponse, WatchlistsResponse,
};
use async_trait::async_trait;

/// Service for managing watchlists in the IG Markets API
///
/// Watchlists allow users to organize and track groups of instruments.
/// This trait provides methods for full CRUD operations on watchlists.
#[async_trait]
pub trait WatchlistService: Send + Sync {
    /// Returns all watchlists belonging to the active account
    ///
    /// # Returns
    /// * `Ok(WatchlistsResponse)` - List of all watchlists
    /// * `Err(AppError)` - If the request fails
    async fn get_watchlists(&self) -> Result<WatchlistsResponse, AppError>;

    /// Creates a new watchlist with the given name and optional initial instruments
    ///
    /// # Arguments
    /// * `name` - The name for the new watchlist
    /// * `epics` - Optional list of EPICs to add to the watchlist
    ///
    /// # Returns
    /// * `Ok(CreateWatchlistResponse)` - The created watchlist ID and status
    /// * `Err(AppError)` - If the request fails
    async fn create_watchlist(
        &self,
        name: &str,
        epics: Option<&[String]>,
    ) -> Result<CreateWatchlistResponse, AppError>;

    /// Returns the markets in a specific watchlist
    ///
    /// # Arguments
    /// * `watchlist_id` - The ID of the watchlist to retrieve
    ///
    /// # Returns
    /// * `Ok(WatchlistMarketsResponse)` - The watchlist's markets
    /// * `Err(AppError)` - If the request fails
    async fn get_watchlist(&self, watchlist_id: &str)
    -> Result<WatchlistMarketsResponse, AppError>;

    /// Deletes a watchlist
    ///
    /// # Arguments
    /// * `watchlist_id` - The ID of the watchlist to delete
    ///
    /// # Returns
    /// * `Ok(StatusResponse)` - The deletion status
    /// * `Err(AppError)` - If the request fails
    async fn delete_watchlist(&self, watchlist_id: &str) -> Result<StatusResponse, AppError>;

    /// Adds an instrument to a watchlist
    ///
    /// # Arguments
    /// * `watchlist_id` - The ID of the watchlist
    /// * `epic` - The EPIC of the instrument to add
    ///
    /// # Returns
    /// * `Ok(StatusResponse)` - The operation status
    /// * `Err(AppError)` - If the request fails
    async fn add_to_watchlist(
        &self,
        watchlist_id: &str,
        epic: &str,
    ) -> Result<StatusResponse, AppError>;

    /// Removes an instrument from a watchlist
    ///
    /// # Arguments
    /// * `watchlist_id` - The ID of the watchlist
    /// * `epic` - The EPIC of the instrument to remove
    ///
    /// # Returns
    /// * `Ok(StatusResponse)` - The operation status
    /// * `Err(AppError)` - If the request fails
    async fn remove_from_watchlist(
        &self,
        watchlist_id: &str,
        epic: &str,
    ) -> Result<StatusResponse, AppError>;
}
