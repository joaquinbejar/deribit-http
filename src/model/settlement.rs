/******************************************************************************
   Author: Joaquín Béjar García
   Email: jb@taunais.com
   Date: 21/7/25
******************************************************************************/
use pretty_simple_display::{DebugPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

/// Settlement event types
#[derive(DebugPretty, DisplaySimple, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SettlementType {
    /// Regular settlement event
    Settlement,
    /// Delivery event for futures/options
    Delivery,
    /// Bankruptcy event
    Bankruptcy,
}

impl Default for SettlementType {
    fn default() -> Self {
        Self::Settlement
    }
}

/// Settlement event information
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settlement {
    /// Type of settlement event
    #[serde(alias = "type")]
    pub settlement_type: SettlementType,
    /// Timestamp of the settlement event (milliseconds since Unix epoch)
    pub timestamp: i64,
    /// Instrument name (settlement and delivery only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    /// Position size in quote currency (settlement and delivery only)
    #[serde(alias = "position", skip_serializing_if = "Option::is_none")]
    pub position_size: Option<f64>,
    /// Mark price at settlement time in quote currency (settlement and delivery only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<f64>,
    /// Underlying index price at time of event in quote currency (settlement and delivery only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_price: Option<f64>,
    /// Profit and loss in base currency (settlement and delivery only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_loss: Option<f64>,
    /// Funding in base currency (settlement for perpetual product only)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<f64>,
    // Additional fields from deribit-http types.rs
    /// Session profit and loss (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_profit_loss: Option<f64>,
    /// Session bankruptcy (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_bankrupt_cy: Option<f64>,
    /// Session tax (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_tax: Option<f64>,
    /// Session tax rate (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_tax_rate: Option<f64>,
    /// Socialized losses (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub socialized_losses: Option<f64>,
    /// Additional fields that might be present in the API response
    #[serde(flatten)]
    pub additional_fields: std::collections::HashMap<String, serde_json::Value>,
}

impl Settlement {
    /// Create a new settlement event
    pub fn new(settlement_type: SettlementType, timestamp: i64) -> Self {
        Self {
            settlement_type,
            timestamp,
            instrument_name: None,
            position_size: None,
            mark_price: None,
            index_price: None,
            profit_loss: None,
            funding: None,
            session_profit_loss: None,
            session_bankrupt_cy: None,
            session_tax: None,
            session_tax_rate: None,
            socialized_losses: None,
            additional_fields: std::collections::HashMap::new(),
        }
    }

    /// Create a settlement event for an instrument
    pub fn for_instrument(
        settlement_type: SettlementType,
        timestamp: i64,
        instrument_name: String,
    ) -> Self {
        Self {
            settlement_type,
            timestamp,
            instrument_name: Some(instrument_name),
            position_size: None,
            mark_price: None,
            index_price: None,
            profit_loss: None,
            funding: None,
            session_profit_loss: None,
            session_bankrupt_cy: None,
            session_tax: None,
            session_tax_rate: None,
            socialized_losses: None,
            additional_fields: std::collections::HashMap::new(),
        }
    }

    /// Set position details
    pub fn with_position(mut self, size: f64, mark_price: f64, index_price: f64) -> Self {
        self.position_size = Some(size);
        self.mark_price = Some(mark_price);
        self.index_price = Some(index_price);
        self
    }

    /// Set profit/loss
    pub fn with_pnl(mut self, pnl: f64) -> Self {
        self.profit_loss = Some(pnl);
        self
    }

    /// Set funding (for perpetuals)
    pub fn with_funding(mut self, funding: f64) -> Self {
        self.funding = Some(funding);
        self
    }

    /// Check if this is a settlement event
    pub fn is_settlement(&self) -> bool {
        matches!(self.settlement_type, SettlementType::Settlement)
    }

    /// Check if this is a delivery event
    pub fn is_delivery(&self) -> bool {
        matches!(self.settlement_type, SettlementType::Delivery)
    }

    /// Check if this is a bankruptcy event
    pub fn is_bankruptcy(&self) -> bool {
        matches!(self.settlement_type, SettlementType::Bankruptcy)
    }
}

impl Default for Settlement {
    fn default() -> Self {
        Self::new(SettlementType::default(), 0)
    }
}

/// Collection of settlements
#[derive(DebugPretty, DisplaySimple, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settlements {
    /// List of settlement events
    pub settlements: Vec<Settlement>,
}

impl Settlements {
    /// Create a new settlements collection
    pub fn new() -> Self {
        Self {
            settlements: Vec::new(),
        }
    }

    /// Add a settlement to the collection
    pub fn add(&mut self, settlement: Settlement) {
        self.settlements.push(settlement);
    }

    /// Get settlements by type
    pub fn by_type(&self, settlement_type: SettlementType) -> Vec<&Settlement> {
        self.settlements
            .iter()
            .filter(|s| s.settlement_type == settlement_type)
            .collect()
    }

    /// Get settlements for a specific instrument
    pub fn by_instrument(&self, instrument_name: &str) -> Vec<&Settlement> {
        self.settlements
            .iter()
            .filter(|s| {
                s.instrument_name
                    .as_ref()
                    .is_some_and(|name| name == instrument_name)
            })
            .collect()
    }
}

impl Default for Settlements {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settlement_creation() {
        let settlement = Settlement::new(SettlementType::Settlement, 1640995200000);
        assert_eq!(settlement.settlement_type, SettlementType::Settlement);
        assert_eq!(settlement.timestamp, 1640995200000);
        assert!(settlement.instrument_name.is_none());
    }

    #[test]
    fn test_settlement_builder() {
        let settlement = Settlement::for_instrument(
            SettlementType::Delivery,
            1640995200000,
            "BTC-25MAR23".to_string(),
        )
        .with_position(1.5, 45000.0, 44950.0)
        .with_pnl(75.0);

        assert_eq!(settlement.settlement_type, SettlementType::Delivery);
        assert_eq!(settlement.instrument_name, Some("BTC-25MAR23".to_string()));
        assert_eq!(settlement.position_size, Some(1.5));
        assert_eq!(settlement.profit_loss, Some(75.0));
    }

    #[test]
    fn test_settlement_type_checks() {
        let settlement = Settlement::new(SettlementType::Settlement, 0);
        assert!(settlement.is_settlement());
        assert!(!settlement.is_delivery());
        assert!(!settlement.is_bankruptcy());
    }

    #[test]
    fn test_settlements_collection() {
        let mut settlements = Settlements::new();
        settlements.add(Settlement::new(SettlementType::Settlement, 1000));
        settlements.add(Settlement::new(SettlementType::Delivery, 2000));

        assert_eq!(settlements.settlements.len(), 2);
        assert_eq!(settlements.by_type(SettlementType::Settlement).len(), 1);
        assert_eq!(settlements.by_type(SettlementType::Delivery).len(), 1);
    }

    #[test]
    fn test_serde() {
        let settlement = Settlement::for_instrument(
            SettlementType::Settlement,
            1640995200000,
            "BTC-PERPETUAL".to_string(),
        )
        .with_funding(0.0001);

        let json = serde_json::to_string(&settlement).unwrap();
        let deserialized: Settlement = serde_json::from_str(&json).unwrap();
        assert_eq!(settlement, deserialized);
    }
}
