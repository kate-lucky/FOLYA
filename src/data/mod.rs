use polars::prelude::*;
use std::path::Path;

pub mod processor;

/// Error type for data processing operations
#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Polars error: {0}")]
    Polars(#[from] PolarsError),
    #[error("Invalid source: {0}")]
    InvalidSource(String),
}

/// Result type alias for data operations
pub type Result<T> = std::result::Result<T, DataError>;

/// Trait for data processors that can read from various sources
/// and return a Polars LazyFrame for zero-copy operations
pub trait DataProcessor {
    /// Process data from a given source path and return a LazyFrame
    fn process_lazy<P: AsRef<Path>>(source: P) -> Result<LazyFrame>;
    
    /// Process data and collect into a DataFrame
    fn process<P: AsRef<Path>>(source: P) -> Result<DataFrame> {
        Self::process_lazy(source)?.collect().map_err(DataError::Polars)
    }
}
