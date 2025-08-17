# Deribit HTTP Client Basic Examples

This directory contains basic examples to demonstrate the usage of the Deribit HTTP client.

## Available Examples

- Public Endpoints (`public_endpoints`)
  - Demonstrates: `/public/get_time`, `/public/status`, `/public/test` (and documents that `/public/hello` is WebSocket-only)
- Authentication Endpoints (`authentication_endpoints`)
  - Demonstrates: `/public/auth`, `/public/exchange_token`, `/public/fork_token`, `/private/logout`
- Market Data Endpoints (`market_data_endpoints`)
  - Demonstrates: `/public/get_currencies`, `/public/get_apr_history`, `/public/get_book_summary_by_currency`, `/public/get_book_summary_by_instrument`, `/public/get_contract_size`
- Order Book Endpoints (`order_book_endpoints`)
  - Demonstrates: `/public/get_order_book`, `/public/get_order_book_by_instrument_id`
- Index Endpoints (`index_endpoints`)
  - Demonstrates: `/public/get_index`, `/public/get_index_price`, `/public/get_index_price_names`
- Instrument Endpoints (`instrument_endpoints`)
  - Demonstrates: `/public/get_instruments`, `/public/get_instrument`
- Delivery & Expirations Endpoints (`delivery_expirations_endpoints`)
  - Demonstrates: `/public/get_delivery_prices`, `/public/get_expirations`
- Funding Rate Endpoints (`funding_rate_endpoints`)
  - Demonstrates: `/public/get_funding_rate_history`, `/public/get_funding_rate_value`
- Historical Volatility Endpoints (`historical_volatility_endpoints`)
  - Demonstrates: `/public/get_historical_volatility`
- Settlement Endpoints (`settlement_endpoints`)
  - Demonstrates: `/public/get_last_settlements_by_currency`, `/public/get_last_settlements_by_instrument`
- Ticker Endpoints (`ticker_endpoints`)
  - Demonstrates: `/public/ticker`
- Trade Endpoints (`trade_endpoints`)
  - Demonstrates: `/public/get_last_trades_by_currency`, `/public/get_last_trades_by_currency_and_time`, `/public/get_last_trades_by_instrument`, `/public/get_last_trades_by_instrument_and_time`
- TradingView Chart Endpoints (`tradingview_chart_endpoints`)
  - Demonstrates: `/public/get_tradingview_chart_data`

## Configuration

### Required Environment Variables

Create a `.env` file in the project's root directory (`deribit-http/.env`) with the following variables:

```bash
# Deribit Testnet OAuth2 Credentials
DERIBIT_CLIENT_ID=your_client_id_here
DERIBIT_CLIENT_SECRET=your_client_secret_here

# Optional Configuration
DERIBIT_TESTNET=true
DERIBIT_HTTP_TIMEOUT=30
```

### Obtaining Deribit Credentials

