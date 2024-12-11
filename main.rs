use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]  // Deserialize each row into a struct
struct Record {
    work_year: u32,
    experience_level: String,
    employment_type: String,
    job_title: String,
    salary: f64,
    salary_currency: String,
    salary_in_usd: f64,
    employee_residence: String,
    remote_ratio: u32,
    company_location: String,
    company_size: String,
}

fn read_csv(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
    let mut records = Vec::new();
    
    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    
    Ok(records)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "ds_salaries.csv";  // Adjusted based on your project structure
    let records = read_csv(file_path)?;

    // Print the first 5 records as a test
    for record in records.iter().take(5) {
        println!("{:?}", record);
    }

    Ok(())
}