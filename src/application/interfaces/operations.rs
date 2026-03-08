/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 8/3/26
******************************************************************************/

//! Operations service interface for IG Markets API
//!
//! This module defines the interface for managing API application operations,
//! including retrieving application details and disabling API keys.

use crate::error::AppError;
use crate::model::responses::{ApplicationDetailsResponse, StatusResponse};
use async_trait::async_trait;

/// Service for managing API application operations in the IG Markets API
///
/// This service provides methods for retrieving API application details
/// and managing API key status.
#[async_trait]
pub trait OperationsService: Send + Sync {
    /// Returns a list of client-owned applications
    ///
    /// # Returns
    /// * `Ok(ApplicationDetailsResponse)` - Details of all client applications
    /// * `Err(AppError)` - If the request fails
    async fn get_client_apps(&self) -> Result<ApplicationDetailsResponse, AppError>;

    /// Disables the current application key
    ///
    /// Disables the current application key from processing further requests.
    /// Disabled keys may be re-enabled via the My Account section on
    /// the IG Web Dealing Platform.
    ///
    /// # Returns
    /// * `Ok(StatusResponse)` - The disable operation status
    /// * `Err(AppError)` - If the request fails
    async fn disable_client_app(&self) -> Result<StatusResponse, AppError>;
}
