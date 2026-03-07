# Deribit HTTP Client Private Examples

This directory contains private (authenticated) examples demonstrating trading, order management, user trades, and mass quotes using the Deribit HTTP client.

## Available Examples

- Accounting Endpoints (`accounting_endpoints`)
  - Demonstrates: `/private/get_account_summary`, `/private/get_account_summaries`, `/private/get_positions`, `/private/get_subaccounts`, `/private/get_transaction_log`
- Trading Endpoints (`trading_endpoints`)
  - Demonstrates: `/private/buy`, `/private/sell`, `/private/edit`, `/private/edit_by_label`
- Cancellation Endpoints (`cancellation_endpoints`)
  - Demonstrates: `/private/cancel`, `/private/cancel_all`, `/private/cancel_all_by_currency`, `/private/cancel_all_by_currency_pair`, `/private/cancel_all_by_instrument`, `/private/cancel_all_by_kind_or_type`, `/private/cancel_by_label`
- Order Query Endpoints (`order_query_endpoints`)
  - Demonstrates: `/private/get_open_orders`, `/private/get_open_orders_by_currency`, `/private/get_open_orders_by_instrument`, `/private/get_open_orders_by_label`, `/private/get_order_history_by_currency`, `/private/get_order_history_by_instrument`, `/private/get_order_state`
- Order History Endpoints (`order_history_endpoints`)
  - Demonstrates: `/private/get_order_history`, `/private/get_trigger_order_history`
- User Trades Endpoints (`user_trades_endpoints`)
  - Demonstrates: `/private/get_user_trades_by_currency`, `/private/get_user_trades_by_currency_and_time`, `/private/get_user_trades_by_instrument`, `/private/get_user_trades_by_instrument_and_time`, `/private/get_user_trades_by_order`
- Mass Quote Endpoints (`mass_quote_endpoints`)
  - Demonstrates: `/private/mass_quote`, `/private/cancel_quotes`

### New in v0.6.0

- Wallet Endpoints (`wallet_endpoints`)
  - Demonstrates: `/private/get_current_deposit_address`, `/private/create_deposit_address`, `/private/get_address_book`, `/private/add_to_address_book`, `/private/remove_from_address_book`, `/private/withdraw`, `/private/cancel_withdrawal`
- Subaccount Endpoints (`subaccount_endpoints`)
  - Demonstrates: `/private/get_subaccounts`, `/private/get_subaccounts_details`, `/private/create_subaccount`, `/private/change_subaccount_name`, `/private/set_email_for_subaccount`, `/private/toggle_subaccount_login`, `/private/toggle_notifications_from_subaccount`, `/private/remove_subaccount`
- API Key Endpoints (`api_key_endpoints`)
  - Demonstrates: `/private/list_api_keys`, `/private/get_api_key`, `/private/create_api_key`, `/private/edit_api_key`, `/private/enable_api_key`, `/private/disable_api_key`, `/private/reset_api_key`, `/private/set_api_key_as_default`, `/private/remove_api_key`
- Transfer Endpoints (`transfer_endpoints`)
  - Demonstrates: `/private/get_transfers`, `/private/submit_transfer_to_subaccount`, `/private/submit_transfer_to_user`, `/private/submit_transfer_between_subaccounts`, `/private/cancel_transfer_by_id`
- Beneficiary Endpoints (`beneficiary_endpoints`)
  - Demonstrates: `/private/list_address_beneficiaries`, `/private/get_address_beneficiary`, `/private/save_address_beneficiary`, `/private/delete_address_beneficiary`
- Block Trade Endpoints (`block_trade_endpoints`)
  - Demonstrates: `/private/get_last_block_trades_by_currency`, `/private/get_block_trade`, `/private/execute_block_trade`, `/private/verify_block_trade`, `/private/invalidate_block_trade_signature`
- Combo Endpoints (`combo_endpoints`)
  - Demonstrates: `/private/get_combo_ids`, `/private/get_combo_details`, `/private/create_combo`, `/private/verify_combo`, `/private/execute_combo`
- Position Endpoints (`position_endpoints`)
  - Demonstrates: `/private/get_position`, `/private/get_positions`, `/private/move_positions`

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
cargo run --bin accounting_endpoints
cargo run --bin trading_endpoints
cargo run --bin cancellation_endpoints
cargo run --bin order_query_endpoints
cargo run --bin order_history_endpoints
cargo run --bin user_trades_endpoints
cargo run --bin mass_quote_endpoints

# New in v0.6.0
cargo run --bin wallet_endpoints
cargo run --bin subaccount_endpoints
cargo run --bin api_key_endpoints
cargo run --bin transfer_endpoints
cargo run --bin beneficiary_endpoints
cargo run --bin block_trade_endpoints
cargo run --bin combo_endpoints
cargo run --bin position_endpoints
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

### Wallet Endpoints (`wallet_endpoints`) - New in v0.6.0
Run:
```bash
cargo run --bin wallet_endpoints
```
What you’ll see:
- Current deposit address retrieval
- Deposit address creation
- Address book listing (withdrawal/transfer addresses)
- Demonstration of withdraw/cancel patterns (not executed)

### Subaccount Endpoints (`subaccount_endpoints`) - New in v0.6.0
Run:
```bash
cargo run --bin subaccount_endpoints
```
What you’ll see:
- List of all subaccounts with portfolio info
- Subaccount details with positions
- Demonstration of create/modify/remove patterns

### API Key Endpoints (`api_key_endpoints`) - New in v0.6.0
Run:
```bash
cargo run --bin api_key_endpoints
```
What you’ll see:
- List of all API keys with scopes
- Specific API key details
- Demonstration of create/edit/enable/disable patterns

### Transfer Endpoints (`transfer_endpoints`) - New in v0.6.0
Run:
```bash
cargo run --bin transfer_endpoints
```
What you’ll see:
- Transfer history for BTC/ETH
- Demonstration of transfer patterns (not executed)

### Beneficiary Endpoints (`beneficiary_endpoints`) - New in v0.6.0
Run:
```bash
cargo run --bin beneficiary_endpoints
```
What you’ll see:
- List of address beneficiaries (Travel Rule compliance)
- Demonstration of save/delete patterns

### Block Trade Endpoints (`block_trade_endpoints`) - New in v0.6.0
Run:
```bash
cargo run --bin block_trade_endpoints
```
What you’ll see:
- Recent block trades by currency
- Demonstration of execute/verify patterns

### Combo Endpoints (`combo_endpoints`) - New in v0.6.0
Run:
```bash
cargo run --bin combo_endpoints
```
What you’ll see:
- Available combo IDs by currency
- Demonstration of create/verify/execute patterns

### Position Endpoints (`position_endpoints`) - New in v0.6.0
Run:
```bash
cargo run --bin position_endpoints
```
What you’ll see:
- All positions with filtering (currency/kind)
- Specific position details
- Demonstration of move_positions pattern