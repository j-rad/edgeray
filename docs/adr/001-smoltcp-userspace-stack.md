# ADR-001: Selection of smoltcp Userspace TCP/IP Stack

## Status

Accepted

## Date

2025-12-25

## Context

EdgeRay requires a userspace TCP/IP stack to convert raw IP packets from the TUN device into logical TCP streams and UDP datagrams that can be routed through proxy protocols. We evaluated several options:

1. **Kernel Stack + Raw Sockets**: Use the OS kernel's TCP/IP stack
2. **lwIP (via FFI)**: Popular embedded TCP/IP stack in C
3. **smoltcp**: Pure Rust userspace TCP/IP stack
4. **Custom Implementation**: Build our own minimal stack

## Decision

We chose **smoltcp** as our userspace TCP/IP stack.

## Rationale

### Why smoltcp

1. **Pure Rust**: Memory safety guarantees, no FFI overhead or C dependency management
2. **no_std Compatible**: Can run in constrained environments (embedded, WASM future)
3. **Feature-Complete**: Supports TCP, UDP, ICMP, IPv4, IPv6
4. **Async Support**: Native `async` feature integrates cleanly with Tokio
5. **Well-Maintained**: Active development, good documentation
6. **Proven**: Used in production by multiple projects

### Why Not Alternatives

| Option | Rejection Reason |
|--------|------------------|
| Kernel Stack | Cannot intercept before routing; loses traffic visibility |
| lwIP | C dependency, unsafe FFI, harder to maintain |
| Custom | Time-intensive, error-prone for complex protocols |

## Consequences

### Positive

- Zero-copy packet processing with `bytes` crate integration
- Full control over TCP state machine for protocol-specific optimizations
- Consistent behavior across all platforms (Linux, macOS, Windows, Android, iOS)
- Memory usage bounded by explicit socket limits

### Negative

- Slightly higher CPU usage vs kernel stack (userspace context switches)
- Must implement our own connection tracking
- Some edge cases may differ from kernel behavior

## Implementation

Located in `rustray/src/tun/tun2socks.rs`:

```rust
use smoltcp::iface::{Interface, SocketSet};
use smoltcp::socket::tcp::Socket as TcpSocket;
use smoltcp::socket::udp::Socket as UdpSocket;
```

The `Tun2SocksEngine` bridges tun-rs device to smoltcp interface.
