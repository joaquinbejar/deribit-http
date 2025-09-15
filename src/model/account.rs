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

    /// Get account summary for a specific currency
    pub fn get_account(&self, currency: &String) -> Option<&AccountSummaryResponse> {
        self.accounts.iter().find(|acc| &acc.currency == currency)
    }

    /// Calculate total equity across all accounts
    pub fn total_equity(&self) -> f64 {
        self.accounts.iter().map(|acc| acc.equity).sum()
    }

    /// Calculate total unrealized PnL across all accounts
    pub fn total_unrealized_pnl(&self) -> f64 {
        self.accounts
            .iter()
            .map(|account| account.unrealized_pnl.unwrap_or(0.0))
            .sum()
    }

    /// Calculate total realized PnL across all accounts
    pub fn total_realized_pnl(&self) -> f64 {
        self.accounts
            .iter()
            .map(|acc| acc.realized_pnl.unwrap_or(0.0))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_test_account_summary() -> AccountSummaryResponse {
        AccountSummaryResponse {
            currency: "BTC".to_string(),
            balance: 1.5,
            equity: 1.4,
            available_funds: 1.2,
            margin_balance: 0.3,
            unrealized_pnl: Some(-0.1),
            realized_pnl: Some(0.05),
            total_pl: Some(-0.05),
            session_funding: Some(0.0),
            session_rpl: Some(0.0),
            session_upl: Some(0.0),
            maintenance_margin: 0.1,
            initial_margin: 0.2,
            available_withdrawal_funds: Some(1.0),
            cross_collateral_enabled: Some(true),
            delta_total: Some(0.5),
            futures_pl: Some(0.03),
            futures_session_rpl: Some(0.01),
            futures_session_upl: Some(-0.02),
            options_delta: Some(0.3),
            options_gamma: Some(0.05),
            options_pl: Some(-0.08),
            options_session_rpl: Some(0.01),
            options_session_upl: Some(-0.06),
            options_theta: Some(-0.02),
            options_vega: Some(0.1),
            portfolio_margining_enabled: Some(false),
            projected_delta_total: Some(0.6),
            projected_initial_margin: Some(0.25),
            projected_maintenance_margin: Some(0.12),
            system_name: Some("deribit".to_string()),
            account_type: Some("main".to_string()),
            delta_total_map: HashMap::new(),
            deposit_address: Some("bc1qtest123".to_string()),
            fees: Some(vec![HashMap::new()]),
            limits: serde_json::json!({}),
            locked_balance: Some(0.0),
            margin_model: Some("segregated_sm".to_string()),
            options_gamma_map: Some(HashMap::new()),
            options_theta_map: Some(HashMap::new()),
            options_vega_map: Some(HashMap::new()),
            options_value: Some(0.0),
            spot_reserve: Some(0.0),
            testnet: Some(true),
            us_diff: Some(0),
            us_in: Some(0),
            us_out: Some(0),
            estimated_liquidation_ratio: Some(0.0),
            estimated_liquidation_ratio_map: Some(HashMap::new()),
            fee_balance: Some(0.0),
        }
    }

    #[test]
    fn test_account_summary_margin_utilization() {
        let account = create_test_account_summary();
        let utilization = account.margin_utilization();
        assert!((utilization - 14.285714285714286).abs() < 0.0001); // 0.2 / 1.4 * 100
    }

    #[test]
    fn test_account_summary_margin_utilization_zero_equity() {
        let mut account = create_test_account_summary();
        account.equity = 0.0;
        assert_eq!(account.margin_utilization(), 0.0);
    }

    #[test]
    fn test_account_summary_available_margin() {
        let account = create_test_account_summary();
        assert_eq!(account.available_margin(), 1.2); // 1.4 - 0.2
    }

    #[test]
    fn test_account_summary_is_at_risk() {
        let account = create_test_account_summary();
        assert!(!account.is_at_risk(20.0)); // 14.28% < 20%
        assert!(account.is_at_risk(10.0)); // 14.28% > 10%
    }

    #[test]
    fn test_account_summary_return_on_equity() {
        let account = create_test_account_summary();
        let roe = account.return_on_equity();
        assert!((roe - (-3.571428571428571)).abs() < 0.0001); // -0.05 / 1.4 * 100
    }

    #[test]
    fn test_account_summary_return_on_equity_zero_equity() {
        let mut account = create_test_account_summary();
        account.equity = 0.0;
        assert_eq!(account.return_on_equity(), 0.0);
    }

    #[test]
    fn test_portfolio_new() {
        let portfolio = Portfolio::new("USD".to_string());
        assert_eq!(portfolio.currency, "USD");
        assert!(portfolio.accounts.is_empty());
        assert_eq!(portfolio.total_usd_value, None);
        assert!(!portfolio.cross_margin_enabled);
    }

    #[test]
    fn test_portfolio_add_account() {
        let mut portfolio = Portfolio::new("USD".to_string());
        let account = create_test_account_summary();
        portfolio.add_account(account);
        assert_eq!(portfolio.accounts.len(), 1);
    }

    #[test]
    fn test_portfolio_get_account() {
        let mut portfolio = Portfolio::new("USD".to_string());
        let account = create_test_account_summary();
        portfolio.add_account(account);

        let found = portfolio.get_account(&"BTC".to_string());
        assert!(found.is_some());
        assert_eq!(found.unwrap().currency, "BTC");

        let not_found = portfolio.get_account(&"ETH".to_string());
        assert!(not_found.is_none());
    }

    #[test]
    fn test_portfolio_total_equity() {
        let mut portfolio = Portfolio::new("USD".to_string());
        let mut account1 = create_test_account_summary();
        account1.equity = 1.0;
        let mut account2 = create_test_account_summary();
        account2.equity = 2.0;

        portfolio.add_account(account1);
        portfolio.add_account(account2);

        assert_eq!(portfolio.total_equity(), 3.0);
    }

    #[test]
    fn test_portfolio_total_realized_pnl() {
        let mut portfolio = Portfolio::new("USD".to_string());
        let mut account1 = create_test_account_summary();
        account1.realized_pnl = Some(0.05);
        let mut account2 = create_test_account_summary();
        account2.realized_pnl = Some(0.03);

        portfolio.add_account(account1);
        portfolio.add_account(account2);

        assert_eq!(portfolio.total_realized_pnl(), 0.08);
    }

    #[test]
    fn test_account_summary_serialization() {
        let account = create_test_account_summary();
        let json = serde_json::to_string(&account).unwrap();
        let deserialized: AccountSummaryResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(account.currency, deserialized.currency);
        assert_eq!(account.balance, deserialized.balance);
    }

    #[test]
    fn test_portfolio_serialization() {
        let portfolio = Portfolio::new("USD".to_string());
        let json = serde_json::to_string(&portfolio).unwrap();
        let deserialized: Portfolio = serde_json::from_str(&json).unwrap();
        assert_eq!(portfolio.currency, deserialized.currency);
    }

    #[test]
    fn test_subaccount_creation() {
        let subaccount = Subaccount {
            email: "test@example.com".to_string(),
            id: 12345,
            login_enabled: true,
            portfolio: None,
            receive_notifications: false,
            system_name: "deribit".to_string(),
            tif: Some("GTC".to_string()),
            subaccount_type: "subaccount".to_string(),
            username: "testuser".to_string(),
        };

        assert_eq!(subaccount.email, "test@example.com");
        assert_eq!(subaccount.id, 12345);
        assert!(subaccount.login_enabled);
    }

    #[test]
    fn test_portfolio_info_creation() {
        let portfolio_info = PortfolioInfo {
            available_funds: 1000.0,
            available_withdrawal_funds: 900.0,
            balance: 1100.0,
            currency: "BTC".to_string(),
            delta_total: 0.5,
            equity: 1050.0,
            initial_margin: 100.0,
            maintenance_margin: 50.0,
            margin_balance: 150.0,
            session_rpl: 10.0,
            session_upl: -5.0,
            total_pl: 5.0,
        };

        assert_eq!(portfolio_info.currency, "BTC");
        assert_eq!(portfolio_info.balance, 1100.0);
    }

    #[test]
    fn test_debug_and_display_implementations() {
        let account = create_test_account_summary();
        let debug_str = format!("{:?}", account);
        let display_str = format!("{}", account);

        assert!(debug_str.contains("BTC"));
        assert!(display_str.contains("BTC"));
    }
}
