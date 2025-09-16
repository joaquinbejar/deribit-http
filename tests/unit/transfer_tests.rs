use deribit_http::model::transfer::*;
use serde_json;

// Helper function to create a mock transfer
fn create_mock_transfer() -> Transfer {
    Transfer::new(
        12345,
        "BTC".to_string(),
        1.5,
        0.0005,
        "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
        1640995200000, // 2022-01-01 00:00:00 UTC
    )
}

// Helper function to create a mock subaccount transfer
fn create_mock_subaccount_transfer() -> SubaccountTransfer {
    SubaccountTransfer::new(
        67890,
        0.1,
        "ETH".to_string(),
        0,   // main account
        123, // subaccount
        1640995200000,
    )
}

#[cfg(test)]
mod transfer_state_tests {
    use super::*;

    #[test]
    fn test_transfer_state_default() {
        let state = TransferState::default();
        assert_eq!(state, TransferState::Prepared);
    }

    #[test]
    fn test_transfer_state_serialization() {
        let state = TransferState::Confirmed;
        let serialized = serde_json::to_string(&state).unwrap();
        assert_eq!(serialized, "\"confirmed\"");
    }

    #[test]
    fn test_transfer_state_deserialization() {
        let json = "\"cancelled\"";
        let state: TransferState = serde_json::from_str(json).unwrap();
        assert_eq!(state, TransferState::Cancelled);
    }

