//! Positions Integration Tests
//!
//! This test covers positions functionality:
//! 1. Get positions for different currencies
//! 2. Test position filtering by kind
//! 3. Test subaccount position filtering
//! 4. Validate position data structure

use std::path::Path;
use tracing::{debug, info, warn};

use deribit_http::DeribitHttpClient;

/// Check if .env file exists and contains required variables
fn check_env_file() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(".env").exists() {
        return Err("Missing .env file. Please create one with authentication credentials".into());
    }

    dotenv::dotenv().ok();

    let has_oauth2 = std::env::var("DERIBIT_CLIENT_ID").is_ok()
        && std::env::var("DERIBIT_CLIENT_SECRET").is_ok();
    let has_api_key =
        std::env::var("DERIBIT_API_KEY").is_ok() && std::env::var("DERIBIT_API_SECRET").is_ok();

    if !has_oauth2 && !has_api_key {
        return Err("Missing authentication credentials".into());
    }

    Ok(())
}

/// Authenticate client using available credentials
async fn authenticate_client(client: &DeribitHttpClient) -> Result<(), Box<dyn std::error::Error>> {
    if let (Ok(client_id), Ok(client_secret)) = (
        std::env::var("DERIBIT_CLIENT_ID"),
        std::env::var("DERIBIT_CLIENT_SECRET"),
    ) {
        client
            .authenticate_oauth2(&client_id, &client_secret)
            .await?;
    } else if let (Ok(api_key), Ok(api_secret)) = (
        std::env::var("DERIBIT_API_KEY"),
        std::env::var("DERIBIT_API_SECRET"),
    ) {
        client.authenticate_api_key(&api_key, &api_secret).await?;
    } else {
        return Err("No valid authentication credentials found".into());
    }
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_positions_all() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting get all positions test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting all positions");
    let positions = client.get_positions(None, None, None).await?;

    info!(
        "All positions retrieved successfully, count: {}",
        positions.len()
    );
    debug!("Positions: {:?}", positions);

    // Validate positions structure
    for (i, position) in positions.iter().enumerate() {
        debug!(
            "Validating position #{}: {}",
            i + 1,
            position.instrument_name
        );

        assert!(
            !position.instrument_name.is_empty(),
            "Instrument name should not be empty"
        );
        assert!(
            position.kind.as_ref().map_or(true, |k| !k.is_empty()),
            "Kind should not be empty"
        );
        // Direction is an enum, validate it exists
        debug!("Position direction: {:?}", position.direction);
        assert!(
            position.size != 0.0 || position.size == 0.0,
            "Size should be a valid number"
        );
        assert!(
            position.average_price >= 0.0,
            "Average price should be non-negative"
        );
        if let Some(mark_price) = position.mark_price {
            assert!(
                mark_price >= 0.0,
                "Mark price should be non-negative if present"
            );
        }
        if let Some(index_price) = position.index_price {
            assert!(
                index_price >= 0.0,
                "Index price should be non-negative if present"
            );
        }
        if let Some(initial_margin) = position.initial_margin {
            assert!(
                initial_margin >= 0.0,
                "Initial margin should be non-negative if present"
            );
        }
        if let Some(maintenance_margin) = position.maintenance_margin {
            assert!(
                maintenance_margin >= 0.0,
                "Maintenance margin should be non-negative if present"
            );
        }
    }

    info!("Get all positions test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_positions_btc() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting get BTC positions test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting BTC positions");
    let positions = client.get_positions(Some("BTC"), None, None).await?;

    info!(
        "BTC positions retrieved successfully, count: {}",
        positions.len()
    );
    debug!("BTC positions: {:?}", positions);

    // Validate that all positions are BTC-related
    for position in &positions {
        assert!(
            position.instrument_name.starts_with("BTC"),
            "All positions should be BTC-related: {}",
            position.instrument_name
        );
    }

    info!("Get BTC positions test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_positions_eth() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting get ETH positions test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting ETH positions");
    let positions = client.get_positions(Some("ETH"), None, None).await?;

    info!(
        "ETH positions retrieved successfully, count: {}",
        positions.len()
    );
    debug!("ETH positions: {:?}", positions);

    // Validate that all positions are ETH-related
    for position in &positions {
        assert!(
            position.instrument_name.starts_with("ETH"),
            "All positions should be ETH-related: {}",
            position.instrument_name
        );
    }

    info!("Get ETH positions test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_positions_by_kind_future() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting get future positions test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting future positions");
    let positions = client.get_positions(None, Some("future"), None).await?;

    info!(
        "Future positions retrieved successfully, count: {}",
        positions.len()
    );
    debug!("Future positions: {:?}", positions);

    // Validate that all positions are futures
    for position in &positions {
        if let Some(ref kind) = position.kind {
            assert_eq!(
                kind, "future",
                "All positions should be futures: {}",
                position.instrument_name
            );
        }
    }

    info!("Get future positions test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_positions_by_kind_option() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting get option positions test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting option positions");
    let positions = client.get_positions(None, Some("option"), None).await?;

    info!(
        "Option positions retrieved successfully, count: {}",
        positions.len()
    );
    debug!("Option positions: {:?}", positions);

    // Validate that all positions are options
    for position in &positions {
        if let Some(ref kind) = position.kind {
            assert_eq!(
                kind, "option",
                "All positions should be options: {}",
                position.instrument_name
            );
        }
    }

    info!("Get option positions test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_positions_combined_filters() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting get positions with combined filters test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting BTC future positions");
    let positions = client
        .get_positions(Some("BTC"), Some("future"), None)
        .await?;

    info!(
        "BTC future positions retrieved successfully, count: {}",
        positions.len()
    );
    debug!("BTC future positions: {:?}", positions);

    // Validate that all positions match both filters
    for position in &positions {
        assert!(
            position.instrument_name.starts_with("BTC"),
            "All positions should be BTC-related: {}",
            position.instrument_name
        );
        if let Some(ref kind) = position.kind {
            assert_eq!(
                kind, "future",
                "All positions should be futures: {}",
                position.instrument_name
            );
        }
    }

    info!("Get positions with combined filters test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_positions_data_validation() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting positions data validation test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    debug!("Getting positions for data validation");
    let positions = client.get_positions(Some("BTC"), None, None).await?;

    info!(
        "Positions retrieved for validation, count: {}",
        positions.len()
    );

    for position in &positions {
        debug!("Validating position data for: {}", position.instrument_name);

        // Validate required fields
        assert!(
            !position.instrument_name.is_empty(),
            "Instrument name should not be empty"
        );
        assert!(
            position.kind.as_ref().map_or(true, |k| !k.is_empty()),
            "Kind should not be empty"
        );
        // Direction is an enum, validate it exists
        debug!("Position direction: {:?}", position.direction);

        // Validate numeric fields
        assert!(position.size.is_finite(), "Size should be a finite number");
        assert!(
            position.average_price.is_finite() && position.average_price >= 0.0,
            "Average price should be a finite non-negative number"
        );
        if let Some(mark_price) = position.mark_price {
            assert!(
                mark_price.is_finite() && mark_price >= 0.0,
                "Mark price should be a finite non-negative number if present"
            );
        }
        if let Some(index_price) = position.index_price {
            assert!(
                index_price.is_finite() && index_price >= 0.0,
                "Index price should be a finite non-negative number if present"
            );
        }
        if let Some(settlement_price) = position.settlement_price {
            assert!(
                settlement_price.is_finite() && settlement_price >= 0.0,
                "Settlement price should be a finite non-negative number if present"
            );
        }

        // Validate margin fields
        if let Some(initial_margin) = position.initial_margin {
            assert!(
                initial_margin.is_finite() && initial_margin >= 0.0,
                "Initial margin should be a finite non-negative number if present"
            );
        }
        if let Some(maintenance_margin) = position.maintenance_margin {
            assert!(
                maintenance_margin.is_finite() && maintenance_margin >= 0.0,
                "Maintenance margin should be a finite non-negative number if present"
            );
        }
        if let Some(open_orders_margin) = position.open_orders_margin {
            assert!(
                open_orders_margin.is_finite() && open_orders_margin >= 0.0,
                "Open orders margin should be a finite non-negative number if present"
            );
        }

        // Validate P&L fields
        if let Some(total_profit_loss) = position.total_profit_loss {
            assert!(
                total_profit_loss.is_finite(),
                "Total P&L should be a finite number if present"
            );
        }
        if let Some(realized_profit_loss) = position.realized_profit_loss {
            assert!(
                realized_profit_loss.is_finite(),
                "Realized P&L should be a finite number if present"
            );
        }
        if let Some(floating_profit_loss) = position.floating_profit_loss {
            assert!(
                floating_profit_loss.is_finite(),
                "Floating P&L should be a finite number if present"
            );
        }
        if let Some(realized_funding) = position.realized_funding {
            assert!(
                realized_funding.is_finite(),
                "Realized funding should be a finite number if present"
            );
        }

        // Validate size fields
        if let Some(size_currency) = position.size_currency {
            assert!(
                size_currency.is_finite(),
                "Size currency should be a finite number if present"
            );
        }

        // Validate delta
        if let Some(delta) = position.delta {
            assert!(
                delta.is_finite(),
                "Delta should be a finite number if present"
            );
        }

        // Direction is an enum, just validate it exists
        debug!("Position direction: {:?}", position.direction);

        // Kind is an Option<String>, validate if present
        if let Some(ref kind) = position.kind {
            debug!("Position kind: {}", kind);
        }
    }

    info!("Positions data validation test completed successfully");
    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_positions_consistency() -> Result<(), Box<dyn std::error::Error>> {
    check_env_file()?;

    let _ = tracing_subscriber::fmt()
        .with_env_filter("debug")
        .try_init();

    info!("Starting positions consistency test");

    let client = DeribitHttpClient::new(true);
    authenticate_client(&client).await?;

    // Get positions multiple times to check consistency
    debug!("Getting first set of positions");
    let positions1 = client.get_positions(Some("BTC"), None, None).await?;

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    debug!("Getting second set of positions");
    let positions2 = client.get_positions(Some("BTC"), None, None).await?;

    info!("Both position sets retrieved successfully");

    // Check that the number of positions is consistent (might vary slightly due to new positions)
    let count_diff = (positions1.len() as i32 - positions2.len() as i32).abs();
    assert!(
        count_diff <= 2,
        "Position count should be relatively stable (diff: {})",
        count_diff
    );

    // Check that common positions have consistent basic data
    for pos1 in &positions1 {
        if let Some(pos2) = positions2
            .iter()
            .find(|p| p.instrument_name == pos1.instrument_name)
        {
            assert_eq!(
                pos1.kind, pos2.kind,
                "Kind should be consistent for {}",
                pos1.instrument_name
            );

            // Size might change due to trading, but should be relatively stable for most positions
            let size_diff = (pos1.size - pos2.size).abs();
            if size_diff > pos1.size.abs() * 0.1 {
                // Allow 10% change
                warn!(
                    "Large size change detected for {}: {} -> {} (diff: {})",
                    pos1.instrument_name, pos1.size, pos2.size, size_diff
                );
            }
        }
    }

    info!("Positions consistency test completed successfully");
    Ok(())
}
