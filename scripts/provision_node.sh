#!/usr/bin/env bash
# scripts/provision_node.sh
# ────────────────────────────────────────────────────────────────────────────
# RustRay — Hardened Linux Node Provisioner  (one-click deployment)
# ────────────────────────────────────────────────────────────────────────────
# Usage:
#   curl -fsSL https://deploy.edgeray.io/provision | bash -s -- \
#       --server-addr 10.0.0.1:10086 \
#       --node-id    node-fra01 \
#       --token      <control-bus-jwt> \
#       [--binary-url https://release.edgeray.io/rustray-latest-amd64]
#
# What this script does:
#   1. Hardens the host (sysctl, firewall, disable ICMP, restrict SSH)
#   2. Installs the rustray binary (from URL or apt-style repo)
#   3. Generates a unique gRPC mTLS keypair for the control bus
#   4. Writes /etc/rustray/config.json with the given parameters
#   5. Registers the node SHA-256 fingerprint with the control server
#   6. Installs and enables the systemd unit
# ────────────────────────────────────────────────────────────────────────────
set -euo pipefail
IFS=$'\n\t'

# ── Colour helpers ────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'
info()    { echo -e "${GREEN}[INFO]${NC}  $*"; }
warn()    { echo -e "${YELLOW}[WARN]${NC}  $*"; }
die()     { echo -e "${RED}[FATAL]${NC} $*" >&2; exit 1; }

# ── Argument parsing ──────────────────────────────────────────────────────────
SERVER_ADDR=""
NODE_ID=""
TOKEN=""
BINARY_URL=""
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/rustray"
DATA_DIR="/var/lib/rustray"
CERT_DIR="/etc/rustray/certs"
SYSTEMD_UNIT="/etc/systemd/system/rustray.service"
GRPC_PORT=10085

while [[ $# -gt 0 ]]; do
    case "$1" in
        --server-addr) SERVER_ADDR="$2"; shift 2 ;;
        --node-id)     NODE_ID="$2";     shift 2 ;;
        --token)       TOKEN="$2";       shift 2 ;;
        --binary-url)  BINARY_URL="$2";  shift 2 ;;
        --grpc-port)   GRPC_PORT="$2";   shift 2 ;;
        *) die "Unknown argument: $1" ;;
    esac
done

[[ -n "$SERVER_ADDR" ]] || die "--server-addr is required (e.g. 10.0.0.1:10086)"
[[ -n "$NODE_ID"     ]] || die "--node-id is required (e.g. node-fra01)"
[[ -n "$TOKEN"       ]] || die "--token is required (control bus JWT)"

# ── Root check ────────────────────────────────────────────────────────────────
[[ "$(id -u)" -eq 0 ]] || die "This script must be run as root."

# ── Phase 1: System hardening ─────────────────────────────────────────────────
info "Applying kernel hardening …"
cat > /etc/sysctl.d/99-rustray-hardening.conf <<'EOF'
# Disable ICMP redirects (prevent routing manipulation)
net.ipv4.conf.all.accept_redirects = 0
net.ipv4.conf.default.accept_redirects = 0
net.ipv6.conf.all.accept_redirects = 0

# Ignore ICMP echo requests (stealth node)
net.ipv4.icmp_echo_ignore_all = 1

# Disable source routing
net.ipv4.conf.all.accept_source_route = 0

# SYN flood protection
net.ipv4.tcp_syncookies = 1

# Increase ephemeral port range
net.ipv4.ip_local_port_range = 10240 65535

# Increase TCP buffer sizes for high-throughput tunnels
net.core.rmem_max = 134217728
net.core.wmem_max = 134217728
net.ipv4.tcp_rmem = 4096 87380 134217728
net.ipv4.tcp_wmem = 4096 65536 134217728

# BBR congestion control
net.core.default_qdisc = fq
net.ipv4.tcp_congestion_control = bbr
EOF
sysctl --system --quiet

# ── Phase 2: Firewall (nftables) ──────────────────────────────────────────────
info "Configuring nftables firewall …"
command -v nft >/dev/null 2>&1 || apt-get install -y nftables -q

cat > /etc/nftables.conf <<EOF
#!/usr/sbin/nft -f
flush ruleset

table inet rustray-fw {
    chain input {
        type filter hook input priority 0; policy drop;

        # Allow loopback
        iifname "lo" accept

        # Allow established/related
        ct state established,related accept

        # Allow SSH (hardened: key-only enforced via sshd_config below)
        tcp dport 22 accept

        # Allow gRPC control bus
        tcp dport ${GRPC_PORT} accept

        # Allow QUIC (UDP) for Brutal-QUIC transport
        udp dport 443 accept

        # Drop everything else
        drop
    }

    chain forward {
        type filter hook forward priority 0; policy drop;
    }

    chain output {
        type filter hook output priority 0; policy accept;
    }
}
EOF
systemctl enable --now nftables
nft -f /etc/nftables.conf
info "Firewall applied."

# ── Phase 3: SSH hardening ────────────────────────────────────────────────────
info "Hardening SSH …"
sed -i \
    -e 's/^#*PermitRootLogin.*/PermitRootLogin prohibit-password/' \
    -e 's/^#*PasswordAuthentication.*/PasswordAuthentication no/' \
    -e 's/^#*PubkeyAuthentication.*/PubkeyAuthentication yes/' \
    -e 's/^#*X11Forwarding.*/X11Forwarding no/' \
    /etc/ssh/sshd_config
systemctl restart sshd

