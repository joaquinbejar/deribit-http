//! Portfolio simulation models for Deribit API
//!
//! This module contains types for portfolio margin simulation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request for simulate_portfolio endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulatePortfolioRequest {
    /// Currency for the simulation (e.g., "BTC", "ETH")
    pub currency: String,
    /// Whether to add simulated positions to existing positions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_positions: Option<bool>,
    /// Map of instrument names to simulated position sizes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulated_positions: Option<HashMap<String, f64>>,
}

impl SimulatePortfolioRequest {
    /// Creates a new simulation request for the specified currency
    #[must_use]
    pub fn new(currency: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            add_positions: None,
            simulated_positions: None,
        }
    }

    /// Sets whether to add positions to existing ones
    #[must_use]
    pub fn with_add_positions(mut self, add: bool) -> Self {
        self.add_positions = Some(add);
        self
    }

    /// Sets the simulated positions
    #[must_use]
    pub fn with_simulated_positions(mut self, positions: HashMap<String, f64>) -> Self {
        self.simulated_positions = Some(positions);
        self
    }
}

/// Response for simulate_portfolio endpoint
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulatePortfolioResponse {
    /// Projected initial margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_initial_margin: Option<f64>,
    /// Projected maintenance margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_maintenance_margin: Option<f64>,
    /// Projected delta total
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_delta_total: Option<f64>,
    /// Change in margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_change: Option<f64>,
    /// Available funds after simulation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_funds: Option<f64>,
    /// Additional margin data
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// Response for PME (Portfolio Margin Engine) simulation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PmeSimulateResponse {
    /// Total projected margin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_margin: Option<f64>,
    /// Liquidation price estimate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liquidation_price: Option<f64>,
    /// Risk metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_value: Option<f64>,
    /// Additional PME data
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_portfolio_request_builder() {
        let mut positions = HashMap::new();
        positions.insert("BTC-PERPETUAL".to_string(), 1.0);

        let request = SimulatePortfolioRequest::new("BTC")
            .with_add_positions(true)
            .with_simulated_positions(positions);

        assert_eq!(request.currency, "BTC");
        assert_eq!(request.add_positions, Some(true));
        assert!(request.simulated_positions.is_some());
    }

    #[test]
    fn test_simulate_portfolio_response_deserialization() {
        let json = r#"{
            "projected_initial_margin": 0.05,
            "projected_maintenance_margin": 0.03,
            "projected_delta_total": 1.5,
            "margin_change": 0.01,
            "available_funds": 0.95
        }"#;

        let response: SimulatePortfolioResponse =
            serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(response.projected_initial_margin, Some(0.05));
        assert_eq!(response.projected_maintenance_margin, Some(0.03));
    }

    #[test]
    fn test_pme_simulate_response_deserialization() {
        let json = r#"{
            "projected_margin": 0.1,
            "liquidation_price": 50000.0,
            "risk_value": 0.05
        }"#;

        let response: PmeSimulateResponse = serde_json::from_str(json).expect("Failed to parse");
        assert_eq!(response.projected_margin, Some(0.1));
        assert_eq!(response.liquidation_price, Some(50000.0));
    }
}
