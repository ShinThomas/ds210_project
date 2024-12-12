use crate::Record; // Import the Record from main.rs
use ndarray::Array2;
use std::collections::HashMap;

// Function to calculate salary correlation
pub fn calculate_salary_correlation(records: &[Record]) -> Array2<f64> {
    // Example logic for correlation calculation (this can be extended with actual correlation computation)
    let mut correlation_matrix = Array2::<f64>::zeros((3, 3));

    // Fill correlation matrix with dummy values for now
    correlation_matrix[[0, 0]] = 1.0; // Example for 'experience_level' correlation with itself
    correlation_matrix[[0, 1]] = 0.5; // Dummy correlation with 'remote_ratio'
    correlation_matrix[[0, 2]] = 0.3; // Dummy correlation with 'company_size'

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