/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Subaccount information
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct Subaccount {
    /// User email
    pub email: String,
    /// Account/Subaccount identifier
    pub id: u64,
    /// true when password for the subaccount has been configured
    pub is_password: Option<bool>,
    /// Informs whether login to the subaccount is enabled
    pub login_enabled: bool,
    /// Margin model
    pub margin_model: Option<String>,
    /// New email address that has not yet been confirmed (only included if with_portfolio == true)
    pub not_confirmed_email: Option<String>,
    /// Portfolio information (optional)
    pub portfolio: Option<Portfolio>,
    /// hashed identifier used in the Proof Of Liability for the subaccount
    pub proof_id: Option<String>,
    /// signature used as a base string for proof_id hash
    pub proof_id_signature: Option<String>,
    /// When true - receive all notification emails on the main email
    pub receive_notifications: bool,
    /// Names of assignments with Security Keys assigned
    pub security_keys_assignments: Option<Vec<String>>,
    /// Whether the Security Keys authentication is enabled
    pub security_keys_enabled: Option<bool>,
    /// System generated user nickname
    pub system_name: String,
    /// Account type
    #[serde(rename = "type")]
    pub subaccount_type: String,
    /// Username
    pub username: String,
}

/// Currency portfolio information
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct CurrencyPortfolio {
    /// The account's balance reserved in other orders
    pub additional_reserve: Option<f64>,
    /// Available funds
    pub available_funds: f64,
    /// Available withdrawal funds
    pub available_withdrawal_funds: f64,
    /// Account balance
    pub balance: f64,
    /// Currency symbol
    pub currency: String,
    /// Account equity
    pub equity: f64,
    /// Initial margin requirement
    pub initial_margin: f64,
    /// Maintenance margin requirement
    pub maintenance_margin: f64,
    /// Margin balance
    pub margin_balance: f64,
    /// The account's balance reserved in active spot orders
    pub spot_reserve: f64,
}

/// Trading product detail
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct TradingProductDetail {
    /// Whether enabled
    pub enabled: bool,
    /// Product name
    pub product: String,
    /// Whether overwriteable
    pub overwriteable: bool,
}

/// Portfolio information (legacy)
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    /// Bitcoin portfolio information
    pub btc: Option<CurrencyPortfolio>,
    /// Ethereum portfolio information
    pub eth: Option<CurrencyPortfolio>,
    /// USDC portfolio information (if applicable)
    pub usdc: Option<CurrencyPortfolio>,
    /// USDT portfolio information (if applicable)
    pub usdt: Option<CurrencyPortfolio>,
    /// EURR portfolio information (if applicable)
    pub eurr: Option<CurrencyPortfolio>,
}