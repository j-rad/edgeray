# ADR-002: Selection of tun-rs for Cross-Platform TUN Device

## Status

Accepted

## Date

2025-12-25

## Context

EdgeRay needs cross-platform TUN (network tunnel) device support to intercept and inject IP packets. The TUN device is the bridge between the OS network stack and our userspace processing.

Options evaluated:

1. **Platform-specific implementations**: Write separate code for each OS
2. **tun (crate)**: Older Rust TUN library
3. **tun-rs**: Modern async-first Rust TUN library
4. **tokio-tun**: Tokio-specific TUN wrapper

## Decision

We chose **tun-rs** as our TUN device library.

## Rationale

### Why tun-rs

1. **Cross-Platform**: Single API for Linux, macOS, Windows, Android, iOS
2. **Async-Native**: Built with `async` support from the ground up
3. **Feature-Rich**: MTU configuration, IPv4/IPv6, platform-specific optimizations
4. **Active Maintenance**: Regular updates, responsive maintainers
5. **Modern API**: Ergonomic Rust interface with proper error handling

### Platform Support Matrix

| Platform | Backend | Status |
|----------|---------|--------|
| Linux | /dev/net/tun | ✅ Full support |
| macOS | utun | ✅ Full support |
| Windows | Wintun | ✅ Full support |
| Android | VpnService FD | ✅ Via file descriptor |
| iOS | NetworkExtension | ✅ Via file descriptor |

### Why Not Alternatives

| Option | Rejection Reason |
|--------|------------------|
| Platform-specific | Maintenance burden, code duplication |
| tun (crate) | Less active, older API design |
| tokio-tun | Less cross-platform coverage |

## Consequences

### Positive

- Single codebase for all platforms
- Async integration with Tokio runtime
- Dynamic MTU adjustment at runtime
- File descriptor passing for mobile platforms

### Negative

- Platform quirks still require conditional handling
- Windows requires Wintun driver installation
- Mobile platforms need additional VPN service setup

## Implementation

Located in `rustray/src/tun/tun_device.rs`:

```rust
use tun_rs::AsyncDevice;

pub struct TunDevice {
    device: AsyncDevice,
    config: TunConfig,
    name: String,
}

impl TunDevice {
    pub async fn create(config: TunConfig) -> anyhow::Result<Self> {
        let mut tun_config = tun_rs::Configuration::default();
        tun_config.mtu(config.mtu() as i32);
        // ... platform-specific configuration
    }
}
```

Mobile platforms pass pre-created file descriptors from VpnService/NetworkExtension.
