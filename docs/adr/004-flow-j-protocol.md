# ADR-004: Flow-J Protocol Design

## Status

Accepted

## Date

2025-12-25

## Context

Flow-J is EdgeRay's advanced traffic obfuscation protocol designed to evade sophisticated deep packet inspection (DPI) and machine learning-based traffic analysis. It extends VLESS with additional stealth features.

## Decision

Implement Flow-J as a modular flow engine with pluggable transports and obfuscation layers.

## Rationale

### Design Goals

1. **ML Resistance**: Defeat traffic fingerprinting via probabilistic shaping
2. **CDN Traversal**: Work through Cloudflare, AWS CloudFront, etc.
3. **Protocol Camouflage**: Mimic legitimate HTTPS/HTTP3 traffic patterns
4. **Forward Error Correction**: Handle lossy networks gracefully
5. **Adaptive Behavior**: Adjust parameters based on network conditions

### Flow-J Components

```
┌─────────────────────────────────────────────────┐
│                   Flow-J Engine                  │
├─────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐            │
│  │ Probabilistic│  │   Markov    │            │
│  │   Shaper     │  │   Jitter    │            │
│  └──────────────┘  └──────────────┘            │
├─────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐            │
│  │   Padding    │  │    FEC      │            │
│  │   Manager    │  │   Encoder   │            │
│  └──────────────┘  └──────────────┘            │
├─────────────────────────────────────────────────┤
│  Transport Modes: TCP | QUIC | SplitHTTP | MQTT │
└─────────────────────────────────────────────────┘
```

## Consequences

### Positive

- Defeats ML-based traffic classification
- Works through CDNs and enterprise proxies
- Graceful degradation on lossy networks
- Configurable performance/stealth tradeoff

### Negative

- Higher latency due to padding and jitter
- Bandwidth overhead from FEC redundancy
- Complex configuration surface

## Implementation

### Core Flow Engine

Located in `rustray/src/protocols/flow_j.rs`:

```rust
pub struct FlowJOutbound {
    settings: FlowJSettings,
    dns_server: Arc<DnsServer>,
    stats: Arc<StatsManager>,
    shaper: ProbabilisticShaper,
    jitter: MarkovJitter,
    fec_encoder: Option<ReedSolomonEncoder>,
}
```

### Transport Modes

| Mode | File | Use Case |
|------|------|----------|
| REALITY | `flow_j_reality.rs` | Direct TLS connection |
| CDN | `flow_j_cdn.rs` | SplitHTTP through CDN |
| MQTT | `flow_j_mqtt.rs` | IoT protocol mimicry |
| FEC | `flow_j_fec.rs` | Forward Error Correction |

### Configuration

```json
{
  "flow": "j",
  "flowSettings": {
    "padding": 0.3,
    "jitter": { "min": 10, "max": 50 },
    "fec": { "dataShards": 10, "parityShards": 3 },
    "transport": "xhttp"
  }
}
```

## Link Format Extension

Flow-J extends standard VLESS links:

```
vless://uuid@host:443?flow=j&fec=1&padding=0.3&transport=xhttp#name
```

| Parameter | Description |
|-----------|-------------|
| `flow=j` | Enable Flow-J engine |
| `fec=1` | Enable Forward Error Correction |
| `padding=0.3` | 30% padding ratio |
| `transport=xhttp` | SplitHTTP transport mode |
