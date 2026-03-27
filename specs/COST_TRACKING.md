# FOLYA Cost & Token Tracking Specification 🐱💼

This document defines the methodology and automation for tracking resource consumption within the **FOLYA** project.

## 1. Tracking Methodology

| Resource Type | Tracking Method | Verification Source |
| :--- | :--- | :--- |
| **Gemini API** | **Session Status Audit**: I use the `session_status` tool to retrieve real-time token counts and request numbers from the current turn. | OpenClaw `session_status` output. |
| **Grok API** | **API Headers**: For sub-agents using Grok, I inspect the metadata for `x-request-id` and token usage headers (if available) or estimate based on context size. | API metadata / Sub-agent logs. |
| **ChatGPT (OAuth)**| **Request Logging**: Since this is a flat-rate subscription, I track the **number of requests** to monitor rate-limit usage and efficiency. | Assistant internal request counter. |
| **Kimi AI Key** | **AI Key Metrics**: I track input/output tokens per task to ensure we stay within the monthly high-precision quota. | Kimi response metadata. |

## 2. Token Estimation Script (Conceptual)

I can implement a local Python script to estimate costs before and after tasks. This script will be stored in `workspace/folya/tools/cost_tracker.py`.

```python
import json

# Standard Pricing (USD per 1M tokens)
PRICING = {
    "gemini-3.1-pro": {"in": 2.00, "out": 12.00},
    "gemini-3.1-flash-lite": {"in": 0.25, "out": 1.50},
    "grok-4-1-fast": {"in": 0.20, "out": 0.60},
}

def calculate_cost(model, input_tokens, output_tokens):
    price = PRICING.get(model)
    if not price:
        return 0.0
    cost_in = (input_tokens / 1_000_000) * price["in"]
    cost_out = (output_tokens / 1_000_000) * price["out"]
    return round(cost_in + cost_out, 6)

# Usage in Orchestration
# 1. Run 'session_status' to get current totals.
# 2. Subtract previous totals to get 'Delta' for the last task.
# 3. Append Delta to MILESTONES.md.
```

## 3. Automation Plan: `REVIEWER` Integration

As the **Agent Orchestrator**, I will integrate cost tracking into the **REVIEWER** role protocol:

1.  **Phase Start**: Record current session token counts (`baseline`).
2.  **Phase End**: Retrieve new token counts and calculate the `Delta`.
3.  **Automated Logging**: The Reviewer will automatically APPEND the cost of each sub-agent task to the **`MILESTONES.md`** file during the merge process.

## 4. Data Verification & Integrity
To ensure the accuracy of the automated cost tracking:
- **Periodic Manual Audit**: Every 24 hours, the Orchestrator will manually run `openclaw sessions --json --active 1440` and compare the cumulative totals against the `MILESTONES.md` file. Any discrepancy >5% will be investigated.
- **Source of Truth**: The `openclaw sessions` command is the definitive source of truth for the current session's token consumption.

## 5. Log Rotation & Cleanup (The Janitor)
To prevent the audit logs from consuming excessive disk space:
1. **Rotation**: The `cost_audit.log` will be rotated weekly. The last 4 weeks of logs will be kept as `.log.1`, `.log.2`, etc.
2. **Cleanup**: Any logs older than 30 days will be automatically deleted by the `cost_auditor.sh` script.
3. **Archive**: Before deletion, significant milestone summaries are already permanently recorded in `MILESTONES.md`, ensuring the high-level financial history is never lost.

---
*Created by Kate (Professional Secretary & Agent Orchestrator)*
