# Deribit HTTP Client Basic Examples

This directory contains basic examples to demonstrate the usage of the Deribit HTTP client.

## Available Examples

### 1. Basic Example (`basic`)
A general example demonstrating the client's basic configuration and connectivity.

### 2. Authentication Endpoints (`authentication_endpoints`)
A complete example demonstrating all of Deribit's authentication endpoints:

- **`/public/auth`** - Initial OAuth2 authentication
- **`/public/exchange_token`** - Token exchange for different subject_ids
- **`/public/fork_token`** - Create a new session with the same permissions
- **`/private/logout`** - Log out and invalidate the token

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

### Basic Example
```bash
# From the deribit-http/ directory
cd examples/basic
cargo run --bin basic
```

### Authentication Endpoints Example
```bash
# From the deribit-http/ directory
cd examples/basic
cargo run --bin authentication_endpoints
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
```📄 Token type: bearer
⏰ Expires in: 900 segundos
🔑 Access token: 1755358792907.1bzKD...
🔄 Refresh token: 1755962692907.1Z7FU...
🎯 Scope: session:rest-6fLVUiTbfwM= block_trade:read_write trade:read_write...
🆔 Session ID: 62178.FdRo6Dxh.rest-6fLVUiTbfwM=

🔄 2. INTERCAMBIO DE TOKEN PARA DIFERENTE SUBJECT_ID
----------------------------------------------------
✅ Intercambio de token exitoso
🎯 Subject ID: 10
...
```

## Manejo de Errores

Los ejemplos incluyen manejo completo de errores para:

- **Credenciales faltantes o inválidas**
- **Problemas de conectividad de red**
- **Errores de la API de Deribit**
- **Tokens expirados o inválidos**
- **Permisos insuficientes**

## Logging

Los ejemplos utilizan `tracing` para logging detallado. Puedes ajustar el nivel de logging con la variable de entorno:

```bash
RUST_LOG=debug cargo run --bin authentication_endpoints
```

Niveles disponibles: `error`, `warn`, `info`, `debug`, `trace`

## Notas Importantes

1. **Testnet vs Production**: Los ejemplos están configurados para usar Deribit Testnet por defecto
2. **Rate Limiting**: El cliente incluye rate limiting automático
3. **Seguridad**: Nunca hardcodees credenciales en el código fuente
4. **Tokens**: Los access tokens tienen una duración limitada (típicamente 15 minutos)
5. **Refresh Tokens**: Los refresh tokens pueden usarse para obtener nuevos access tokens

## Troubleshooting

### Error: "Missing required environment variable"
- Verifica que el archivo `.env` existe y contiene las variables correctas
- Asegúrate de que estás ejecutando desde el directorio correcto

### Error: "OAuth2 authentication failed: bad_request"
- Verifica que tus credenciales son correctas
- Asegúrate de estar usando credenciales de Testnet para el ejemplo

### Error: "Method not found"
- Verifica que la URL base es correcta
- Asegúrate de que no hay duplicación de `/api/v2` en la URL

## Recursos Adicionales

- [Documentación de la API de Deribit](https://docs.deribit.com/)
- [Deribit Testnet](https://test.deribit.com)
- [Documentación de OAuth2](https://docs.deribit.com/#authentication)