    #[test]
    fn test_all_transfer_states() {
        let states = vec![
            TransferState::Prepared,
            TransferState::Confirmed,
            TransferState::Cancelled,
            TransferState::WaitingForAdmin,
            TransferState::InsufficientFunds,
            TransferState::WithdrawalLimit,
        ];

        for state in states {
            let serialized = serde_json::to_string(&state).unwrap();
            let deserialized: TransferState = serde_json::from_str(&serialized).unwrap();
            assert_eq!(state, deserialized);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transfer_creation() {
        let transfer = create_mock_transfer();
        assert_eq!(transfer.id, 12345);
        assert_eq!(transfer.currency, "BTC");
        assert_eq!(transfer.amount, 1.5);
        assert_eq!(transfer.fee, 0.0005);
        assert_eq!(
            transfer.address,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh"
        );
        assert_eq!(transfer.state, TransferState::Prepared);
        assert_eq!(transfer.created_timestamp, 1640995200000);
        assert_eq!(transfer.updated_timestamp, 1640995200000);
        assert!(transfer.transaction_id.is_none());
        assert!(transfer.confirmed_timestamp.is_none());
        assert!(transfer.transfer_type.is_none());
    }

    #[test]
    fn test_transfer_with_transaction_id() {
        let transfer = create_mock_transfer().with_transaction_id("abc123def456".to_string());
        assert_eq!(transfer.transaction_id, Some("abc123def456".to_string()));
    }

    #[test]
    fn test_transfer_with_state() {
        let transfer = create_mock_transfer().with_state(TransferState::Confirmed);
        assert_eq!(transfer.state, TransferState::Confirmed);
    }

    #[test]
    fn test_transfer_with_type() {
        let transfer = create_mock_transfer().with_type("withdrawal".to_string());
        assert_eq!(transfer.transfer_type, Some("withdrawal".to_string()));
    }

    #[test]
    fn test_transfer_confirm() {
        let mut transfer = create_mock_transfer();
        let confirm_time = 1640995260000;
        transfer.confirm(confirm_time);

        assert_eq!(transfer.state, TransferState::Confirmed);
        assert_eq!(transfer.confirmed_timestamp, Some(confirm_time));
        assert_eq!(transfer.updated_timestamp, confirm_time);
    }

    #[test]
    fn test_transfer_cancel() {
        let mut transfer = create_mock_transfer();
        let cancel_time = 1640995260000;
        transfer.cancel(cancel_time);

        assert_eq!(transfer.state, TransferState::Cancelled);
        assert_eq!(transfer.updated_timestamp, cancel_time);
    }

    #[test]
    fn test_transfer_is_confirmed() {
        let mut transfer = create_mock_transfer();
        assert!(!transfer.is_confirmed());

        transfer.confirm(1640995260000);
        assert!(transfer.is_confirmed());
    }

    #[test]
    fn test_transfer_is_cancelled() {
        let mut transfer = create_mock_transfer();
        assert!(!transfer.is_cancelled());

        transfer.cancel(1640995260000);
        assert!(transfer.is_cancelled());
    }

    #[test]
    fn test_transfer_is_pending() {
        let transfer_prepared = create_mock_transfer();
        assert!(transfer_prepared.is_pending());

        let transfer_waiting = create_mock_transfer().with_state(TransferState::WaitingForAdmin);
        assert!(transfer_waiting.is_pending());

        let transfer_confirmed = create_mock_transfer().with_state(TransferState::Confirmed);
        assert!(!transfer_confirmed.is_pending());
    }

    #[test]
    fn test_transfer_net_amount() {
        let transfer = create_mock_transfer();
        assert_eq!(transfer.net_amount(), 1.4995); // 1.5 - 0.0005
    }

    #[test]
    fn test_transfer_serialization() {
        let transfer = create_mock_transfer()
            .with_transaction_id("tx123".to_string())
            .with_type("withdrawal".to_string());

        let serialized = serde_json::to_string(&transfer).unwrap();
        assert!(serialized.contains("\"id\":12345"));
        assert!(serialized.contains("\"currency\":\"BTC\""));
        assert!(serialized.contains("\"amount\":1.5"));
        assert!(serialized.contains("\"transaction_id\":\"tx123\""));
        assert!(serialized.contains("\"state\":\"prepared\""));
    }

    #[test]
    fn test_transfer_deserialization() {
        let json = r#"{
            "id": 12345,
            "currency": "BTC",
            "amount": 1.5,
            "fee": 0.0005,
            "address": "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh",
            "transaction_id": "tx123",
            "state": "confirmed",
            "created_timestamp": 1640995200000,
            "updated_timestamp": 1640995260000,
            "confirmed_timestamp": 1640995260000,
            "transfer_type": "withdrawal"
        }"#;

        let transfer: Transfer = serde_json::from_str(json).unwrap();
        assert_eq!(transfer.id, 12345);
        assert_eq!(transfer.currency, "BTC");
        assert_eq!(transfer.state, TransferState::Confirmed);
        assert_eq!(transfer.transaction_id, Some("tx123".to_string()));
    }

    #[test]
    fn test_transfer_clone() {
        let transfer = create_mock_transfer();
        let cloned = transfer.clone();
        assert_eq!(transfer, cloned);
    }

    #[test]
    fn test_transfer_round_trip_serialization() {
        let original = create_mock_transfer()
            .with_transaction_id("tx456".to_string())
            .with_state(TransferState::Confirmed);

        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: Transfer = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }
}

#[cfg(test)]
mod transfers_tests {
    use super::*;

    #[test]
    fn test_transfers_new() {
        let transfers = Transfers::new();
        assert!(transfers.transfers.is_empty());
    }

    #[test]
    fn test_transfers_default() {
        let transfers = Transfers::default();
        assert!(transfers.transfers.is_empty());
    }

    #[test]
    fn test_transfers_add() {
        let mut transfers = Transfers::new();
        let transfer = create_mock_transfer();
        transfers.add(transfer.clone());

        assert_eq!(transfers.transfers.len(), 1);
        assert_eq!(transfers.transfers[0], transfer);
    }

    #[test]
    fn test_transfers_by_currency() {
        let mut transfers = Transfers::new();
        let btc_transfer = create_mock_transfer();
        let eth_transfer = Transfer::new(
            67890,
            "ETH".to_string(),
            10.0,
            0.01,
            "0x742d35Cc6634C0532925a3b8D4C9db96".to_string(),
            1640995200000,
        );

        transfers.add(btc_transfer.clone());
        transfers.add(eth_transfer);

        let btc_transfers = transfers.by_currency("BTC".to_string());
        assert_eq!(btc_transfers.len(), 1);
        assert_eq!(btc_transfers[0].currency, "BTC");

        let eth_transfers = transfers.by_currency("ETH".to_string());
        assert_eq!(eth_transfers.len(), 1);
        assert_eq!(eth_transfers[0].currency, "ETH");
    }