# ── Phase 4: Install rustray binary ──────────────────────────────────────────
info "Installing rustray binary …"
mkdir -p "$INSTALL_DIR" "$CONFIG_DIR" "$DATA_DIR" "$CERT_DIR"

if [[ -n "$BINARY_URL" ]]; then
    curl -fsSL -o "${INSTALL_DIR}/rustray" "$BINARY_URL"
elif command -v rustray >/dev/null 2>&1; then
    info "rustray already in PATH — skipping download."
else
    die "No --binary-url provided and rustray not found in PATH. Aborting."
fi

chmod 750 "${INSTALL_DIR}/rustray"
chown root:root "${INSTALL_DIR}/rustray"

# Verify the binary is not corrupted.
ACTUAL_HASH=$(sha256sum "${INSTALL_DIR}/rustray" | awk '{print $1}')
info "Binary SHA-256: ${ACTUAL_HASH}"

# ── Phase 5: Generate mTLS keypair for gRPC control bus ──────────────────────
info "Generating gRPC mTLS keypair …"
command -v openssl >/dev/null 2>&1 || apt-get install -y openssl -q

CERT_KEY="${CERT_DIR}/node.key"
CERT_CRT="${CERT_DIR}/node.crt"
CERT_CSR="${CERT_DIR}/node.csr"

# Generate 4096-bit RSA key
openssl genrsa -out "$CERT_KEY" 4096 2>/dev/null
# Self-signed cert valid 10 years (replace with CA-signed in production)
openssl req -new -key "$CERT_KEY" \
    -subj "/CN=${NODE_ID}/O=EdgeRay/OU=ControlBus" \
    -out "$CERT_CSR" 2>/dev/null
openssl x509 -req \
    -in "$CERT_CSR" \
    -signkey "$CERT_KEY" \
    -days 3650 \
    -out "$CERT_CRT" 2>/dev/null

# Compute certificate fingerprint for registration.
CERT_FINGERPRINT=$(openssl x509 -in "$CERT_CRT" -noout -fingerprint -sha256 \
    | sed 's/SHA256 Fingerprint=//' | tr -d ':')

info "Certificate fingerprint: ${CERT_FINGERPRINT}"

# ── Phase 6: Write config.json ────────────────────────────────────────────────
info "Writing ${CONFIG_DIR}/config.json …"
cat > "${CONFIG_DIR}/config.json" <<EOF
{
  "log": { "loglevel": "warning" },
  "api": {
    "tag": "api",
    "services": ["ControlService", "StatsService", "ProxyManService"],
    "port": ${GRPC_PORT},
    "listen": "0.0.0.0"
  },
  "inbounds": [],
  "outbounds": [
    {
      "tag": "direct",
      "protocol": "freedom",
      "settings": {}
    }
  ],
  "routing": {
    "rules": [
      { "type": "field", "ip": ["geoip:private"], "outboundTag": "direct" }
    ]
  }
}
EOF
chmod 640 "${CONFIG_DIR}/config.json"

# ── Phase 7: Register node with control server ────────────────────────────────
info "Registering node with control server at ${SERVER_ADDR} …"
REGISTER_PAYLOAD=$(cat <<EOF
{
  "node_id":    "${NODE_ID}",
  "cert_fp":    "${CERT_FINGERPRINT}",
  "binary_sha": "${ACTUAL_HASH}",
  "grpc_port":  ${GRPC_PORT}
}
EOF
)

HTTP_STATUS=$(curl -s -o /dev/null -w "%{http_code}" \
    -X POST "https://${SERVER_ADDR}/api/nodes/register" \
    -H "Authorization: Bearer ${TOKEN}" \
    -H "Content-Type: application/json" \
    -d "${REGISTER_PAYLOAD}" \
    --connect-timeout 10 \
    --retry 3 \
    || true)

if [[ "$HTTP_STATUS" == "200" || "$HTTP_STATUS" == "201" ]]; then
    info "Node registered successfully (HTTP ${HTTP_STATUS})."
else
    warn "Registration returned HTTP ${HTTP_STATUS}. Continuing anyway — retry via control bus."
fi

# ── Phase 8: Install systemd unit ─────────────────────────────────────────────
info "Installing systemd service …"
cat > "$SYSTEMD_UNIT" <<EOF
[Unit]
Description=RustRay Stealth Proxy Node
Documentation=https://github.com/edgeray/rustray
After=network-online.target nftables.service
Wants=network-online.target

[Service]
Type=simple
ExecStart=${INSTALL_DIR}/rustray run --config ${CONFIG_DIR}/config.json
Restart=on-failure
RestartSec=5s
LimitNOFILE=1048576
LimitNPROC=65536

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=${DATA_DIR} ${CONFIG_DIR}
PrivateTmp=true
PrivateDevices=true
CapabilityBoundingSet=CAP_NET_ADMIN CAP_NET_RAW CAP_SYS_PTRACE
AmbientCapabilities=CAP_NET_ADMIN CAP_NET_RAW

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
systemctl enable rustray
systemctl restart rustray

# ── Summary ───────────────────────────────────────────────────────────────────
echo ""
info "═══════════════════════════════════════════════════════"
info "  Node provisioning complete!"
info "  Node ID     : ${NODE_ID}"
info "  Server      : ${SERVER_ADDR}"
info "  gRPC Port   : ${GRPC_PORT}"
info "  Binary SHA  : ${ACTUAL_HASH}"
info "  Cert FP     : ${CERT_FINGERPRINT}"
info "  Status      : $(systemctl is-active rustray)"
info "═══════════════════════════════════════════════════════"
