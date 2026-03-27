# FOLYA External Resource Specification 🐱💼

This document lists all the external libraries, APIs, and resources required for the **FOLYA** project. This is to ensure all agents are aware of the available and required tools.

## 1. Programming Languages & Tools
- **Rust (1.77+)**: Primary language for the quant framework.
- **Cargo**: Rust's build system and package manager.

## 2. External Libraries (Crates)
All Rust projects should include the following core dependencies:

| Crate | Version | Purpose |
| :--- | :--- | :--- |
| **tokio** | 1.x | Async runtime |
| **serde** | 1.x | Serialization/Deserialization |
| **serde_json** | 1.x | JSON processing |
| **reqwest** | 0.11+ | HTTP client for market data APIs |
| **sqlx** | 0.7+ | Database interaction |
| **chrono** | 0.4+ | Time and date handling |
| **anyhow** | 1.x | Error handling |
| **tracing** | 0.1+ | Logging and tracing |

## 3. Market Data APIs (To be integrated)
The project will target the following APIs:
- **Binance API**: For crypto market data.
- **Interactive Brokers API**: For equities and options.
- **Alpha Vantage**: For historical stock data.
- **Yahoo Finance**: For general market information.

*Note: All API keys must be securely stored in the environment or a separate non-committed configuration file.*

## 4. Databases
- **PostgreSQL**: For storing historical market data and trade logs.
- **Redis**: For high-speed real-time data caching.

## 5. Deployment Resources
- **GitHub**: Source control and project management.
- **Docker Hub**: For container image storage.
- **AWS/GCP**: Potential cloud providers for live trading instances.

## 6. Development Tools
- **Clippy**: Rust linter for code quality.
- **Rustfmt**: Code formatting for consistency.
- **Cargo-expand**: For macro debugging.
- **Cargo-tarpaulin**: For code coverage analysis.

---
*Created by Kate (Professional Secretary)*
