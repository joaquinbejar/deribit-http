/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 15/9/25
******************************************************************************/
use crate::model::withdrawal::WithdrawalPriority;
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Supported cryptocurrency currencies in the Deribit platform
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    /// Bitcoin cryptocurrency
    Btc,
    /// Ethereum cryptocurrency
    Eth,
    /// USD Coin stablecoin
    Usdc,
    /// Tether USD stablecoin
    Usdt,
    /// Euro-backed stablecoin
    Eurr,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::Btc => write!(f, "BTC"),
            Currency::Eth => write!(f, "ETH"),
            Currency::Usdc => write!(f, "USDC"),
            Currency::Usdt => write!(f, "USDT"),
            Currency::Eurr => write!(f, "EURR"),
        }
    }
}

/// Currency structure
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct CurrencyStruct {
    /// Currency symbol (BTC, ETH, etc.)
    pub currency: String,
    /// Long currency name
    pub currency_long: String,
    /// Number of decimal places for the currency
    pub decimals: Option<u32>,
    /// Fee precision (decimal places)
    pub fee_precision: Option<u32>,
    /// Minimum confirmations required
    pub min_confirmations: u32,
    /// Minimum withdrawal fee
    pub min_withdrawal_fee: f64,
    /// Standard withdrawal fee
    pub withdrawal_fee: f64,
    /// Withdrawal priorities
    pub withdrawal_priorities: Vec<WithdrawalPriority>,
    /// APR for yield-generating tokens
    pub apr: Option<f64>,
    /// Coin type identifier (e.g., "BTC", "ETH", "USDC")
    pub coin_type: Option<String>,
    /// Network fee
    pub network_fee: Option<f64>,
    /// Network currency used for fees
    pub network_currency: Option<String>,
    /// Whether the currency is part of the cross collateral pool
    pub in_cross_collateral_pool: Option<bool>,
}

/// Currency information and configuration
#[skip_serializing_none]
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrencyInfo {
    /// Coin type identifier (e.g., "BITCOIN", "ETHEREUM")
    pub coin_type: String,
    /// Currency code
    pub currency: String,
    /// Full currency name
    pub currency_long: String,
    /// Fee precision (decimal places)
    pub fee_precision: i32,
    /// Minimum confirmations required
    pub min_confirmations: i32,
    /// Minimum withdrawal fee
    pub min_withdrawal_fee: f64,
    /// Standard withdrawal fee
    pub withdrawal_fee: f64,
    /// Available withdrawal priorities
    pub withdrawal_priorities: Vec<WithdrawalPriority>,
    /// Whether the currency is disabled
    pub disabled: Option<bool>,
    /// Minimum deposit amount
    pub min_deposit_amount: Option<f64>,
    /// Maximum withdrawal amount
    pub max_withdrawal_amount: Option<f64>,
}

impl CurrencyInfo {
    /// Create new currency info
    pub fn new(
        coin_type: String,
        currency: String,
        currency_long: String,
        fee_precision: i32,
        min_confirmations: i32,
        min_withdrawal_fee: f64,
        withdrawal_fee: f64,
    ) -> Self {
        Self {
            coin_type,
            currency,
            currency_long,
            fee_precision,
            min_confirmations,
            min_withdrawal_fee,
            withdrawal_fee,
            withdrawal_priorities: Vec::new(),
            disabled: None,
            min_deposit_amount: None,
            max_withdrawal_amount: None,
        }
    }

    /// Add withdrawal priority
    pub fn add_priority(&mut self, priority: WithdrawalPriority) {
        self.withdrawal_priorities.push(priority);
    }

    /// Set disabled status
    pub fn with_disabled(mut self, disabled: bool) -> Self {
        self.disabled = Some(disabled);
        self
    }

    /// Set deposit limits
    pub fn with_deposit_limit(mut self, min_amount: f64) -> Self {
        self.min_deposit_amount = Some(min_amount);
        self
    }

    /// Set withdrawal limits
    pub fn with_withdrawal_limit(mut self, max_amount: f64) -> Self {
        self.max_withdrawal_amount = Some(max_amount);
        self
    }

    /// Check if currency is enabled
    pub fn is_enabled(&self) -> bool {
        !self.disabled.unwrap_or(false)
    }

    /// Get priority by name
    pub fn get_priority(&self, name: &str) -> Option<&WithdrawalPriority> {
        self.withdrawal_priorities.iter().find(|p| p.name == name)
    }

    /// Get highest priority
    pub fn highest_priority(&self) -> Option<&WithdrawalPriority> {
        self.withdrawal_priorities
            .iter()
            .max_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
    }

    /// Get lowest priority
    pub fn lowest_priority(&self) -> Option<&WithdrawalPriority> {
        self.withdrawal_priorities
            .iter()
            .min_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
    }
}

/// Collection of currency information
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct CurrencyInfoCollection {
    /// List of currency information
    pub currencies: Vec<CurrencyInfo>,
}

impl CurrencyInfoCollection {
    /// Create new collection
    pub fn new() -> Self {
        Self {
            currencies: Vec::new(),
        }
    }

    /// Add currency info
    pub fn add(&mut self, info: CurrencyInfo) {
        self.currencies.push(info);
    }

    /// Get currency info by currency
    pub fn get(&self, currency: String) -> Option<&CurrencyInfo> {
        self.currencies.iter().find(|c| c.currency == currency)
    }

    /// Get enabled currencies
    pub fn enabled(&self) -> Vec<&CurrencyInfo> {
        self.currencies.iter().filter(|c| c.is_enabled()).collect()
    }

    /// Get currencies with withdrawal support
    pub fn with_withdrawal(&self) -> Vec<&CurrencyInfo> {
        self.currencies
            .iter()
            .filter(|c| !c.withdrawal_priorities.is_empty())
            .collect()
    }
}

impl Default for CurrencyInfoCollection {
    fn default() -> Self {
        Self::new()
    }
}

/// Currency-specific expirations
#[derive(DebugPretty, DisplaySimple, Clone, Serialize, Deserialize)]
pub struct CurrencyExpirations {
    /// Future instrument expirations
    pub future: Option<Vec<String>>,
    /// Option instrument expirations
    pub option: Option<Vec<String>>,
}