1. Go to [Deribit Testnet](https://test.deribit.com)
2. Create an account or log in
3. Go to **Account** → **API**
4. Create a new API Key with the necessary permissions:
   - `account:read`
   - `trade:read_write` (optional)
   - `wallet:read` (optional)
5. Copy the `Client ID` and `Client Secret` to your `.env` file

## Running the Examples

You can run any example from this directory with:

```bash
# From the deribit-http/ directory
cd examples/public
cargo run --bin <binary_name>
```

For example:

```bash
cargo run --bin public_endpoints
cargo run --bin authentication_endpoints
cargo run --bin market_data_endpoints
cargo run --bin order_book_endpoints
```

## Demonstrated Features

### Authentication Endpoints

#### 1. OAuth2 Authentication (`/public/auth`)
- Initial authentication with client_id and client_secret
- Obtaining access_token and refresh_token
- Verification of scope and permissions

#### 2. Token Exchange (`/public/exchange_token`)
- Exchange refresh_token for a new access_token
- Change of subject_id to access subaccounts
- Customization of scope for the new session

#### 3. Token Fork (`/public/fork_token`)
- Creation of a new session with the same permissions
- Assignment of a custom name to the session
- Maintenance of the original token's permissions

#### 4. Logout (`/private/logout`)
- Invalidation of the current token on the server
- Secure session closure
- Clearing of local authentication state

## Example Output

When you run the authentication endpoints example, you will see an output similar to this:

```
🚀 Deribit HTTP Client - Authentication Endpoints Example
================================================================

✅ Credentials found in environment variables
📋 Client ID: FdRo6Dxh...

✅ HTTP client created for testnet: https://test.deribit.com/api/v2

🔐 1. INITIAL OAUTH2 AUTHENTICATION
-----------------------------------
✅ OAuth2 authentication successful
📄 Token type: bearer
⏰ Expires in: 900 seconds
🔑 Access token: 1755358792907.1bzKD...
🔄 Refresh token: 1755962692907.1Z7FU...
🎯 Scope: session:rest-6fLVUiTbfwM= block_trade:read_write trade:read_write...
🆔 Session ID: 62178.FdRo6Dxh.rest-6fLVUiTbfwM=

🔄 2. TOKEN EXCHANGE FOR DIFFERENT SUBJECT_ID
--------------------------------------------
✅ Token exchange successful
🎯 Subject ID: 10
...
```

## Error Handling

The examples include comprehensive error handling for:

- **Missing or invalid credentials**
- **Network connectivity issues**
- **Deribit API errors**
- **Expired or invalid tokens**
- **Insufficient permissions**

## Logging

The examples use `tracing` for detailed logging. You can adjust the logging level with the environment variable:

```bash
RUST_LOG=debug cargo run --bin authentication_endpoints
```

Available levels: `error`, `warn`, `info`, `debug`, `trace`

## Important Notes

1. **Testnet vs Production**: Examples are configured to use Deribit Testnet by default
2. **Rate Limiting**: The client includes automatic rate limiting
3. **Security**: Never hardcode credentials in source code
4. **Tokens**: Access tokens have limited duration (typically 15 minutes)
5. **Refresh Tokens**: Refresh tokens can be used to obtain new access tokens

## Troubleshooting

### Error: "Missing required environment variable"
- Verify that the `.env` file exists and contains the correct variables
- Make sure you're running from the correct directory

### Error: "OAuth2 authentication failed: bad_request"
- Verify that your credentials are correct
- Make sure you're using Testnet credentials for the example

### Error: "Method not found"
- Verify that the base URL is correct
- Make sure there's no duplication of `/api/v2` in the URL

## Additional Resources

- [Deribit API Documentation](https://docs.deribit.com/)
- [Deribit Testnet](https://test.deribit.com)
- [OAuth2 Documentation](https://docs.deribit.com/#authentication)

## Per-example Usage and Expected Output

### Public Endpoints (`public_endpoints`)
Run:
```bash
cargo run --bin public_endpoints
```
What you’ll see:
- Server time in milliseconds returned by `/public/get_time`
- A handled informational error for `/public/hello` (WebSocket-only)
- Platform lock status and locked indices from `/public/status`
- Successful connectivity test result from `/public/test`

### Authentication Endpoints (`authentication_endpoints`)
Run:
```bash
cargo run --bin authentication_endpoints
```
What you’ll see:
- Successful OAuth2 authentication via `/public/auth`
- Token exchange via `/public/exchange_token`
- Session fork via `/public/fork_token`
- Logout via `/private/logout`
- Detailed token metadata, scope and session identifiers

### Market Data Endpoints (`market_data_endpoints`)
Run:
```bash
cargo run --bin market_data_endpoints
```
What you’ll see:
- List of supported currencies from `/public/get_currencies`
- APR history for supported yield tokens from `/public/get_apr_history` (may be unavailable on testnet for some assets)
- Book summary by currency and by instrument
- Contract size for futures/options (denominated appropriately per instrument type)

### Order Book Endpoints (`order_book_endpoints`)
Run:
```bash
cargo run --bin order_book_endpoints
```
What you’ll see:
- Top-of-book and depth aggregation for `/public/get_order_book`
- Order book by instrument ID via `/public/get_order_book_by_instrument_id`
- Spread and simple depth analytics, plus handled errors for invalid inputs

### Index Endpoints (`index_endpoints`)
Run:
```bash
cargo run --bin index_endpoints
```
What you’ll see:
- Index values via `/public/get_index` and `/public/get_index_price`
- Available index price names via `/public/get_index_price_names`

### Instrument Endpoints (`instrument_endpoints`)
Run:
```bash
cargo run --bin instrument_endpoints
```
What you’ll see:
- Instrument lists filtered by currency and kind via `/public/get_instruments`
- Single instrument metadata via `/public/get_instrument`

### Delivery & Expirations Endpoints (`delivery_expirations_endpoints`)
Run:
```bash
cargo run --bin delivery_expirations_endpoints
```
What you’ll see:
- Delivery prices via `/public/get_delivery_prices`
- Available expirations via `/public/get_expirations`

### Funding Rate Endpoints (`funding_rate_endpoints`)
Run:
```bash
cargo run --bin funding_rate_endpoints
```
What you’ll see:
- Funding rate history via `/public/get_funding_rate_history`
- Current funding rate value via `/public/get_funding_rate_value`

### Historical Volatility Endpoints (`historical_volatility_endpoints`)
Run:
```bash
cargo run --bin historical_volatility_endpoints
```
What you’ll see:
- Historical volatility time series via `/public/get_historical_volatility`

### Settlement Endpoints (`settlement_endpoints`)
Run:
```bash
cargo run --bin settlement_endpoints
```
What you’ll see:
- Last settlements by currency via `/public/get_last_settlements_by_currency`
- Last settlements by instrument via `/public/get_last_settlements_by_instrument`

### Ticker Endpoints (`ticker_endpoints`)
Run:
```bash
cargo run --bin ticker_endpoints
```
What you’ll see:
- Real-time ticker snapshot for a selected instrument via `/public/ticker`

### Trade Endpoints (`trade_endpoints`)
Run:
```bash
cargo run --bin trade_endpoints
```
What you’ll see:
- Last trades by currency and by instrument
- Time-bounded queries for trades

### TradingView Chart Endpoints (`tradingview_chart_endpoints`)
Run:
```bash
cargo run --bin tradingview_chart_endpoints
```
What you’ll see:
- OHLCV series suitable for TradingView via `/public/get_tradingview_chart_data`

> Note: Testnet instrument availability varies. Some instruments or assets (e.g., specific expiries or yield tokens) may not be present at all times, which will be logged as handled warnings.