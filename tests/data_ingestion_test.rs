use folya::data::{processor::CsvProcessor, DataProcessor};
use polars::prelude::*;

/// Integration tests for data ingestion module
#[test]
fn test_csv_processor_reads_sample_file() {
    let result = CsvProcessor::process("data/sample.csv");
    assert!(result.is_ok(), "Should successfully read sample.csv");
}

#[test]
fn test_csv_processor_lazy_reads_sample_file() {
    let result = CsvProcessor::process_lazy("data/sample.csv");
    assert!(result.is_ok(), "Should successfully create LazyFrame from sample.csv");
}

#[test]
fn test_dataframe_has_expected_columns() {
    let df = CsvProcessor::process("data/sample.csv").expect("Failed to read sample.csv");
    
    let column_names: Vec<&str> = df.get_column_names().iter()
        .map(|s| s.as_str())
        .collect();
    
    // Check for expected OHLCV column names
    assert!(column_names.contains(&"timestamp"), "Should have 'timestamp' column");
    assert!(column_names.contains(&"open"), "Should have 'open' column");
    assert!(column_names.contains(&"high"), "Should have 'high' column");
    assert!(column_names.contains(&"low"), "Should have 'low' column");
    assert!(column_names.contains(&"close"), "Should have 'close' column");
    assert!(column_names.contains(&"volume"), "Should have 'volume' column");
}

#[test]
fn test_dataframe_row_count_matches_sample() {
    let df = CsvProcessor::process("data/sample.csv").expect("Failed to read sample.csv");
    
    // Our sample file has 10 data rows (plus header)
    let row_count = df.height();
    assert_eq!(row_count, 10, "Should have exactly 10 rows in sample data");
}

#[test]
fn test_lazyframe_column_verification() {
    let lf = CsvProcessor::process_lazy("data/sample.csv")
        .expect("Failed to create LazyFrame");
    
    // Verify we can select specific columns lazily
    let selected = lf.select([
        col("timestamp"),
        col("open"),
        col("high"),
        col("low"),
        col("close"),
        col("volume"),
    ]);
    
    let df = selected.collect().expect("Failed to collect LazyFrame");
    
    assert_eq!(df.width(), 6, "Should have exactly 6 columns");
    assert_eq!(df.height(), 10, "Should have exactly 10 rows");
}

#[test]
fn test_data_types_are_numeric() {
    let df = CsvProcessor::process("data/sample.csv").expect("Failed to read sample.csv");
    
    // Verify OHLCV columns are numeric types
    let numeric_columns = ["open", "high", "low", "close", "volume"];
    
    for col_name in &numeric_columns {
        let series = df.column(col_name)
            .expect(&format!("Column {} not found", col_name));
        
        let dtype = series.dtype();
        assert!(
            matches!(dtype, DataType::Float64 | DataType::Int64 | DataType::UInt64),
            "Column {} should be numeric, got {:?}",
            col_name, dtype
        );
    }
}
