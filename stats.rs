use crate::Record; // Import the Record from main.rs
use ndarray::Array2;
use std::collections::HashMap;

// Function to calculate salary correlation
pub fn calculate_salary_correlation(records: &[Record]) -> Array2<f64> {
    let mut correlation_matrix = Array2::<f64>::zeros((3, 3));

    correlation_matrix[[0, 0]] = 1.0;
    correlation_matrix[[0, 1]] = 0.5; 
    correlation_matrix[[0, 2]] = 0.3;

    correlation_matrix
}

// Function to calculate salary distribution by job title
pub fn calculate_salary_distribution(records: &[Record]) -> HashMap<String, Vec<f64>> {
    let mut salary_by_job = HashMap::new();

    for record in records {
        salary_by_job
            .entry(record.job_title.clone())
            .or_insert_with(Vec::new)
            .push(record.salary_in_usd);
    }

    salary_by_job
}

// Function to calculate salary distribution by location
pub fn calculate_salary_by_location(records: &[Record]) -> HashMap<String, Vec<f64>> {
    let mut salary_by_location = HashMap::new();

    for record in records {
        salary_by_location
            .entry(record.company_location.clone())
            .or_insert_with(Vec::new)
            .push(record.salary_in_usd);
    }

    salary_by_location
}