#!/bin/bash
# FOLYA Cost Auditor Script 🐱💼
# Purpose: Automatically track and log token/request costs.

LOG_FILE="/root/.openclaw/workspace/folya/cost_audit.log"
MILESTONE_FILE="/root/.openclaw/workspace/folya/MILESTONES.md"
TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Ensure the log directory exists
mkdir -p "$(dirname "$LOG_FILE")"

echo "[$TIMESTAMP] Cost audit initiated..." >> "$LOG_FILE"

# 1. Fetch current session metrics via openclaw CLI
SESSION_DATA=$(openclaw sessions --json --active 60 2>/dev/null)

if [ $? -eq 0 ]; then
    # Extract total tokens using jq (if available) or basic grep/sed
    INPUT_TOKENS=$(echo "$SESSION_DATA" | grep '"inputTokens"' | head -n 1 | sed 's/.*: \([0-9]*\),/\1/')
    OUTPUT_TOKENS=$(echo "$SESSION_DATA" | grep '"outputTokens"' | head -n 1 | sed 's/.*: \([0-9]*\),/\1/')
    
    # 2. Update MILESTONES.md (Basic update logic)
    # Note: In a real environment, we would calculate the Delta and append it.
    # For now, we log the current cumulative state to the audit log.
    echo "[$TIMESTAMP] Active Session Tokens - In: $INPUT_TOKENS, Out: $OUTPUT_TOKENS" >> "$LOG_FILE"
    echo "[$TIMESTAMP] Cost audit completed." >> "$LOG_FILE"
else
    echo "[$TIMESTAMP] Error: Could not fetch session data from OpenClaw." >> "$LOG_FILE"
fi
