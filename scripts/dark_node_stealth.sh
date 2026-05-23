#!/bin/bash
# scripts/dark_node_stealth.sh
# Mutilates RF transmission power to evade drone thermal/RF triangulation.
# Usage: ./dark_node_stealth.sh <interface>

INTERFACE="${1:-wlan0}"

echo "Starting Dark Node RF Stealth on $INTERFACE..."
echo "Simulating erratic mobile device TxPower signatures."

# Loop forever to vary the signal gain
while true; do
  # Random txpower between 5 dBm and 20 dBm
  GAIN=$(( (RANDOM % 16) + 5 ))
  echo "[$(date -u)] Setting RF gain to ${GAIN} dBm on $INTERFACE"
  
  # Note: Requires root privileges
  iw dev "$INTERFACE" set txpower fixed "${GAIN}00" 2>/dev/null || echo "Failed to set txpower (needs root/iw?)"
  
  # Random delay between 15 to 90 seconds (typical mobile wandering)
  DELAY=$(( (RANDOM % 75) + 15 ))
  sleep "$DELAY"
done
