/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use crate::model::mass_quote::QuoteResult;

/// Mass quote response
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct MassQuoteResponse {
    /// List of quote results
    pub quotes: Vec<QuoteResult>,
}