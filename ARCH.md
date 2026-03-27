# FOLYA Architecture Design Specification (V1.0) 🐱💼

This document defines the high-level architecture of the **FOLYA** Quantitative Trading Framework. It is based on the core principles of high-performance data processing, zero-cost abstraction, and strict logical decoupling.

## 1. Design Philosophy
- **Rust + Polars Core**: Leveraging the Arrow memory model and Polars' lazy evaluation to handle massive time-series data without memory overhead.
- **Physical Isolation**: Logic and data are strictly separated. 
- **Zero-Copy Data Flow**: Utilizing Rust's ownership and borrowing to pass DataFrames through the pipeline without expensive deep copies.
- **Concurreny by Design**: Leveraging `Rayon` for multi-threaded SIMD execution across all available CPU cores.

## 2. Layered Pipeline Architecture (The Four Filters)
The system is built as a unidirectional data pipeline. Data flows upward through four distinct layers:

### 2.1 Data Layer (`DataProcessor`)
- **Input**: Multi-source heterogeneous data (CSV, Parquet, REST APIs, WebSockets).
- **Processing**: Normalization, alignment, handling missing values (Forward Fill), and adjusted price calculations.
- **Output**: Standardized Polars `LazyFrame` indexed by `timestamp` and `ticker`.

### 2.2 Compute Layer (`FeatureEngine`)
- **Responsibility**: Transform raw OHLCV data into feature matrices ($X$) and label vectors ($Y$).
- **Technique**: Uses Polars expressions (`Expr`) for rolling windows (momentum, volatility) and cross-sectional ranking (`.over("date").rank()`).
- **Anti-Bias**: Strict physical isolation between feature generation and label generation to prevent look-ahead bias.

### 2.3 Model Layer (`AlphaModel` Trait)
- **Interface**:
    - `fn train(&mut self, features: &DataFrame, target: &str)`
    - `fn predict(&self, features: &DataFrame) -> PolarsResult<Series>`
- **Implementation**: Initially targeting **LightGBM** via FFI (C++ bridge) for robust gradient boosting on low signal-to-noise ratio financial data.

### 2.4 Execution & Risk Layer (`ExecutionEngine`)
The final gatekeeper before order generation, using a three-stage filter:
1.  **Macro State Overlay (`HMMOverlay`)**: Highest priority. Uses Hidden Markov Models to detect "Liquidity Crunch" or "High Risk" macro regimes. Intercepts all long signals during high-risk states.
2.  **Micro Stop-Loss (`ATRStopLoss`)**: Absolute defense per ticker. Uses Average True Range (ATR) trailing stops to clear positions if volatility-adjusted price thresholds are hit.
3.  **Dynamic Rebalancer (`Rebalancer`)**: Relative alpha rotation. Rebalances the portfolio based on Alpha Model rankings (e.g., Long Top 10%, Sell Bottom 30%).

## 3. Data Flow & Concurrency
1.  **Lazy Evaluation**: Calculations are built as a Directed Acyclic Graph (DAG) and only executed via `.collect()` when needed.
2.  **Vectorized SIMD**: All Polars operations are SIMD-optimized and automatically parallelized via Rayon.
3.  **Ownership Transfer**: DataFrames are passed by reference (`&DataFrame`) to models and filters to maintain zero-copy integrity.

## 4. Implementation Roadmap
1.  **Phase 1**: `DataProcessor` & `FeatureEngine` (The Foundation).
2.  **Phase 2**: `AlphaModel` Trait & LightGBM Integration (The Signal).
3.  **Phase 3**: `HMMOverlay` & `ATRStopLoss` (The Shield).
4.  **Phase 4**: `ExecutionEngine` & Order Protocol (The Sword).

---
*Created by Kate (Professional Secretary & Agent Orchestrator)*
