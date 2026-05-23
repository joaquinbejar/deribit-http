//! Options Order Types Example
//!
//! Exercises every order-type permutation supported by `OrderRequest` against a
//! BTC_USDC option that has both bid and ask liquidity.
//!
//! Flow:
//! 1. Discover BTC_USDC option expirations via `/public/get_expirations`.
//! 2. Pick the first expiry.
//! 3. Pull the option chain for that expiry (`USDC` instruments filtered by
//!    `BTC_USDC-{expiry}-` prefix, with per-instrument tickers).
//! 4. Pick the strike with the tightest two-sided market (non-zero bid AND ask).
//! 5. Place every order-type / TIF / advanced / trigger / linked variant on
//!    that instrument, well away from the touch so nothing fills.
//! 6. Cancel everything at the end (`cancel_all_by_instrument`).
//!
//! ```text
//!  ┌─────────────────────────── ORDER TYPE MATRIX (OrderRequest) ──────────────────────────────┐
//!  │                                                                                            │
//!  │   PRICE-BASED (no trigger)                                                                 │
//!  │   ─────────────────────────                                                                │
//!  │   Limit         → price, time_in_force                                                     │
//!  │   Market        → no price (uses NBBO)                                                     │
//!  │   MarketLimit   → market w/ price ceiling/floor protection                                 │
//!  │                                                                                            │
//!  │   TRIGGER-BASED (require trigger + trigger_price | trigger_offset)                         │
//!  │   ──────────────────────────────────────────────────────────────                           │
//!  │   StopLimit     → trigger → becomes limit  @ price                                         │
//!  │   StopMarket    → trigger → becomes market                                                 │
//!  │   TakeLimit     → take-profit, becomes limit  @ price                                      │
//!  │   TakeMarket    → take-profit, becomes market                                              │
//!  │   TrailingStop  → trigger_offset trails best price                                         │
//!  │                                                                                            │
//!  │   TIME IN FORCE                ADVANCED (options-only pricing)                             │
//!  │   ─────────────                ──────────────────────────────                              │
//!  │   GoodTilCancelled (GTC)       Usd   → price denominated in USD                            │
//!  │   GoodTilDay      (GTD)        Implv → price denominated in implied volatility             │
//!  │   FillOrKill      (FOK)                                                                    │
//!  │   ImmediateOrCancel (IOC)      FLAGS                                                       │
//!  │                                ─────                                                       │
//!  │   LINKED ORDERS                post_only, reject_post_only, reduce_only, mmp               │
//!  │   ─────────────                                                                            │
//!  │   OneTriggersOther     (OTO)                                                               │
//!  │   OneCancelsOther      (OCO)                                                               │
//!  │   OneTriggersOneCancelsOther (OTOCO)                                                       │
//!  │                                                                                            │
//!  └────────────────────────────────────────────────────────────────────────────────────────────┘
//! ```
//!
//! Env vars:
//! - DERIBIT_CLIENT_ID
//! - DERIBIT_CLIENT_SECRET
//! - DERIBIT_TESTNET   (default: true)
//!
//! Run: `cargo run --bin options_order_types_example`

use deribit_http::prelude::*;
use std::env;
use std::path::Path;
use tracing::{error, info, warn};

const USDC_CURRENCY: &str = "USDC";
const BTC_USDC_PAIR: &str = "btc_usdc";
const BTC_USDC_PREFIX: &str = "BTC_USDC";

/// Picked option instrument with usable bid/ask.
#[derive(Debug, Clone)]
struct LiquidOption {
    instrument_name: String,
    strike: f64,
    best_bid: f64,
    best_ask: f64,
    mid: f64,
    tick_size: f64,
    min_trade_amount: f64,
}