    #[test]
    fn test_transfers_by_state() {
        let mut transfers = Transfers::new();
        let prepared_transfer = create_mock_transfer();
        let confirmed_transfer = create_mock_transfer().with_state(TransferState::Confirmed);

        transfers.add(prepared_transfer);
        transfers.add(confirmed_transfer);

        let prepared_transfers = transfers.by_state(TransferState::Prepared);
        assert_eq!(prepared_transfers.len(), 1);

        let confirmed_transfers = transfers.by_state(TransferState::Confirmed);
        assert_eq!(confirmed_transfers.len(), 1);
    }

    #[test]
    fn test_transfers_pending() {
        let mut transfers = Transfers::new();
        let prepared_transfer = create_mock_transfer();
        let waiting_transfer = create_mock_transfer().with_state(TransferState::WaitingForAdmin);
        let confirmed_transfer = create_mock_transfer().with_state(TransferState::Confirmed);

        transfers.add(prepared_transfer);
        transfers.add(waiting_transfer);
        transfers.add(confirmed_transfer);

        let pending_transfers = transfers.pending();
        assert_eq!(pending_transfers.len(), 2);
    }

    #[test]
    fn test_transfers_confirmed() {
        let mut transfers = Transfers::new();
        let prepared_transfer = create_mock_transfer();
        let confirmed_transfer = create_mock_transfer().with_state(TransferState::Confirmed);

        transfers.add(prepared_transfer);
        transfers.add(confirmed_transfer);

        let confirmed_transfers = transfers.confirmed();
        assert_eq!(confirmed_transfers.len(), 1);
    }

    #[test]
    fn test_transfers_total_amount() {
        let mut transfers = Transfers::new();
        let btc_transfer1 = create_mock_transfer(); // 1.5 BTC
        let btc_transfer2 = Transfer::new(
            67890,
            "BTC".to_string(),
            2.5,
            0.001,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            1640995200000,
        ); // 2.5 BTC
        let eth_transfer = Transfer::new(
            11111,
            "ETH".to_string(),
            10.0,
            0.01,
            "0x742d35Cc6634C0532925a3b8D4C9db96".to_string(),
            1640995200000,
        ); // 10.0 ETH

        transfers.add(btc_transfer1);
        transfers.add(btc_transfer2);
        transfers.add(eth_transfer);

        assert_eq!(transfers.total_amount("BTC".to_string()), 4.0);
        assert_eq!(transfers.total_amount("ETH".to_string()), 10.0);
        assert_eq!(transfers.total_amount("USDT".to_string()), 0.0);
    }

    #[test]
    fn test_transfers_total_fees() {
        let mut transfers = Transfers::new();
        let btc_transfer1 = create_mock_transfer(); // 0.0005 BTC fee
        let btc_transfer2 = Transfer::new(
            67890,
            "BTC".to_string(),
            2.5,
            0.001,
            "bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh".to_string(),
            1640995200000,
        ); // 0.001 BTC fee

        transfers.add(btc_transfer1);
        transfers.add(btc_transfer2);

        assert_eq!(transfers.total_fees("BTC".to_string()), 0.0015);
        assert_eq!(transfers.total_fees("ETH".to_string()), 0.0);
    }

    #[test]
    fn test_transfers_serialization() {
        let mut transfers = Transfers::new();
        transfers.add(create_mock_transfer());

        let serialized = serde_json::to_string(&transfers).unwrap();
        assert!(serialized.contains("\"transfers\":"));
        assert!(serialized.contains("\"id\":12345"));
    }

    #[test]
    fn test_transfers_clone() {
        let mut transfers = Transfers::new();
        transfers.add(create_mock_transfer());

        let cloned = transfers.clone();
        assert_eq!(transfers, cloned);
    }
}

#[cfg(test)]
mod subaccount_transfer_tests {
    use super::*;

