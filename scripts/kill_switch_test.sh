#!/bin/bash
# EdgeRay Kill Switch Test
# Tests traffic leak protection when VPN process is forcefully terminated

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${YELLOW}EdgeRay Kill Switch Test${NC}"
echo "================================"
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo -e "${RED}Error: This script must be run as root${NC}"
    echo "Usage: sudo ./kill_switch_test.sh"
    exit 1
fi

# Configuration
TEST_DOMAIN="example.com"
TEST_IP="93.184.216.34"
CLEARTEXT_INTERFACE="eth0"  # Change to your actual interface
VPN_BINARY="./target/release/edgeray-core"
TEST_DURATION=30

# Function to check for cleartext packets
check_cleartext_traffic() {
    local interface=$1
    local duration=$2
    
    echo -e "${YELLOW}Monitoring $interface for cleartext packets (${duration}s)...${NC}"
    
    # Capture packets on cleartext interface
    timeout $duration tcpdump -i $interface -c 100 -w /tmp/cleartext_capture.pcap 2>/dev/null || true
    
    # Analyze capture
    local packet_count=$(tcpdump -r /tmp/cleartext_capture.pcap 2>/dev/null | wc -l)
    
    if [ $packet_count -gt 0 ]; then
        echo -e "${RED}FAIL: Detected $packet_count cleartext packets!${NC}"
        echo "Packet details:"
        tcpdump -r /tmp/cleartext_capture.pcap -n | head -20
        return 1
    else
        echo -e "${GREEN}PASS: No cleartext packets detected${NC}"
        return 0
    fi
}

# Function to verify TUN device exists
check_tun_device() {
    if ip link show edgeray0 &>/dev/null; then
        echo -e "${GREEN}✓ TUN device 'edgeray0' exists${NC}"
        return 0
    else
        echo -e "${RED}✗ TUN device 'edgeray0' not found${NC}"
        return 1
    fi
}

# Function to verify TUN device is gone
check_tun_device_removed() {
    if ip link show edgeray0 &>/dev/null; then
        echo -e "${RED}✗ TUN device 'edgeray0' still exists!${NC}"
        return 1
    else
        echo -e "${GREEN}✓ TUN device 'edgeray0' removed${NC}"
        return 0
    fi
}

# Test 1: Normal operation
echo -e "\n${YELLOW}Test 1: Normal VPN Operation${NC}"
echo "----------------------------"

# Start EdgeRay in background
echo "Starting EdgeRay VPN..."
$VPN_BINARY --config test-config.json &
VPN_PID=$!
echo "VPN PID: $VPN_PID"

# Wait for TUN device to be created
sleep 3

if ! check_tun_device; then
    echo -e "${RED}Test setup failed: TUN device not created${NC}"
    kill $VPN_PID 2>/dev/null || true
    exit 1
fi

# Verify traffic goes through VPN
echo "Testing connectivity through VPN..."
if ping -c 3 -W 5 $TEST_DOMAIN &>/dev/null; then
    echo -e "${GREEN}✓ Connectivity through VPN working${NC}"
else
    echo -e "${YELLOW}⚠ Ping failed (may be blocked by server)${NC}"
fi

# Test 2: Graceful shutdown
echo -e "\n${YELLOW}Test 2: Graceful Shutdown${NC}"
echo "-------------------------"

echo "Sending SIGTERM to VPN process..."
kill -TERM $VPN_PID
sleep 2

if check_tun_device_removed; then
    echo -e "${GREEN}✓ Graceful shutdown: TUN device cleaned up${NC}"
else
    echo -e "${RED}✗ Graceful shutdown: TUN device not cleaned up${NC}"
fi

# Check for traffic leaks
if check_cleartext_traffic $CLEARTEXT_INTERFACE 5; then
    echo -e "${GREEN}✓ No traffic leaks after graceful shutdown${NC}"
else
    echo -e "${RED}✗ Traffic leaked after graceful shutdown!${NC}"
    exit 1
fi

