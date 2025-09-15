/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use crate::model::response::other::AccountSummaryResponse;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Subaccount information
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Subaccount {
    /// Subaccount email
    pub email: String,
    /// Subaccount ID
    pub id: u64,
    /// Whether login is enabled
    pub login_enabled: bool,
    /// Portfolio information (optional)
    pub portfolio: Option<PortfolioInfo>,
    /// Whether to receive notifications
    pub receive_notifications: bool,
    /// System name
    pub system_name: String,
    /// Time in force (optional)
    pub tif: Option<String>,
    /// Subaccount type
    #[serde(rename = "type")]
    pub subaccount_type: String,
    /// Username
    pub username: String,
}

/// Portfolio information
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct PortfolioInfo {
    /// Available funds
    pub available_funds: f64,
    /// Available withdrawal funds
    pub available_withdrawal_funds: f64,
    /// Balance
    pub balance: f64,
    /// Currency
    pub currency: String,
    /// Delta total
    pub delta_total: f64,
    /// Equity
    pub equity: f64,
    /// Initial margin
    pub initial_margin: f64,
    /// Maintenance margin
    pub maintenance_margin: f64,
    /// Margin balance
    pub margin_balance: f64,
    /// Session realized P&L
    pub session_rpl: f64,
    /// Session unrealized P&L
    pub session_upl: f64,
    /// Total P&L
    pub total_pl: f64,
}

/// Portfolio information
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    /// Currency of the portfolio
    pub currency: String,
    /// Account summaries for different currencies
    pub accounts: Vec<AccountSummaryResponse>,
    /// Total portfolio value in USD
    pub total_usd_value: Option<f64>,
    /// Cross-currency margin enabled
    pub cross_margin_enabled: bool,
}

impl Portfolio {
    /// Create a new empty portfolio
    pub fn new(currency: String) -> Self {
        Self {
            currency,
            accounts: Vec::new(),
            total_usd_value: None,
            cross_margin_enabled: false,
        }
    }

    /// Add an account summary to the portfolio
    pub fn add_account(&mut self, account: AccountSummaryResponse) {
        self.accounts.push(account);
    }
}
