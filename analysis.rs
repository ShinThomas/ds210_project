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