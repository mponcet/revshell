# RevShell

A reverse shell tool written in Rust that leverages UPnP for port forwarding. Useful for CTF (Capture The Flag) challenges and penetration testing scenarios.

## Features

- Automatic UPnP port forwarding
- TCP and UDP server support for reverse shell connections
- Configurable host and port mapping
- Cleanup of port mapping on exit

## Usage

```bash
revshell -H <host_ip> -p <external_port>:<internal_port> [--protocol tcp|udp]
```

### Arguments

- `-H, --host`: The local IP address to bind the server to
- `-p, --port`: Port mapping in format `external_port:internal_port`
- `--protocol`: Protocol to use (tcp or udp, default: tcp)

### Examples

**TCP Server (default):**
```bash
revshell -H 192.168.1.100 -p 8080:4444
```

**UDP Server:**
```bash
revshell -H 192.168.1.100 -p 8080:4444 --protocol udp
```

This will:
1. Forward external port 8080 to internal port 4444 using UPnP
2. Start a TCP/UDP server listening on 192.168.1.100:4444
3. Display your external IP address
4. Clean up the port mapping when the program exits

For reverse shell examples and payloads, see [revshells.com](https://revshells.com).

## Install

```bash
cargo install --path .
```

## License

MIT License - see [LICENSE](LICENSE) file for details