    #[test]
    fn test_subaccount_transfer_creation() {
        let transfer = create_mock_subaccount_transfer();
        assert_eq!(transfer.id, 67890);
        assert_eq!(transfer.amount, 0.1);
        assert_eq!(transfer.currency, "ETH");
        assert_eq!(transfer.source, 0);
        assert_eq!(transfer.destination, 123);
        assert_eq!(transfer.state, TransferState::Prepared);
        assert_eq!(transfer.timestamp, 1640995200000);
        assert_eq!(transfer.transfer_type, "subaccount");
    }

    #[test]
    fn test_subaccount_transfer_with_state() {
        let transfer = create_mock_subaccount_transfer().with_state(TransferState::Confirmed);
        assert_eq!(transfer.state, TransferState::Confirmed);
    }

    #[test]
    fn test_subaccount_transfer_with_type() {
        let transfer = create_mock_subaccount_transfer().with_type("internal".to_string());
        assert_eq!(transfer.transfer_type, "internal");
    }

    #[test]
    fn test_subaccount_transfer_is_main_subaccount_transfer() {
        let main_to_sub = create_mock_subaccount_transfer(); // source = 0
        assert!(main_to_sub.is_main_subaccount_transfer());

        let sub_to_main = SubaccountTransfer::new(
            12345,
            1.0,
            "BTC".to_string(),
            123, // subaccount
            0,   // main account
            1640995200000,
        );
        assert!(sub_to_main.is_main_subaccount_transfer());

        let sub_to_sub = SubaccountTransfer::new(
            12345,
            1.0,
            "BTC".to_string(),
            123, // subaccount
            456, // another subaccount
            1640995200000,
        );
        assert!(!sub_to_sub.is_main_subaccount_transfer());
    }

    #[test]
    fn test_subaccount_transfer_is_subaccount_to_subaccount() {
        let main_to_sub = create_mock_subaccount_transfer(); // source = 0
        assert!(!main_to_sub.is_subaccount_to_subaccount());

        let sub_to_sub = SubaccountTransfer::new(
            12345,
            1.0,
            "BTC".to_string(),
            123, // subaccount
            456, // another subaccount
            1640995200000,
        );
        assert!(sub_to_sub.is_subaccount_to_subaccount());
    }

    #[test]
    fn test_subaccount_transfer_serialization() {
        let transfer = create_mock_subaccount_transfer();
        let serialized = serde_json::to_string(&transfer).unwrap();

        assert!(serialized.contains("\"id\":67890"));
        assert!(serialized.contains("\"amount\":0.1"));
        assert!(serialized.contains("\"currency\":\"ETH\""));
        assert!(serialized.contains("\"source\":0"));
        assert!(serialized.contains("\"destination\":123"));
        assert!(serialized.contains("\"state\":\"prepared\""));
    }

    #[test]
    fn test_subaccount_transfer_deserialization() {
        let json = r#"{
            "amount": 0.1,
            "currency": "ETH",
            "destination": 123,
            "id": 67890,
            "source": 0,
            "state": "confirmed",
            "timestamp": 1640995200000,
            "transfer_type": "subaccount"
        }"#;

        let transfer: SubaccountTransfer = serde_json::from_str(json).unwrap();
        assert_eq!(transfer.id, 67890);
        assert_eq!(transfer.amount, 0.1);
        assert_eq!(transfer.currency, "ETH");
        assert_eq!(transfer.state, TransferState::Confirmed);
    }

    #[test]
    fn test_subaccount_transfer_clone() {
        let transfer = create_mock_subaccount_transfer();
        let cloned = transfer.clone();
        assert_eq!(transfer, cloned);
    }

    #[test]
    fn test_subaccount_transfer_round_trip_serialization() {
        let original = create_mock_subaccount_transfer()
            .with_state(TransferState::Confirmed)
            .with_type("internal".to_string());

        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: SubaccountTransfer = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_subaccount_transfer_edge_cases() {
        // Test with zero amounts
        let zero_transfer = SubaccountTransfer::new(1, 0.0, "BTC".to_string(), 0, 1, 1640995200000);
        assert_eq!(zero_transfer.amount, 0.0);

        // Test with very large amounts
        let large_transfer =
            SubaccountTransfer::new(2, 1000000.0, "USDT".to_string(), 1, 2, 1640995200000);
        assert_eq!(large_transfer.amount, 1000000.0);
    }
}
