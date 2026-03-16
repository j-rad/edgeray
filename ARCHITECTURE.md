# EdgeRay Architecture

> High-Performance Cross-Platform VPN Client with Rust Core

## Overview

EdgeRay is a modern VPN client implementing multiple proxy protocols with a focus on performance, security, and cross-platform compatibility. The architecture follows a layered design with a pure-Rust core that abstracts platform-specific concerns.

## System Context

```mermaid
graph TD
    subgraph "User Interface"
        WEB[Dioxus Web/Desktop UI]
        MOBILE[Mobile UI - Flutter/Native]
    end
    
    subgraph "Application Layer"
        TAURI[Tauri Runtime]
        UNIFFI[UniFFI Bindings]
    end
    
    subgraph "Core Engine - rustray"
        ROUTER[Router - Geo/Domain Matching]
        PROTOCOLS[Protocol Suite]
        TRANSPORT[Transport Layer]
        VPN[VPN Stack - tun2socks]
    end
    
    subgraph "Platform Layer"
        TUN[tun-rs - TUN Device]
        SMOLTCP[smoltcp - TCP/IP Stack]
        OS[OS Network Stack]
    end
    
    WEB --> TAURI
    MOBILE --> UNIFFI
    TAURI --> ROUTER
    UNIFFI --> ROUTER
    ROUTER --> PROTOCOLS
    PROTOCOLS --> TRANSPORT
    VPN --> TUN
    TUN --> OS
    VPN --> SMOLTCP
```

## Core Components

### rustray (Rust Core Library)

The core library provides all proxy functionality in pure Rust:

| Component | Location | Purpose |
|-----------|----------|---------|
| Protocols | `src/protocols/` | VMess, VLESS, Trojan, Hysteria2, WireGuard, Shadowsocks |
| Transport | `src/transport/` | TLS, REALITY, ECH, WebSocket, gRPC, QUIC, SplitHTTP |
| Router | `src/router.rs` | GeoIP/GeoSite matching, domain-based routing |
| VPN Stack | `src/tun/` | tun-rs device, smoltcp TCP/IP stack, Tun2Socks engine |
| API | `src/api/` | gRPC service for control plane |

### Protocol Suite

```mermaid
graph LR
    subgraph "Proxy Protocols"
        VMESS[VMess AEAD]
        VLESS[VLESS + Vision]
        TROJAN[Trojan]
        HY2[Hysteria2 BBR]
        WG[WireGuard]
        SS[Shadowsocks 2022]
    end
    
    subgraph "Transport Security"
        TLS[TLS 1.3]
        REALITY[REALITY]
        ECH[ECH]
        PQC[PQC - ML-KEM]
    end
    
    subgraph "Stream Transports"
        TCP[TCP]
        WS[WebSocket]
        GRPC[gRPC]
        QUIC[QUIC/H3]
        XHTTP[SplitHTTP/xhttp]
    end
    
    VMESS --> TLS
    VLESS --> REALITY
    VLESS --> TLS
    TROJAN --> TLS
    HY2 --> QUIC
    
    TLS --> TCP
    TLS --> WS
    REALITY --> TCP
    ECH --> TLS
```

## Packet Lifecycle

The complete path of a packet through the VPN stack:

```mermaid
sequenceDiagram
    participant App as Application
    participant OS as OS Network Stack
    participant TUN as TUN Device (tun-rs)
    participant SMOL as smoltcp TCP/IP
    participant T2S as Tun2Socks Engine
    participant ROUTER as Router
    participant PROTO as Protocol Handler
    participant REMOTE as Remote Server

    App->>OS: TCP/UDP Socket Call
    OS->>TUN: IP Packet (via routing table)
    TUN->>SMOL: Raw IP Packet
    
    Note over SMOL: Userspace TCP/IP Processing
    SMOL->>T2S: TCP Stream / UDP Datagram
    
    T2S->>ROUTER: Extract destination (host:port)
    ROUTER->>ROUTER: GeoIP/GeoSite Matching
    
    alt Direct Connection
        ROUTER->>OS: Bypass to OS Stack
    else Proxy Route
        ROUTER->>PROTO: Select Outbound
        PROTO->>PROTO: Protocol Encapsulation
        PROTO->>REMOTE: Encrypted Tunnel
    end
    
    REMOTE-->>PROTO: Response
    PROTO-->>T2S: Decapsulated Data
    T2S-->>SMOL: TCP ACK / UDP Response
    SMOL-->>TUN: IP Packet
    TUN-->>OS: Inject to Network
    OS-->>App: Socket Response
```

## VLESS REALITY Handshake

