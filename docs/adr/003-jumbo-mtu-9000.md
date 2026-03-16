# ADR-003: Jumbo MTU 9000 Profile Design

## Status

Accepted

## Date

2025-12-25

## Context

Network MTU (Maximum Transmission Unit) significantly impacts VPN throughput and latency. Standard Ethernet uses 1500 bytes, but many datacenter and enterprise networks support Jumbo Frames (9000 bytes).

EdgeRay must support both profiles to optimize for different environments:

- **Mobile/Standard**: Conservative 1500 MTU for broad compatibility
- **Datacenter/LAN**: Jumbo 9000 MTU for maximum throughput

## Decision

Implement a dynamic MTU profile system with explicit MSS clamping.

## Rationale

### MTU Profile Benefits

| Profile | MTU | MSS (IPv4) | Use Case |
|---------|-----|------------|----------|
| Cellular | 1400 | 1360 | Mobile networks with tunnel overhead |
| Standard | 1500 | 1460 | Normal ethernet, internet |
| Jumbo | 9000 | 8960 | Datacenter, high-speed LAN |

### Why Dynamic Profiles

1. **Performance**: 6x fewer packets for same data with Jumbo frames
2. **Latency**: Less per-packet overhead (headers, syscalls, encryption)
3. **Compatibility**: Graceful fallback when Jumbo not supported
4. **User Control**: Let users choose based on their environment

### MSS Clamping

TCP Maximum Segment Size must be clamped to prevent IP fragmentation:

```
MSS = MTU - IP_Header(20/40) - TCP_Header(20)
```

We intercept TCP SYN packets and modify the MSS option in-flight.

## Consequences

### Positive

- 5-6x throughput improvement in Jumbo environments
- Reduced CPU overhead (fewer packets to process)
- Lower encryption overhead per byte
- User-selectable optimization

### Negative

- Jumbo frames may not work across all networks
- Path MTU Discovery complexity
- Buffer sizing must scale with MTU
- Some protocols assume 1500 MTU

## Implementation

### MtuProfile Enum

Located in `rustray/src/tun/tun_device.rs`:

```rust
#[derive(Debug, Clone, Copy)]
pub enum MtuProfile {
    Cellular,  // 1400 MTU
    Standard,  // 1500 MTU  
    Jumbo,     // 9000 MTU
    Custom(u16),
}

impl MtuProfile {
    pub fn mtu(&self) -> u16 {
        match self {
            MtuProfile::Cellular => 1400,
            MtuProfile::Standard => 1500,
            MtuProfile::Jumbo => 9000,
            MtuProfile::Custom(mtu) => *mtu,
        }
    }
    
    pub fn mss_ipv4(&self) -> u16 {
        self.mtu() - 40 // IP(20) + TCP(20)
    }
}
```

### MSS Clamping

Located in `rustray/src/tun/tun2socks.rs`:

```rust
pub fn clamp_mss(packet: &mut [u8], mtu_profile: MtuProfile) -> bool {
    // Parse TCP SYN packet
    // Find MSS option
    // Clamp if > profile.mss_ipv4()
    // Recalculate TCP checksum
}
```

### Buffer Sizing

All packet buffers are sized based on MTU profile:

```rust
impl MtuProfile {
    pub fn buffer_size(&self) -> usize {
        self.mtu() as usize + 64 // Safety margin
    }
}
```

## Testing

1. **Local Test**: Inject 9000-byte buffer, verify reassembly
2. **Benchmark**: Measure throughput at each MTU profile
3. **Fragmentation Test**: Verify no IP fragmentation occurs
