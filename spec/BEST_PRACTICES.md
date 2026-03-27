# FOLYA Software Development Best Practices 🐱💼

This document defines the architectural standards and coding practices for the **FOLYA** project. All code must adhere to these guidelines to ensure performance, maintainability, and reliability.

## 1. Architectural Patterns
- **Modular Monolith**: Start with a well-structured modular monolith. Each domain (Market Data, Execution, Risk Management) must be a separate Rust module or workspace crate.
- **Dependency Injection**: Use traits to define interfaces. Inject concrete implementations (e.g., `BinanceClient`, `MockExchange`) to facilitate testing and flexibility.
- **Actor Model (Optional)**: For high-concurrency trading logic, consider the Actor pattern (via `tokio::sync::mpsc`) to manage state without complex locking.

## 2. Rust-Specific Memory Management
- **Zero-Copy**: Prefer references (`&T`) and slices (`&[u8]`) over cloning where possible, especially for high-frequency market data.
- **Smart Pointers**: Use `Arc<T>` for thread-safe shared state and `Box<T>` for heap allocation of large structures.
- **Ownership**: Strictly follow Rust's ownership rules to avoid memory leaks and data races. Avoid `unsafe` unless absolutely necessary for FFI or extreme performance (must be documented).

## 3. Data Source & Management
- **Abstraction Layer**: All data sources (APIs, Databases, CSVs) must be abstracted behind a `DataSource` trait.
- **Buffering**: Use ring buffers or channels for real-time data ingestion to prevent backpressure from slowing down the system.
- **Persistence Strategy**:
    - **Hot Data**: Redis for real-time order books.
    - **Cold Data**: PostgreSQL (TimescaleDB) for historical OHLCV data.

## 4. Backend-Frontend Cooperation
- **Schema-First API**: Use JSON-RPC or REST with OpenAPI (Swagger) specifications.
- **Type-Safe Contracts**: Export Rust types to TypeScript/PWA frontend using tools like `ts-rs` to ensure contract consistency.
- **State Updates**: Prefer WebSockets for real-time price/order updates; reserve REST for configuration and historical queries.

## 5. Testing & Quality Assurance
- **Unit Testing**: 80%+ coverage for core quant logic. Use `cargo test`.
- **Integration Testing**: Test full flows (e.g., Order -> Execution -> Confirmation) using mock exchange APIs.
- **Property-Based Testing**: Use `proptest` for validating complex trading algorithms against a wide range of inputs.
- **Benchmarking**: Use `criterion.rs` to measure the latency of critical paths (order routing, signal generation).

## 6. Error Handling
- **No `panic!`**: Never use `unwrap()` or `expect()` in production code.
- **Result Type**: Always return `Result<T, E>`. Use `anyhow` for application-level errors and `thiserror` for library-level errors.

## 7. Deployment & CI/CD
- **Immutable Infrastructure**: Deploy via Docker containers defined in `/spec/ENVIRONMENT.md`.
- **Health Checks**: Implement `/health` and `/ready` endpoints for monitoring.
- **Atomic Pushes**: Always push to `kate-dev` first. Automated tests must pass before merging to `main`.

---
*Created by Kate (Professional Secretary)*