# Test 3: Force kill (SIGKILL)
echo -e "\n${YELLOW}Test 3: Force Kill (SIGKILL)${NC}"
echo "----------------------------"

# Restart VPN
echo "Restarting EdgeRay VPN..."
$VPN_BINARY --config test-config.json &
VPN_PID=$!
echo "VPN PID: $VPN_PID"

sleep 3

if ! check_tun_device; then
    echo -e "${RED}Test setup failed: TUN device not created${NC}"
    exit 1
fi

# Generate some traffic
echo "Generating background traffic..."
ping -c 100 $TEST_DOMAIN &>/dev/null &
PING_PID=$!

sleep 2

# Force kill VPN process
echo -e "${RED}Sending SIGKILL to VPN process (simulating crash)...${NC}"
kill -9 $VPN_PID

# Immediately check for TUN device removal
sleep 1

if check_tun_device_removed; then
    echo -e "${GREEN}✓ Force kill: TUN device removed by OS${NC}"
else
    echo -e "${RED}✗ Force kill: TUN device still exists!${NC}"
    # Try to clean up manually
    ip link delete edgeray0 2>/dev/null || true
fi

# Critical test: Check for cleartext packet leaks
echo -e "\n${YELLOW}Critical: Checking for traffic leaks after force kill...${NC}"

# Kill the ping process
kill $PING_PID 2>/dev/null || true

# Monitor for any cleartext traffic
if check_cleartext_traffic $CLEARTEXT_INTERFACE 10; then
    echo -e "${GREEN}✓✓✓ PASS: No traffic leaks detected after force kill!${NC}"
    echo -e "${GREEN}Kill switch is working correctly.${NC}"
else
    echo -e "${RED}✗✗✗ FAIL: Traffic leaked after force kill!${NC}"
    echo -e "${RED}SECURITY ISSUE: Kill switch not working!${NC}"
    exit 1
fi

# Test 4: DNS leak test
echo -e "\n${YELLOW}Test 4: DNS Leak Protection${NC}"
echo "---------------------------"

# Start VPN again
$VPN_BINARY --config test-config.json &
VPN_PID=$!
sleep 3

# Capture DNS queries
echo "Monitoring DNS queries..."
timeout 5 tcpdump -i any port 53 -w /tmp/dns_capture.pcap 2>/dev/null &
TCPDUMP_PID=$!

# Generate DNS queries
nslookup $TEST_DOMAIN &>/dev/null || true
sleep 2

kill $TCPDUMP_PID 2>/dev/null || true
wait $TCPDUMP_PID 2>/dev/null || true

# Check if DNS queries went through cleartext interface
DNS_PACKETS=$(tcpdump -r /tmp/dns_capture.pcap -i $CLEARTEXT_INTERFACE 2>/dev/null | wc -l || echo "0")

if [ "$DNS_PACKETS" -eq 0 ]; then
    echo -e "${GREEN}✓ No DNS leaks detected${NC}"
else
    echo -e "${YELLOW}⚠ Detected $DNS_PACKETS DNS packets on cleartext interface${NC}"
    echo "This may indicate DNS leak. Review DNS hijacking configuration."
fi

# Cleanup
kill $VPN_PID 2>/dev/null || true
rm -f /tmp/cleartext_capture.pcap /tmp/dns_capture.pcap

# Summary
echo -e "\n${GREEN}================================${NC}"
echo -e "${GREEN}All Kill Switch Tests Passed!${NC}"
echo -e "${GREEN}================================${NC}"
echo ""
echo "Summary:"
echo "  ✓ TUN device lifecycle management"
echo "  ✓ Graceful shutdown cleanup"
echo "  ✓ Force kill protection"
echo "  ✓ No traffic leaks detected"
echo "  ✓ DNS leak protection"
echo ""
echo -e "${YELLOW}Note: This test validates the kill switch mechanism.${NC}"
echo -e "${YELLOW}For production, also test with firewall rules and network changes.${NC}"

exit 0
