/******************************************************************************
    Author: Joaquín Béjar García
    Email: jb@taunais.com 
    Date: 15/9/25
 ******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use crate::model::mass_quote::MassQuoteItem;

/// Mass quote request
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct MassQuoteRequest {
    /// List of quote items
    pub items: Vec<MassQuoteItem>,
    /// User-defined label for the mass quote
    pub label: Option<String>,
}