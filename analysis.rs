use std::collections::{HashMap, HashSet};

/// Calculates the degree distribution of the graph.
pub fn calculate_degree_distribution(graph: &HashMap<String, Vec<String>>) -> HashMap<usize, usize> {
    let mut degree_count = HashMap::new();
    for neighbors in graph.values() {
        let degree = neighbors.len();
        *degree_count.entry(degree).or_insert(0) += 1;
    }
    degree_count
}

/// Prints the degree distribution.
pub fn print_degree_distribution(degree_distribution: &HashMap<usize, usize>) {
    println!("Degree Distribution:");
    for (degree, count) in degree_distribution {
        println!("Degree: {}, Count: {}", degree, count);
    }
}

/// Saves the degree distribution to a file.
pub fn save_degree_distribution_to_file(
    degree_distribution: &HashMap<usize, usize>,
    file_path: &str,
) -> Result<(), std::io::Error> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(file_path)?;
    writeln!(file, "Degree,Count")?;
    for (degree, count) in degree_distribution {
        writeln!(file, "{},{}", degree, count)?;
    }
    Ok(())
}

/// Calculates centrality measures for nodes in the graph.
pub fn calculate_centrality_measures(graph: &HashMap<String, Vec<String>>) -> HashMap<String, f64> {
    let mut centrality = HashMap::new();
    for (node, neighbors) in graph {
        let unique_neighbors: HashSet<_> = neighbors.iter().collect();
        centrality.insert(node.clone(), unique_neighbors.len() as f64);
    }
    centrality
}

/// Fits the degree distribution to a power-law and calculates the exponent.
pub fn fit_power_law(degree_distribution: &HashMap<usize, usize>) -> f64 {
    let degrees: Vec<usize> = degree_distribution.keys().cloned().collect();
    let counts: Vec<usize> = degree_distribution.values().cloned().collect();

    // Maximum Likelihood Estimation for the power-law exponent (alpha)
    let log_degrees: Vec<f64> = degrees.iter().map(|&d| d as f64).collect();
    let log_counts: Vec<f64> = counts.iter().map(|&c| c as f64).collect();

    // Calculate the exponent using a simple method (log-log fit)
    let sum_log_degrees: f64 = log_degrees.iter().sum();
    let sum_log_counts: f64 = log_counts.iter().sum();
    let sum_log_degrees_squared: f64 = log_degrees.iter().map(|&x| x.powi(2)).sum();
    let n = log_degrees.len() as f64;  // Correctly use `log_degrees.len()` for the count

    // Fit the power law exponent (alpha)
    let alpha = (n * sum_log_counts - sum_log_degrees * log_degrees.len() as f64)
        / (n * sum_log_degrees_squared - sum_log_degrees.powi(2));

    alpha
}

/// Performs the Kolmogorov-Smirnov test to evaluate the fit of the power-law distribution.
pub fn ks_test(degree_distribution: &HashMap<usize, usize>, alpha: f64) -> f64 {
    let degrees: Vec<usize> = degree_distribution.keys().cloned().collect();
    let counts: Vec<usize> = degree_distribution.values().cloned().collect();

    let n = degrees.len() as f64;
    let mut ks_statistic = 0.0;

    for (degree, &count) in degree_distribution {
        let expected_prob = (*degree as f64).powf(-alpha); // Dereference `degree` to get value
        let observed_prob = count as f64 / n;
        ks_statistic += (observed_prob - expected_prob).abs();
    }

    ks_statistic
}