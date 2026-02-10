# AGENTS.md - RevShell Codebase Guide

## Project Overview

RevShell is a Rust-based reverse shell tool that leverages UPnP for automatic port forwarding. It's designed for CTF challenges and penetration testing scenarios, supporting both TCP and UDP protocols.

## Essential Commands

### Build
```bash
cargo build
```

### Run
```bash
cargo run -- -H <host_ip> -p <external_port>:<internal_port> [--protocol tcp|udp]
```

### Install
```bash
cargo install --path .
```

### Test
```bash
cargo test
```

## Code Organization

```
src/
├── main.rs          # Entry point, CLI parsing, main logic
├── server.rs        # Server trait and protocol handling
├── tcp.rs           # TCP server implementation
├── udp.rs           # UDP server implementation
├── igd.rs           # UPnP/IGD gateway functionality
└── nonblocking_stdin.rs  # Non-blocking stdin handling
```

## Key Components

### Main Logic Flow (main.rs)
1. Parse CLI arguments using clap
2. Search for UPnP gateway
3. Add port mapping via UPnP
4. Get external IP address
5. Start server (TCP/UDP)
6. Clean up port mapping on exit

### Server Architecture
- `Protocol` enum: `Tcp` | `Udp`
- `Server` trait: Defines `run()` and `shutdown()` methods
- Protocol-specific implementations: `TcpServer` and `UdpServer`

### UPnP/IGD Gateway (igd.rs)
- `GatewayExt`: Wrapper around `igd_next::Gateway`
- Handles port mapping creation and cleanup
- Tracks active port mappings with `PortMapping` struct

### Non-blocking STDIN
- Uses dedicated thread for blocking I/O
- Communicates via MPSC channel
- Enables interactive shell while maintaining async server

## Naming Conventions

- **Types**: PascalCase (`TcpServer`, `UdpServer`, `GatewayExt`)
- **Functions**: snake_case (`parse_host`, `add_port`, `get_external_ip`)
- **Variables**: snake_case (`external_port`, `internal_port`, `bind_addr`)
- **Traits**: PascalCase (`Server`)
- **Enums**: PascalCase (`Protocol`)

## Error Handling

- Uses `anyhow` crate for error handling
- `Result` type used throughout
- Errors propagated with `?` operator
- Custom error messages with `anyhow!()` and `bail!()` macros

## Async Patterns

- Uses Tokio runtime (`#[tokio::main]`)
- `async_trait` for async trait methods
- `tokio::select!` for concurrent I/O operations
- `Arc<Notify>` for shutdown coordination
- `BufWriter` for buffered TCP streams

## Testing Approach

- No explicit test files found in current codebase
- Testing would likely involve:
  - Mocking UPnP gateway for IGD functionality
  - Testing server connection handling
  - Verifying port mapping lifecycle

## Important Gotchas

1. **UPnP Dependency**: Requires UPnP-enabled router for port forwarding
2. **Port Mapping Cleanup**: Automatic cleanup on exit via `cleanup()` method
3. **Single Connection Handling**: TCP server handles one connection at a time (no concurrent connections)
4. **UDP Client Tracking**: UDP server tracks first client that connects
5. **Non-blocking STDIN**: Uses dedicated thread to avoid blocking Tokio runtime

## Build Configuration

- Rust 2024 edition
- Key dependencies:
  - `tokio` (full features) - async runtime
  - `igd-next` - UPnP/IGD functionality
  - `clap` - CLI argument parsing
  - `anyhow` - error handling
  - `tracing` - logging
  - `async-trait` - async traits

## Logging

- Uses `tracing` crate
- Log level: INFO (set in `main.rs:53`)
- Log messages for:
  - New connections
  - Connection terminations
  - External IP display
  - Port mapping cleanup

## CLI Structure

```rust
#[derive(Parser, Debug)]
struct Args {
    host: IpAddr,           // -H, --host
    port: (u16, u16),       // -p, --port (external:internal)
    protocol: Protocol,     // --protocol (tcp|udp, default: tcp)
}
```

## Protocol Handling

- TCP: Uses `TcpListener` and handles connections sequentially
- UDP: Uses `UdpSocket` and tracks single client address
- Both protocols support bidirectional communication between stdin/stdout and network

## Shutdown Mechanism

- Ctrl+C handling via `tokio::signal::ctrl_c()`
- Graceful shutdown through `Server::shutdown()` method
- Uses `Arc<Notify>` for coordinated shutdown across connections
