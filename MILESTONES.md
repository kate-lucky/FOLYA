# FOLYA Project Milestones & Budget Control 🐱💼

This document tracks the strategic progression and resource consumption of the **FOLYA** project. It is managed by the Agent Orchestrator (Kate) to ensure high-velocity execution within the specified budget.

## 1. Project Roadmap & Status

| Milestone | Phase | Key Deliverables | Status | Target Date |
| :--- | :--- | :--- | :--- | :--- |
| **M1** | **Foundation** | `ARCH.md`, `/specs`, Project Scaffolding | **COMPLETED** | 2026-03-27 |
| **M2** | **Data Ingestion** | `DataProcessor`, Polars integration, Binance/CSV connectors | *PENDING* | 2026-04-03 |
| **M3** | **Feature Engineering**| `FeatureEngine`, Rolling windows, Cross-sectional ranking | *PENDING* | 2026-04-10 |
| **M4** | **Alpha Modeling** | `AlphaModel` trait, LightGBM FFI, Training pipeline | *PENDING* | 2026-04-20 |
| **M5** | **Risk & Strategy** | `HMMOverlay`, `ATRStopLoss`, `ExecutionEngine` | *PENDING* | 2026-04-30 |
| **M6** | **Alpha Test** | Backtesting engine, Performance reporting (Sharpe/Drawdown) | *PENDING* | 2026-05-10 |

---

## 2. Budget & Resource Consumption Tracking

**Total Monthly Budget (Consolidated)**: £50 (~$63 USD)
**Reporting Period**: 2026-03-27 to 2026-04-27

### 2.1 Consumption Summary (Current Session/Milestone M1)

| Resource | Model | Requests | Input Tokens | Output Tokens | Est. Cost (USD) |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Gemini API** | Gemini-Flash | 12 | ~45k | ~5k | $0.008 |
| **Grok API** | Grok-4-1-Fast | 0 | 0 | 0 | $0.000 |
| **ChatGPT (OAuth)**| GPT-4o | 15 | N/A (Sub) | N/A (Sub) | $0.000* |
| **Kimi AI Key** | Kimi-Code | 4 | ~12k | ~3k | $0.000** |
| **TOTAL** | | **31** | | | **$0.008** |

*\* ChatGPT is under a fixed monthly subscription; usage does not impact the variable API budget.*
*\*\* Kimi usage is tracked against the monthly AI Key quota; no direct USD cost per request.*

### 2.2 Cost per Milestone (Historical)
- **M1 (Foundation)**: $0.008 (Establishing specs, architecture, and project init).

---

## 3. Resource Optimization Strategy
- **Low-Cost Processing**: Use **Gemini-Flash** for routine file reads, log analysis, and status checks.
- **Precision Coding**: Reserve **Kimi** for complex Rust implementation to minimize code iteration cycles.
- **Complex Reasoning**: Use **ChatGPT (OAuth)** for architectural planning and math modeling to leverage the fixed-cost subscription.
- **Live Intelligence**: Use **Grok** only when real-time X/Twitter sentiment or news is required for signal generation.

---
*Created by Kate (Professional Secretary & Agent Orchestrator)*
