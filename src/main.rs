<<<<<<< HEAD
fn main() {
    println!("Hello, world!");
=======
mod data;

use data::{processor::CsvProcessor, DataProcessor};

fn main() {
    println!("FOLYA Data Ingestion Test");
    println!("========================\n");

    let sample_path = "data/sample.csv";
    
    match CsvProcessor::process_lazy(sample_path) {
        Ok(lf) => {
            match lf.collect() {
                Ok(df) => {
                    println!("Successfully loaded data from {}", sample_path);
                    println!("\nFirst 5 rows:");
                    
                    // Display first 5 rows
                    let preview = df.head(Some(5));
                    println!("{}", preview);
                    
                    println!("\nDataFrame shape: {:?}", df.shape());
                    println!("Columns: {:?}", df.get_column_names());
                }
                Err(e) => {
                    eprintln!("Error collecting DataFrame: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error processing CSV: {}", e);
            std::process::exit(1);
        }
    }
>>>>>>> personal/feat/data-engineer/m2-data-ingestion
}
