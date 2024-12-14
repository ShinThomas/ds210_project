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

/// Calculates Jaccard similarity between the neighbors of two nodes.
pub fn jaccard_similarity(node1: &str, node2: &str, graph: &HashMap<String, Vec<String>>) -> f64 {
    if let (Some(neighbors1), Some(neighbors2)) = (graph.get(node1), graph.get(node2)) {
        let set1: HashSet<_> = neighbors1.iter().collect();
        let set2: HashSet<_> = neighbors2.iter().collect();

        let intersection_size = set1.intersection(&set2).count();
        let union_size = set1.union(&set2).count();

        intersection_size as f64 / union_size as f64
    } else {
        0.0 // Return 0 similarity if one or both nodes don't exist
    }
}

/// Finds the most similar and most dissimilar pairs of nodes based on Jaccard similarity.
pub fn find_most_similar_dissimilar_nodes(graph: &HashMap<String, Vec<String>>) -> ((String, String, f64), (String, String, f64)) {
    let mut most_similar = ("".to_string(), "".to_string(), f64::NEG_INFINITY);
    let mut most_dissimilar = ("".to_string(), "".to_string(), f64::INFINITY);

    let nodes: Vec<String> = graph.keys().cloned().collect();
    
    for i in 0..nodes.len() {
        for j in i + 1..nodes.len() {
            let node1 = &nodes[i];
            let node2 = &nodes[j];

            let similarity = jaccard_similarity(node1, node2, graph);

            // Update most similar
            if similarity > most_similar.2 {
                most_similar = (node1.clone(), node2.clone(), similarity);
            }

            // Update most dissimilar
            if similarity < most_dissimilar.2 {
                most_dissimilar = (node1.clone(), node2.clone(), similarity);
            }
        }
    }

    (most_similar, most_dissimilar)
}