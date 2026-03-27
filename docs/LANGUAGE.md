# FOLYA Language Choice: Rust

## Decision (2026-03-25)
Primary: **Rust** for quant trading framework.

### Rationale
- **Performance**: Zero-cost abstractions, no GC—ideal backtests/HFT.
- **Safety**: Memory/thread-safe; fewer bugs in strategies.
- **Ecosystem**: ta-rs (MACD/RSI/BB), polars (fast DF), rust-ccxt (exchanges), tactical (portfolio).
- **Local fit**: Builds on `/quant-lab-rust`; arm64/Linux native.
- **Modern**: Git-friendly, CI/CD ready (cargo test/doc).

### Alternatives Considered
- **Python**: Prototyping king (pandas/TA-Lib/ccxt), but slow live/risk-parallel.
- **C++**: Raw speed, but verbose/unsafe.
- **Julia**: Math perf, small community.

Update if needed. Targets: HK/US stocks, crypto.