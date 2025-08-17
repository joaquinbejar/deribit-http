# Deribit HTTP Client Private Examples

This directory contains private (authenticated) examples demonstrating trading, order management, user trades, and mass quotes using the Deribit HTTP client.

## Available Examples

- Trading Endpoints (`trading_endpoints`)
  - Demonstrates: `/private/buy`, `/private/sell`, `/private/edit`, `/private/edit_by_label`
- Cancellation Endpoints (`cancellation_endpoints`)
  - Demonstrates: `/private/cancel`, `/private/cancel_all`, `/private/cancel_all_by_currency`, `/private/cancel_all_by_currency_pair`, `/private/cancel_all_by_instrument`, `/private/cancel_all_by_kind_or_type`, `/private/cancel_by_label`
- Order Query Endpoints (`order_query_endpoints`)
  - Demonstrates: `/private/get_open_orders`, `/private/get_open_orders_by_currency`, `/private/get_open_orders_by_instrument`, `/private/get_open_orders_by_label`, `/private/get_order_history_by_currency`, `/private/get_order_history_by_instrument`, `/private/get_order_state`
- Order History Endpoints (`order_history_endpoints`)
  - Demonstrates: `/private/get_order_history`, `/private/get_stop_order_history`
- User Trades Endpoints (`user_trades_endpoints`)
  - Demonstrates: `/private/get_user_trades_by_currency`, `/private/get_user_trades_by_currency_and_time`, `/private/get_user_trades_by_instrument`, `/private/get_user_trades_by_instrument_and_time`, `/private/get_user_trades_by_order`
- Mass Quote Endpoints (`mass_quote_endpoints`)
  - Demonstrates: `/private/mass_quote`, `/private/cancel_quotes`

## Configuration

### Required Environment Variables

Create a `.env` file in the project root (`deribit-http/.env`):

```bash
# Deribit Testnet OAuth2 Credentials
DERIBIT_CLIENT_ID=your_client_id_here
DERIBIT_CLIENT_SECRET=your_client_secret_here

# Optional
DERIBIT_TESTNET=true
DERIBIT_HTTP_TIMEOUT=30
```

### OAuth2 Scopes

Ensure your API key has appropriate scopes for private endpoints:
- account:read
- trade:read_write (required for placing/editing/canceling orders and quotes)
- wallet:read (optional)

Obtain credentials from Deribit Testnet: https://test.deribit.com (Account → API).

## Running the Examples

From the `deribit-http/` directory:

```bash
cd examples/private
cargo run --bin <binary_name>
```

For example:

```bash
cargo run --bin trading_endpoints
cargo run --bin cancellation_endpoints
cargo run --bin order_query_endpoints
cargo run --bin order_history_endpoints
cargo run --bin user_trades_endpoints
cargo run --bin mass_quote_endpoints
```

## Common Patterns and Safety

- All examples authenticate via OAuth2 using `DERIBIT_CLIENT_ID` and `DERIBIT_CLIENT_SECRET`.
- Testnet is enabled by default for safety. Do not run these examples on production unless you know what you are doing.
- Order examples use conservative prices and/or `post_only` where possible to avoid immediate execution. Nevertheless, orders may still execute in changing markets.
- Examples often create test orders and then cancel them during cleanup.

## Logging

Examples use `tracing` for structured logs. Adjust verbosity:

```bash
RUST_LOG=debug cargo run --bin trading_endpoints
```

Available levels: error, warn, info, debug, trace.

## Error Handling

Examples include robust error handling for:
- Missing/invalid environment variables
- OAuth2 authentication failures
- API errors (invalid params, insufficient scope, etc.)
- Network timeouts or connectivity issues

## Troubleshooting

- "Missing required environment variable": ensure `.env` exists with correct values and you run from the crate root before `cd examples/private`.
- "OAuth2 authentication failed": verify Testnet credentials and scopes.
- "Method not found": confirm base URL and no duplicated `/api/v2`.
- "Insufficient permissions": add the required scopes to your API key.

## Additional Resources

- Deribit API Docs: https://docs.deribit.com/
- Deribit Testnet: https://test.deribit.com
- OAuth2 Guide: https://docs.deribit.com/#authentication

## Per-example Usage and Expected Output

### Trading Endpoints (`trading_endpoints`)
Run:
```bash
cargo run --bin trading_endpoints
```
What you’ll see:
- Successful OAuth2 auth
- A limit buy and a limit sell placed (prices set to avoid fill)
- Edit by ID and edit by label attempts
- Optional cleanup via `/private/cancel`

### Cancellation Endpoints (`cancellation_endpoints`)
Run:
```bash
cargo run --bin cancellation_endpoints
```
What you’ll see:
- Several test orders created with labels/instruments
- Single cancel, cancel by label, cancel by instrument
- Cancel by currency and mass cancellations

### Order Query Endpoints (`order_query_endpoints`)
Run:
```bash
cargo run --bin order_query_endpoints
```
What you’ll see:
- Creation of diverse test orders
- Queries across open orders (all/by currency/by instrument/by label)
- History by currency/instrument and single order state

### Order History Endpoints (`order_history_endpoints`)
Run:
```bash
cargo run --bin order_history_endpoints
```
What you’ll see:
- Mark prices fetched to place realistic orders
- Limit and stop-limit test orders created
- Order history and stop order history fetched

### User Trades Endpoints (`user_trades_endpoints`)
Run:
```bash
cargo run --bin user_trades_endpoints
```
What you’ll see:
- Mark prices fetched and near-market orders placed
- Possible partial fills generating trades
- User trades retrieved by currency/instrument/time and by order

### Mass Quote Endpoints (`mass_quote_endpoints`)
Run:
```bash
cargo run --bin mass_quote_endpoints
```
What you’ll see:
- Mass quotes submitted for multiple instruments
- Verification using auxiliary queries
- Optional quote cancellations