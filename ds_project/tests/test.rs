use ds_project::Graph;

#[test]
fn test_add_edge() {
    let mut graph = Graph::new();

    // Add edges
    graph.add_edge("Node1", "Node2");
    graph.add_edge("Node2", "Node3");
    graph.add_edge("Node1", "Node3");
    graph.add_edge("Node1", "Node2"); // Duplicate edge

    // Verify adjacency list
    assert!(graph.nodes.contains_key("Node1"));
    assert!(graph.nodes.contains_key("Node2"));

    // Verify neighbors of Node1
    let node1_neighbors = &graph.nodes["Node1"];
    assert_eq!(node1_neighbors.len(), 3); // Node1 has three neighbors
    assert!(node1_neighbors.contains(&"Node2".to_string()));
    assert!(node1_neighbors.contains(&"Node3".to_string()));

    // Verify neighbors of Node2
    let node2_neighbors = &graph.nodes["Node2"];
    assert_eq!(node2_neighbors.len(), 1); // Node2 has one neighbor
    assert!(node2_neighbors.contains(&"Node3".to_string()));
}

#[test]
fn test_empty_graph() {
    let graph = Graph::new();

    // Verify that graph is empty
    assert!(graph.nodes.is_empty(), "Graph should have no nodes initially.");

    // Attempt to retrieve non-existent node's neighbors (should return None)
    assert!(graph.nodes.get("NonExistentNode").is_none(), "Non-existent node should not be found.");
}