Detailed handshake sequence for VLESS with REALITY transport:

```mermaid
sequenceDiagram
    participant C as Client
    participant S as REALITY Server
    participant D as Decoy Server

    Note over C: Generate X25519 Ephemeral Keypair
    Note over C: Embed public key in Client Hello
    
    C->>S: TLS ClientHello (mimics browser fingerprint)
    
    Note over S: Extract ephemeral public key
    Note over S: Validate Short ID
    
    alt Valid REALITY Auth
        Note over S: Derive shared secret
        S->>C: TLS ServerHello (with REALITY params)
        Note over C,S: ECDH Key Exchange Complete
        
        C->>S: Encrypted VLESS Request
        S->>C: Encrypted VLESS Response
        
        Note over C,S: Bi-directional encrypted tunnel
    else Invalid Auth
        Note over S: Forward to decoy
        S->>D: Proxy ClientHello
        D->>S: Real TLS Response
        S->>C: Decoy Response
        Note over C: Connection appears as normal HTTPS
    end
```

## MTU Profiles

The VPN stack supports multiple MTU profiles for different network environments:

| Profile | MTU | MSS (IPv4) | MSS (IPv6) | Use Case |
|---------|-----|------------|------------|----------|
| Cellular | 1400 | 1360 | 1340 | Mobile networks with overhead |
| Standard | 1500 | 1460 | 1440 | Normal ethernet |
| Jumbo | 9000 | 8960 | 8940 | High-performance LAN/DC |

### MSS Clamping

TCP MSS is dynamically clamped in SYN packets to prevent fragmentation:

```
MSS = MTU - IP_Header - TCP_Header
IPv4: MSS = MTU - 20 - 20
IPv6: MSS = MTU - 40 - 20
```

## Kill-Switch Implementation

Zero-leak protection is implemented via atomic flag:

```mermaid
stateDiagram-v2
    [*] --> Healthy: Core Started
    Healthy --> Unhealthy: Error/Disconnect
    Unhealthy --> Healthy: Reconnected
    
    state Healthy {
        [*] --> Processing
        Processing --> Processing: Forward Packets
    }
    
    state Unhealthy {
        [*] --> Dropping
        Dropping --> Dropping: Drop All Packets
    }
```

When `CORE_HEALTHY` is `false`:

- All outgoing packets are silently dropped
- No traffic leaks to the direct network
- System routes remain in place
- Reconnection attempts continue

## Cross-Platform Support

| Platform | UI Framework | Core Binding | TUN Implementation |
|----------|--------------|--------------|-------------------|
| Linux | Dioxus (Tauri) | Native | tun-rs |
| macOS | Dioxus (Tauri) | Native | tun-rs (utun) |
| Windows | Dioxus (Tauri) | Native | tun-rs (Wintun) |
| Android | Kotlin/Compose | UniFFI | VpnService + tun |
| iOS | Swift/SwiftUI | UniFFI | NetworkExtension |

## Directory Structure

```
edgeray-workspace/
├── rustray/                    # Core Rust library
│   ├── src/
│   │   ├── protocols/          # VMess, VLESS, Trojan, etc.
│   │   ├── transport/          # TLS, REALITY, WebSocket, etc.
│   │   ├── tun/                # TUN device and Tun2Socks
│   │   ├── router.rs           # Routing engine
│   │   └── api/                # gRPC control plane
│   └── benches/                # Performance benchmarks
├── edgeray-app/                # Desktop/Web UI (Dioxus + Tauri)
│   ├── src/                    # Frontend components
│   └── src-tauri/              # Tauri backend
├── shared-types/               # Shared type definitions
│   └── src/
│       ├── lib.rs              # Protocol, ServerConfig
│       └── parser.rs           # Link parsing/generation
└── docs/
    └── adr/                    # Architecture Decision Records
```

## Performance Characteristics

| Metric | Target | Implementation |
|--------|--------|----------------|
| Packet Latency | < 0.5ms | Lock-free queues (crossbeam) |
| Memory Allocation | Zero-copy | `bytes` crate, buffer pools |
| Crypto | Hardware-accelerated | AES-NI, ARM NEON via ring |
| Binary Size | < 15MB (mobile) | LTO, symbol stripping |

## Security Model

1. **Memory Safety**: Pure Rust core, no unsafe outside FFI boundaries
2. **Traffic Obfuscation**: REALITY, ECH, TLS fingerprinting
3. **Kill-Switch**: Atomic flag prevents leak during disconnect
4. **Replay Protection**: Per-protocol LRU nonce caches
5. **Post-Quantum Ready**: ML-KEM-768 hybrid key exchange support
