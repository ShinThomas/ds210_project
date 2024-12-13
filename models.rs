use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub work_year: u32,
    pub experience_level: String,
    pub employment_type: String,
    pub job_title: String,
    pub salary: f64,
    pub salary_currency: String,
    pub salary_in_usd: f64,
    pub employee_residence: String,
    pub remote_ratio: u32,
    pub company_location: String,
    pub company_size: String,
}