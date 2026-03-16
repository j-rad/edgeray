#!/bin/bash
set -e

# EdgeRay Stress Test - WiFi-to-LTE Handover Resilience
# Validates 100% success rate during network interface switching

echo "╔════════════════════════════════════════════════════════════╗"
echo "║   EdgeRay Stress Test: Network Handover Resilience        ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Configuration
ITERATIONS=100
TEST_URL="https://www.google.com"
TIMEOUT=10
LOG_FILE="stress_test_$(date +%Y%m%d_%H%M%S).log"
LEAK_CHECK_ENABLED=true

# Counters
SUCCESS_COUNT=0
FAILURE_COUNT=0
LEAK_COUNT=0

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1" | tee -a "$LOG_FILE"
}

log_success() {
    echo -e "${GREEN}[✓]${NC} $1" | tee -a "$LOG_FILE"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1" | tee -a "$LOG_FILE"
}

log_error() {
    echo -e "${RED}[✗]${NC} $1" | tee -a "$LOG_FILE"
}

# Check if rustray is running
check_rustray() {
    if ! pgrep -x "rustray" > /dev/null; then
        log_error "rustray is not running. Please start the VPN first."
        exit 1
    fi
}

# Get network connections before test
get_connections() {
    netstat -tn 2>/dev/null | grep ESTABLISHED | wc -l
}

# Simulate network interface switch
simulate_handover() {
    local iteration=$1
    
    # Get connections before
    local conn_before=$(get_connections)
    
    # Simulate interface down/up (requires root)
    if [ "$EUID" -eq 0 ]; then
        # Find active interface
        local iface=$(ip route | grep default | awk '{print $5}' | head -1)
        
        if [ -n "$iface" ]; then
            log_info "Iteration $iteration: Simulating handover on $iface"
            
            # Brief interface toggle
            ip link set "$iface" down
            sleep 0.5
            ip link set "$iface" up
            sleep 1
        else
            log_warn "Iteration $iteration: No active interface found, skipping physical toggle"
        fi
    else
        log_warn "Not running as root, skipping physical interface toggle"
    fi
    
    # Test connectivity
    if timeout "$TIMEOUT" curl -s -o /dev/null -w "%{http_code}" "$TEST_URL" | grep -q "200"; then
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        log_success "Iteration $iteration: Connection successful"
        
        # Check for leaks
        if [ "$LEAK_CHECK_ENABLED" = true ]; then
            local conn_after=$(get_connections)
            local conn_diff=$((conn_after - conn_before))
            
            if [ "$conn_diff" -gt 5 ]; then
                LEAK_COUNT=$((LEAK_COUNT + 1))
                log_warn "Iteration $iteration: Potential leak detected (+$conn_diff connections)"
            fi
        fi
    else
        FAILURE_COUNT=$((FAILURE_COUNT + 1))
        log_error "Iteration $iteration: Connection failed"
    fi
}

# Kill-switch verification
verify_killswitch() {
    log_info "Verifying kill-switch functionality..."
    
    # Stop rustray
    pkill -TERM rustray || true
    sleep 2
    
    # Try to connect without VPN (should fail if kill-switch works)
    if timeout 5 curl -s -o /dev/null "$TEST_URL"; then
        log_error "Kill-switch FAILED: Connection succeeded without VPN"
        return 1
    else
        log_success "Kill-switch PASSED: Connection blocked without VPN"
        return 0
    fi
}

# Main test loop
main() {
    log_info "Starting stress test with $ITERATIONS iterations"
    log_info "Test URL: $TEST_URL"
    log_info "Timeout: ${TIMEOUT}s"
    log_info "Leak detection: $LEAK_CHECK_ENABLED"
    echo ""
    
    check_rustray
    
    # Baseline connection count
    local baseline_conn=$(get_connections)
    log_info "Baseline connections: $baseline_conn"
    echo ""
    
    # Run iterations
    for i in $(seq 1 $ITERATIONS); do
        simulate_handover "$i"
        
        # Brief pause between iterations
        sleep 0.5
        
        # Progress indicator
        if [ $((i % 10)) -eq 0 ]; then
            local success_rate=$(awk "BEGIN {printf \"%.2f\", ($SUCCESS_COUNT / $i) * 100}")
            echo ""
            log_info "Progress: $i/$ITERATIONS iterations (${success_rate}% success rate)"
            echo ""
        fi
    done
    
    # Final statistics
    echo ""
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║   Test Results                                             ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
    
    local success_rate=$(awk "BEGIN {printf \"%.2f\", ($SUCCESS_COUNT / $ITERATIONS) * 100}")
    local failure_rate=$(awk "BEGIN {printf \"%.2f\", ($FAILURE_COUNT / $ITERATIONS) * 100}")
    
    log_info "Total Iterations: $ITERATIONS"
    log_success "Successful: $SUCCESS_COUNT (${success_rate}%)"
    
    if [ "$FAILURE_COUNT" -gt 0 ]; then
        log_error "Failed: $FAILURE_COUNT (${failure_rate}%)"
    else
        log_success "Failed: $FAILURE_COUNT (${failure_rate}%)"
    fi
    
    if [ "$LEAK_COUNT" -gt 0 ]; then
        log_warn "Potential Leaks: $LEAK_COUNT"
    else
        log_success "No leaks detected"
    fi
    
    echo ""
    
    # Final connection count
    local final_conn=$(get_connections)
    local total_leak=$((final_conn - baseline_conn))
    log_info "Final connections: $final_conn (Δ $total_leak from baseline)"
    
    echo ""
    log_info "Detailed log: $LOG_FILE"
    echo ""
    
    # Verify kill-switch
    if [ "$EUID" -eq 0 ]; then
        echo ""
        verify_killswitch
        
        # Restart rustray
        log_info "Restarting rustray..."
        # Add your rustray start command here
        # systemctl start rustray || /usr/bin/rustray &
    fi
    
    # Exit code based on success rate
    if [ "$SUCCESS_COUNT" -eq "$ITERATIONS" ]; then
        echo ""
        log_success "✓ 100% SUCCESS RATE ACHIEVED!"
        exit 0
    else
        echo ""
        log_error "✗ Success rate below 100%"
        exit 1
    fi
}

# Cleanup on exit
cleanup() {
    log_info "Cleaning up..."
}

trap cleanup EXIT

# Run main test
main
