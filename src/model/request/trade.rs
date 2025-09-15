/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use serde::{Deserialize, Serialize};
use crate::model::{Currency, InstrumentKind, SortDirection};

/// Parameters for requesting user trades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradesRequest {
    /// The currency symbol (required)
    pub currency: Currency,
    /// Instrument kind filter (optional)
    pub kind: Option<InstrumentKind>,
    /// The ID of the first trade to be returned (optional)
    pub start_id: Option<String>,
    /// The ID of the last trade to be returned (optional)
    pub end_id: Option<String>,
    /// Number of requested items, default - 10, maximum - 1000 (optional)
    pub count: Option<u32>,
    /// The earliest timestamp to return result from (milliseconds since UNIX epoch) (optional)
    pub start_timestamp: Option<u64>,
    /// The most recent timestamp to return result from (milliseconds since UNIX epoch) (optional)
    pub end_timestamp: Option<u64>,
    /// Direction of results sorting (optional)
    pub sorting: Option<SortDirection>,
    /// Determines whether historical trade records should be retrieved (optional)
    pub historical: Option<bool>,
    /// The user id for the subaccount (optional)
    pub subaccount_id: Option<u32>,
}