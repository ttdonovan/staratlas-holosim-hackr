# hackr-ixproc

A lightweight Solana program activity monitor with real-time transaction and account change tracking.

## Features

- **🚀 Dual Subscription Monitoring**:
  - **📋 Transaction Logs**: Monitors program invocations and transaction logs
  - **💰 Account Changes**: Tracks updates to program-owned accounts
- **⚡ Real-time Processing**: Uses Solana's native pubsub client with built-in reconnection
- **🎯 Multi-program Support**: Monitor multiple Solana programs simultaneously
- **📊 Live Statistics**: HTTP API for health checks and processing stats
- **🐞 Rich Logging**: Structured logging with emojis and detailed context
- **⚙️ Zero Database**: Lightweight in-memory processing (no persistence layer)

## Quick Start

1. **Copy environment configuration**:
   ```bash
   cp .env.example .env
   ```

2. **Configure programs to monitor** (edit `.env`):
   ```bash
   # Star Atlas programs (example)
   PROGRAMS=SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF,PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ

   # Solana RPC endpoints
   RPC_URL=https://rpc.ironforge.network/devnet?apiKey=01JEB7YQ0YPK31WQTC0VQ5Y9YP
   RPC_WS_URL=wss://rpc.ironforge.network/devnet?apiKey=01JEB7YQ0YPK31WQTC0VQ5Y9YP
   ```

3. **Run the processor**:
   ```bash
   cargo run -p hackr-ixproc
   ```

## What You'll See

### Transaction Logs (📋)
```
📥 Received logs notification - fetching transaction details
🎉 Processing transaction for monitored programs
🚀 Found interaction with tracked program!
```

### Account Changes (💰)
```
💰 Account updated for monitored program
account_pubkey=ABC123... program_id=DEF456... lamports=1000000
```

### Statistics (📊)
```
📊 Updated stats for program
program_id=SAgeTraQ... new_count=42
```

## Configuration

Environment variables (see `.env.example`):

- `RPC_URL` - Solana RPC HTTP endpoint (for transaction fetching)
- `RPC_WS_URL` - Solana RPC WebSocket endpoint (for real-time subscriptions)
- `PROGRAMS` - Comma-separated list of program IDs to monitor
- `RUST_LOG` - Logging level (`hackr_ixproc=debug,info` for verbose output)

## HTTP API

- **GET /health** - Health check endpoint
- **GET /stats** - Current processing statistics in JSON format

```bash
# Health check
curl http://localhost:8080/health

# Get processing stats
curl http://localhost:8080/stats
```

## Monitored Program Types

### Star Atlas Programs
- **Holosim (SAGE)**: `SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF`
- **Player Profile**: `PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ`
- **Profile Faction**: `pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj`
- **C4 Sage**: `C4SAgeKLgb3pTLWhVr6NRwWyYFuTR7ZeSXFrzoLwfMzF`

### Any Solana Program
You can monitor any Solana program by adding its program ID to the `PROGRAMS` environment variable.

## Architecture

### Core Components
- **`config.rs`** - Environment-based configuration management
- **`processor.rs`** - Transaction and event processing logic
- **`solana_monitor.rs`** - Dual pubsub subscriptions (logs + accounts)
- **`main.rs`** - HTTP server and application orchestration

### Data Flow
```
Solana Network
    ↓
PubSub Client (2 streams per program)
    ├─ Transaction Logs → handle_logs_notification()
    └─ Account Changes → handle_account_notification()
    ↓
LiteProcessor → Statistics & Logging
    ↓
HTTP API (/stats endpoint)
```

## Advanced Usage

### Debug Logging
Enable detailed logging to see all WebSocket messages and transaction details:
```bash
RUST_LOG=hackr_ixproc=debug cargo run -p hackr-ixproc
```

### Multiple Networks
Create different `.env` files for different networks:
```bash
# .env.devnet
RPC_URL=https://api.devnet.solana.com
RPC_WS_URL=wss://api.devnet.solana.com

# .env.mainnet
RPC_URL=https://api.mainnet-beta.solana.com
RPC_WS_URL=wss://api.mainnet-beta.solana.com
```

### Custom Program Monitoring
Monitor your own programs by adding their IDs:
```bash
PROGRAMS=YourProgram1...,YourProgram2...,YourProgram3...
```

## Troubleshooting

### No Activity Logs
- **Check network**: Ensure you're monitoring the right network (devnet vs mainnet)
- **Verify program IDs**: Confirm programs are active on your chosen network
- **Check RPC endpoints**: Verify WebSocket URL is accessible and has API quota

### Connection Issues
- **API limits**: Some RPC providers have connection limits
- **Firewall**: Ensure WebSocket connections (WSS) are allowed
- **API keys**: Verify your RPC provider API key is valid

### High CPU Usage
- **Reduce programs**: Monitor fewer programs simultaneously
- **Adjust logging**: Use `info` level instead of `debug`
- **Account filters**: The current version monitors ALL accounts for each program

## Performance Notes

- Each program gets **2 WebSocket connections** (logs + accounts)
- Account subscriptions can be very active for popular programs
- Memory usage scales with the number of monitored programs
- No persistence layer - all data is in-memory only
