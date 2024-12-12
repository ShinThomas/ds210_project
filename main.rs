mod stats;
mod graph;
mod analysis;

use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use serde::Deserialize;
use graph::Graph;
use analysis::{calculate_degree_distribution, calculate_centrality_measures, print_degree_distribution, save_degree_distribution_to_file, find_most_similar_dissimilar_nodes};
use stats::{calculate_salary_correlation, calculate_salary_distribution, calculate_salary_by_location};

#[derive(Debug, Deserialize)]
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
    // Load dataset
    let file_path = "ds_salaries.csv";
    let records = read_csv(file_path)?;

    // Display first few records
    for record in records.iter().take(5) {
        println!("{:?}", record);
    }

    // Build graph
    let mut graph = Graph::new();
    for record in &records {
        graph.add_edge(&record.job_title, &record.company_location);
    }

    println!("\nGraph Structure:");
    for (node, neighbors) in &graph.nodes {
        println!("Node: {}, Neighbors: {:?}", node, neighbors);
    }

    // Perform degree distribution analysis
    let degree_distribution = calculate_degree_distribution(&graph.nodes); // Pass graph.nodes
    print_degree_distribution(&degree_distribution);

    // Optionally save degree distribution to file
    let output_file = "degree_distribution.csv";
    save_degree_distribution_to_file(&degree_distribution, output_file)?;
    println!("\nDegree distribution saved to {}", output_file);

    // Perform centrality analysis
    println!("\nCentrality Measures:");
    let centrality_measures = calculate_centrality_measures(&graph.nodes); // Pass graph.nodes
    for (node, centrality) in centrality_measures {
        println!("Node: {}, Centrality: {:.2}", node, centrality);
    }

    // Find most similar and most dissimilar nodes based on Jaccard similarity
    let (most_similar, most_dissimilar) = find_most_similar_dissimilar_nodes(&graph.nodes);

    println!("\nMost Similar Nodes: {} and {}, Similarity: {:.4}", most_similar.0, most_similar.1, most_similar.2);
    println!("Most Dissimilar Nodes: {} and {}, Similarity: {:.4}", most_dissimilar.0, most_dissimilar.1, most_dissimilar.2);

    Ok(())
}