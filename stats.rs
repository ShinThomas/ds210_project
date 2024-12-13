use crate::Record;
use std::collections::HashMap;

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