#[tokio::main]
async fn main() -> Result<(), HttpError> {
    if Path::new(".env").exists() {
        dotenv::dotenv().ok();
    }
    setup_logger();

    info!("🚀 Options Order Types Example (BTC_USDC)");
    info!("=========================================");

    let _client_id = env::var("DERIBIT_CLIENT_ID")
        .map_err(|_| HttpError::ConfigError("DERIBIT_CLIENT_ID not set".to_string()))?;
    let _client_secret = env::var("DERIBIT_CLIENT_SECRET")
        .map_err(|_| HttpError::ConfigError("DERIBIT_CLIENT_SECRET not set".to_string()))?;
    let testnet = env::var("DERIBIT_TESTNET")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);
    info!(
        "Environment: {}",
        if testnet { "Testnet" } else { "Production" }
    );

    let client = DeribitHttpClient::default();

    // ─────────────────────────────────────────────────────────────────────
    // 1. Fetch BTC_USDC option expirations
    // ─────────────────────────────────────────────────────────────────────
    info!("📅 1. FETCH EXPIRATIONS — currency=USDC kind=option pair=btc_usdc");
    let expirations = client
        .get_expirations(USDC_CURRENCY, "option", Some(BTC_USDC_PAIR))
        .await?;
    let expiry = pick_first_expiry(&expirations).ok_or_else(|| {
        HttpError::RequestFailed("No BTC_USDC option expirations available".to_string())
    })?;
    info!("✅ First expiry: {}", expiry);

    // ─────────────────────────────────────────────────────────────────────
    // 2. Fetch option chain for that expiry
    // ─────────────────────────────────────────────────────────────────────
    info!(
        "📋 2. FETCH OPTION CHAIN for {}-{}",
        BTC_USDC_PREFIX, expiry
    );
    let chain = fetch_btc_usdc_chain(&client, &expiry).await?;
    info!(
        "✅ Got {} BTC_USDC instruments expiring {}",
        chain.len(),
        expiry
    );

    // ─────────────────────────────────────────────────────────────────────
    // 3. Pick a strike with two-sided liquidity
    // ─────────────────────────────────────────────────────────────────────
    info!("🎯 3. SELECT STRIKE WITH BID+ASK LIQUIDITY");
    let chosen = pick_liquid_instrument(&chain).ok_or_else(|| {
        HttpError::RequestFailed("No BTC_USDC option with both bid and ask liquidity".to_string())
    })?;
    info!(
        "✅ Picked {} | strike={} | bid={} ask={} mid={} tick={}",
        chosen.instrument_name,
        chosen.strike,
        chosen.best_bid,
        chosen.best_ask,
        chosen.mid,
        chosen.tick_size
    );

    // Pricing helpers — keep us nowhere near the touch.
    let amount = chosen.min_trade_amount.max(0.1);
    let far_bid = round_tick(chosen.best_bid * 0.10, chosen.tick_size).max(chosen.tick_size);
    let far_ask = round_tick(chosen.best_ask * 5.00, chosen.tick_size).max(chosen.tick_size);
    info!(
        "Order plan: amount={} far_bid={} far_ask={}",
        amount, far_bid, far_ask
    );
    println!();

    let mut placed: Vec<OrderOutcome> = Vec::new();

    // ─────────────────────────────────────────────────────────────────────
    // 4. Place every order-type variant
    // ─────────────────────────────────────────────────────────────────────

    // 4.1 Limit + GoodTilCancelled + post_only
    info!("📝 4.1 LIMIT  | GTC + post_only");
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            price: Some(far_bid),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            post_only: Some(true),
            label: Some("ex_limit_gtc_postonly".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.2 Limit + ImmediateOrCancel + reject_post_only
    info!("📝 4.2 LIMIT  | IOC + reject_post_only");
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            price: Some(far_bid),
            time_in_force: Some(TimeInForce::ImmediateOrCancel),
            reject_post_only: Some(true),
            label: Some("ex_limit_ioc".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.3 Limit + FillOrKill
    info!("📝 4.3 LIMIT  | FOK");
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            price: Some(far_bid),
            time_in_force: Some(TimeInForce::FillOrKill),
            label: Some("ex_limit_fok".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.4 Limit + GoodTilDay
    info!("📝 4.4 LIMIT  | GTD");
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            price: Some(far_bid),
            time_in_force: Some(TimeInForce::GoodTilDay),
            post_only: Some(true),
            label: Some("ex_limit_gtd".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.5 Limit + advanced=Usd  (price in USD)
    info!("📝 4.5 LIMIT  | advanced=Usd  (price denominated in USD)");
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            // USD price → use a low USD figure to stay off the book.
            price: Some(1.0),
            advanced: Some(AdvancedOrderType::Usd),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            post_only: Some(true),
            label: Some("ex_limit_usd".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.6 Limit + advanced=Implv  (price in IV)
    // Note: Deribit rejects post_only combined with advanced=Implv.
    info!("📝 4.6 LIMIT  | advanced=Implv (price denominated in implied vol)");
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            // Very low IV bid — won't fill.
            price: Some(5.0),
            advanced: Some(AdvancedOrderType::Implv),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            label: Some("ex_limit_implv".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.7 Limit + mmp (market-maker protection)
    info!("📝 4.7 LIMIT  | mmp=true (market-maker protection)");
    submit(
        &client,
        OrderSide::Sell,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            price: Some(far_ask),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            post_only: Some(true),
            mmp: Some(true),
            label: Some("ex_limit_mmp".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.8 Limit + display_amount (iceberg)
    // Linear option iceberg: total and slice must respect instrument contract-size
    // rules (err 11048 wrong_display_amount_for_option on bad multiples).
    info!("📝 4.8 LIMIT  | display_amount (iceberg)");
    let iceberg_total = 4.0;
    let iceberg_slice = 1.0;
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(iceberg_total),
            type_: Some(OrderType::Limit),
            price: Some(far_bid),
            display_amount: Some(iceberg_slice),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            post_only: Some(true),
            label: Some("ex_limit_iceberg".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.9 StopLimit  — buy stop above market
    info!("📝 4.9 STOP_LIMIT  | trigger=mark_price");
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::StopLimit),
            price: Some(far_ask),
            trigger_price: Some(round_tick(chosen.best_ask * 3.0, chosen.tick_size)),
            trigger: Some(Trigger::MarkPrice),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            label: Some("ex_stop_limit".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.10 StopMarket  — buy stop above market
    info!("📝 4.10 STOP_MARKET | trigger=index_price");
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::StopMarket),
            trigger_price: Some(round_tick(chosen.best_ask * 4.0, chosen.tick_size)),
            trigger: Some(Trigger::IndexPrice),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            label: Some("ex_stop_market".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.11 TakeLimit  — sell take-profit
    info!("📝 4.11 TAKE_LIMIT | trigger=last_price");
    submit(
        &client,
        OrderSide::Sell,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::TakeLimit),
            price: Some(far_ask),
            trigger_price: Some(round_tick(chosen.best_ask * 3.0, chosen.tick_size)),
            trigger: Some(Trigger::LastPrice),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            label: Some("ex_take_limit".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.12 TakeMarket  — sell take-profit
    info!("📝 4.12 TAKE_MARKET| trigger=mark_price");
    submit(
        &client,
        OrderSide::Sell,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::TakeMarket),
            trigger_price: Some(round_tick(chosen.best_ask * 4.0, chosen.tick_size)),
            trigger: Some(Trigger::MarkPrice),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            label: Some("ex_take_market".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.12b Plain Market sell — Market type accepts no TIF.
    info!("📝 4.12b MARKET (sell — will cross spread if liquidity exists)");
    submit(
        &client,
        OrderSide::Sell,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Market),
            label: Some("ex_market".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.13 MarketLimit  (market with price-protection)
    info!("📝 4.13 MARKET_LIMIT");
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::MarketLimit),
            time_in_force: Some(TimeInForce::ImmediateOrCancel),
            label: Some("ex_market_limit".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.14 TrailingStop  (trigger_offset trails market)
    info!("📝 4.14 TRAILING_STOP | trigger_offset");
    submit(
        &client,
        OrderSide::Sell,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::TrailingStop),
            trigger_offset: Some(round_tick(chosen.mid * 0.20, chosen.tick_size)),
            trigger: Some(Trigger::MarkPrice),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            label: Some("ex_trailing_stop".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.15 Reduce-only limit
    info!("📝 4.15 LIMIT  | reduce_only=true");
    submit(
        &client,
        OrderSide::Sell,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            price: Some(far_ask),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            post_only: Some(true),
            reduce_only: Some(true),
            label: Some("ex_reduce_only".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.16 valid_until — for linear options Deribit returns
    // "timestamp too far in the future" for any ms-epoch value we tried,
    // and "timestamp in past" for seconds. Likely the parameter is either
    // (a) only honored on inverse options, or (b) expected as something
    // other than ms or seconds since epoch. Reported as a finding; we
    // still hit the endpoint to record the exact rejection reason.
    info!("📝 4.16 LIMIT  | valid_until (ms-epoch + 10s — expected reject)");
    let valid_until = chrono::Utc::now().timestamp_millis() + 10_000;
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            price: Some(far_bid),
            time_in_force: Some(TimeInForce::GoodTilDay),
            post_only: Some(true),
            valid_until: Some(valid_until),
            label: Some("ex_valid_until".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.17 Linked: OTO (One-Triggers-Other)
    info!("📝 4.17 LINKED  | OneTriggersOther");
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            price: Some(far_bid),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            post_only: Some(true),
            linked_order_type: Some(LinkedOrderType::OneTriggersOther),
            trigger_fill_condition: Some(TriggerFillCondition::CompleteFill),
            label: Some("ex_oto".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.18 Linked: OCO (One-Cancels-Other)
    info!("📝 4.18 LINKED  | OneCancelsOther");
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            price: Some(far_bid),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            post_only: Some(true),
            linked_order_type: Some(LinkedOrderType::OneCancelsOther),
            trigger_fill_condition: Some(TriggerFillCondition::FirstHit),
            label: Some("ex_oco".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.19 Linked: OTOCO (One-Triggers-OneCancelsOther)
    info!("📝 4.19 LINKED  | OneTriggersOneCancelsOther");
    submit(
        &client,
        OrderSide::Buy,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            price: Some(far_bid),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            post_only: Some(true),
            linked_order_type: Some(LinkedOrderType::OneTriggersOneCancelsOther),
            trigger_fill_condition: Some(TriggerFillCondition::Incremental),
            label: Some("ex_otoco".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    // 4.20 Sell limit — confirms both sides reachable
    info!("📝 4.20 SELL LIMIT | GTC + post_only");
    submit(
        &client,
        OrderSide::Sell,
        OrderRequest {
            instrument_name: chosen.instrument_name.clone(),
            contracts: Some(amount),
            type_: Some(OrderType::Limit),
            price: Some(far_ask),
            time_in_force: Some(TimeInForce::GoodTilCancelled),
            post_only: Some(true),
            label: Some("ex_sell_limit_gtc".into()),
            ..base_request(&chosen.instrument_name)
        },
        &mut placed,
    )
    .await;

    println!();
    let accepted = placed.iter().filter(|o| o.accepted).count();
    let rejected = placed.len() - accepted;
    info!(
        "📦 SUMMARY — {} variants tested | {} accepted | {} rejected",
        placed.len(),
        accepted,
        rejected
    );
    println!();
    println!(
        "┌──────────────────────────────┬──────────┬──────────────────────────────────────────────────┐"
    );
    println!(
        "│ Variant                      │ Result   │ State / Server reason                            │"
    );
    println!(
        "├──────────────────────────────┼──────────┼──────────────────────────────────────────────────┤"
    );
    for o in &placed {
        let res = if o.accepted { "ACCEPTED" } else { "REJECTED" };
        let mut reason = o.state_or_reason.clone();
        if reason.len() > 48 {
            reason.truncate(45);
            reason.push_str("...");
        }
        println!("│ {:28} │ {:8} │ {:48} │", o.label, res, reason);
    }
    println!(
        "└──────────────────────────────┴──────────┴──────────────────────────────────────────────────┘"
    );
    println!();

    // ─────────────────────────────────────────────────────────────────────
    // 5. Cleanup — cancel everything on this instrument
    // ─────────────────────────────────────────────────────────────────────
    info!(
        "🧹 5. CLEANUP — cancel_all_by_instrument({})",
        chosen.instrument_name
    );
    match client
        .cancel_all_by_instrument(&chosen.instrument_name)
        .await
    {
        Ok(n) => info!("✅ Cancelled {} live orders", n),
        Err(e) => warn!("⚠️ Cleanup failed: {}", e),
    }

    info!("🎉 Options order-types example completed");
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────

fn base_request(instrument_name: &str) -> OrderRequest {
    OrderRequest {
        order_id: None,
        instrument_name: instrument_name.to_string(),
        amount: None,
        contracts: None,
        type_: None,
        label: None,
        price: None,
        time_in_force: None,
        display_amount: None,
        post_only: None,
        reject_post_only: None,
        reduce_only: None,
        trigger_price: None,
        trigger_offset: None,
        trigger: None,
        advanced: None,
        mmp: None,
        valid_until: None,
        linked_order_type: None,
        trigger_fill_condition: None,
        otoco_config: None,
    }
}

fn pick_first_expiry(resp: &ExpirationsResponse) -> Option<String> {
    // Direct fields (currency="any")
    if let Some(opts) = &resp.option
        && let Some(first) = opts.first()
    {
        return Some(first.clone());
    }
    // currency-keyed map
    for entry in resp.currencies.values() {
        if let Some(opts) = &entry.option
            && let Some(first) = opts.first()
        {
            return Some(first.clone());
        }
    }
    None
}

async fn fetch_btc_usdc_chain(
    client: &DeribitHttpClient,
    expiry: &str,
) -> Result<Vec<OptionInstrument>, HttpError> {
    let instruments = client
        .get_instruments(USDC_CURRENCY, Some("option"), Some(false))
        .await?;
    let prefix = format!("{}-{}-", BTC_USDC_PREFIX, expiry);
    let mut out = Vec::new();
    for inst in instruments {
        if !inst.instrument_name.starts_with(&prefix) {
            continue;
        }
        match client.get_ticker(&inst.instrument_name).await {
            Ok(t) => out.push(OptionInstrument {
                instrument: inst,
                ticker: t,
            }),
            Err(e) => warn!("⚠️ ticker fetch failed for {}: {}", inst.instrument_name, e),
        }
    }
    Ok(out)
}

fn pick_liquid_instrument(chain: &[OptionInstrument]) -> Option<LiquidOption> {
    // Tightest two-sided spread = most liquid.
    let mut best: Option<LiquidOption> = None;
    for opt in chain {
        let bid = opt.ticker.best_bid_price.unwrap_or(0.0);
        let ask = opt.ticker.best_ask_price.unwrap_or(0.0);
        if bid <= 0.0 || ask <= 0.0 || ask <= bid {
            continue;
        }
        let strike = match opt.instrument.strike {
            Some(s) => s,
            None => continue,
        };
        if opt.instrument.option_type.is_none() {
            continue;
        }
        let spread = ask - bid;
        let candidate = LiquidOption {
            instrument_name: opt.instrument.instrument_name.clone(),
            strike,
            best_bid: bid,
            best_ask: ask,
            mid: (bid + ask) / 2.0,
            tick_size: opt.instrument.tick_size.unwrap_or(0.0005),
            min_trade_amount: opt.instrument.min_trade_amount.unwrap_or(0.1),
        };
        match &best {
            None => best = Some(candidate),
            Some(cur) => {
                if spread < (cur.best_ask - cur.best_bid) {
                    best = Some(candidate);
                }
            }
        }
    }
    best
}

fn round_tick(price: f64, tick: f64) -> f64 {
    if tick <= 0.0 {
        return price;
    }
    (price / tick).round() * tick
}

#[derive(Debug, Clone)]
struct OrderOutcome {
    label: String,
    accepted: bool,
    state_or_reason: String,
}

async fn submit(
    client: &DeribitHttpClient,
    side: OrderSide,
    req: OrderRequest,
    placed: &mut Vec<OrderOutcome>,
) {
    let label = req.label.clone().unwrap_or_default();
    let result = match side {
        OrderSide::Buy => client.buy_order(req).await,
        OrderSide::Sell => client.sell_order(req).await,
    };
    match result {
        Ok(resp) => {
            info!(
                "   ✅ accepted: id={} state={} label={}",
                resp.order.order_id, resp.order.order_state, label
            );
            placed.push(OrderOutcome {
                label,
                accepted: true,
                state_or_reason: resp.order.order_state,
            });
        }
        Err(e) => {
            let msg = e.to_string();
            error!("   ❌ rejected ({}): {}", label, msg);
            let reason = extract_reason(&msg);
            placed.push(OrderOutcome {
                label,
                accepted: false,
                state_or_reason: reason,
            });
        }
    }
}

fn extract_reason(msg: &str) -> String {
    // Pull "reason":"..." out of the upstream Deribit JSON if present.
    if let Some(idx) = msg.find("\"reason\":\"")
        && let Some(rest) = msg.get(idx + "\"reason\":\"".len()..)
        && let Some(end) = rest.find('"')
    {
        return rest[..end].to_string();
    }
    if let Some(idx) = msg.find("\"message\":\"")
        && let Some(rest) = msg.get(idx + "\"message\":\"".len()..)
        && let Some(end) = rest.find('"')
    {
        return rest[..end].to_string();
    }
    msg.lines().next().unwrap_or(msg).to_string()
}
