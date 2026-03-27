use polars::prelude::*;
use std::path::Path;
use crate::data::{DataError, DataProcessor, Result};

/// CSV data processor implementing the DataProcessor trait
pub struct CsvProcessor;

impl DataProcessor for CsvProcessor {
    fn process_lazy<P: AsRef<Path>>(source: P) -> Result<LazyFrame> {
        let path = source.as_ref();
        
        if !path.exists() {
            return Err(DataError::InvalidSource(
                format!("File does not exist: {}", path.display())
            ));
        }
        
        if !path.extension().map_or(false, |ext| ext == "csv") {
            return Err(DataError::InvalidSource(
                format!("File is not a CSV: {}", path.display())
            ));
        }
        
        // Use Polars Lazy API for zero-copy, efficient processing
        let lf = LazyCsvReader::new(path)
            .with_has_header(true)
            .with_try_parse_dates(true)
            .finish()
            .map_err(DataError::Polars)?;
        
        Ok(lf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::Builder;

    #[test]
    fn test_csv_processor_valid_file() {
        // Create temp file with .csv extension
        let temp_file = Builder::new()
            .suffix(".csv")
            .tempfile()
            .unwrap();
        
        let mut file = std::fs::File::create(temp_file.path()).unwrap();
        writeln!(file, "timestamp,open,high,low,close,volume").unwrap();
        writeln!(file, "2024-01-01,100.0,105.0,99.0,102.0,1000").unwrap();
        
        let result = CsvProcessor::process_lazy(temp_file.path());
        assert!(result.is_ok());
    }

    #[test]
    fn test_csv_processor_missing_file() {
        let result = CsvProcessor::process_lazy("/nonexistent/file.csv");
        assert!(matches!(result, Err(DataError::InvalidSource(_))));
    }

    #[test]
    fn test_csv_processor_wrong_extension() {
        let temp_file = Builder::new()
            .suffix(".txt")
            .tempfile()
            .unwrap();
        
        let mut file = std::fs::File::create(temp_file.path()).unwrap();
        writeln!(file, "data").unwrap();
        
        let result = CsvProcessor::process_lazy(temp_file.path());
        assert!(matches!(result, Err(DataError::InvalidSource(_))));
    }
}
