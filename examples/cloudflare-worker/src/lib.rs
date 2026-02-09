//! Cloudflare Worker example using deribit-http with WASM
//!
//! This example demonstrates how to use the deribit-http crate in a
//! Cloudflare Worker environment to fetch public market data.
//!
//! ## Running locally
//!
//! ```bash
//! npx wrangler dev
//! ```
//!
//! ## Building
//!
//! ```bash
//! npx wrangler build
//! ```

use deribit_http::prelude::{DeribitHttpClient, HttpConfig, setup_logger};
use worker::*;

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    // Set up logging for WASM (routes to console.log)
    setup_logger();

    let url = req.url()?;
    let path = url.path();

    match path {
        "/" => handle_root().await,
        "/currencies" => handle_currencies().await,
        "/ticker" => handle_ticker(&url).await,
        _ => Response::error("Not Found", 404),
    }
}

async fn handle_root() -> Result<Response> {
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Deribit HTTP Worker Example</title>
</head>
<body>
    <h1>Deribit HTTP Worker Example</h1>
    <p>Available endpoints:</p>
    <ul>
        <li><a href="/currencies">/currencies</a> - List available currencies</li>
        <li><a href="/ticker?instrument=BTC-PERPETUAL">/ticker?instrument=BTC-PERPETUAL</a> - Get ticker for an instrument</li>
    </ul>
</body>
</html>
    "#;
    Response::from_html(html)
}

async fn handle_currencies() -> Result<Response> {
    // Create client with default testnet configuration
    let config = HttpConfig::default();
    let client = DeribitHttpClient::with_config(config);

    match client.get_currencies().await {
        Ok(currencies) => {
            let json = serde_json::to_string_pretty(&currencies)
                .map_err(|e| Error::RustError(e.to_string()))?;
            Response::ok(json).map(|r| r.with_headers(json_headers()))
        }
        Err(e) => Response::error(format!("Failed to fetch currencies: {}", e), 500),
    }
}

async fn handle_ticker(url: &Url) -> Result<Response> {
    let instrument = url
        .query_pairs()
        .find(|(k, _)| k == "instrument")
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| "BTC-PERPETUAL".to_string());

    // Create client with default testnet configuration
    let config = HttpConfig::default();
    let client = DeribitHttpClient::with_config(config);

    match client.get_ticker(&instrument).await {
        Ok(ticker) => {
            let json = serde_json::to_string_pretty(&ticker)
                .map_err(|e| Error::RustError(e.to_string()))?;
            Response::ok(json).map(|r| r.with_headers(json_headers()))
        }
        Err(e) => Response::error(format!("Failed to fetch ticker: {}", e), 500),
    }
}

fn json_headers() -> Headers {
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json").ok();
    headers
}
