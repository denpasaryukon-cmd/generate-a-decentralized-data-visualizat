use std::collections::{HashMap, HashSet};
use tokio::{prelude::*, runtime::Runtime};

struct Node {
    id: usize,
    data: Vec<f64>,
    neighbors: HashSet<usize>,
}

impl Node {
    fn new(id: usize, data: Vec<f64>) -> Node {
        Node {
            id,
            data,
            neighbors: HashSet::new(),
        }
    }
}

struct DecentralizedTracker {
    nodes: HashMap<usize, Node>,
}

impl DecentralizedTracker {
    fn new() -> DecentralizedTracker {
        DecentralizedTracker {
            nodes: HashMap::new(),
        }
    }

    fn add_node(&mut self, id: usize, data: Vec<f64>) {
        self.nodes.insert(id, Node::new(id, data));
    }

    fn connect_nodes(&mut self, node1_id: usize, node2_id: usize) {
        if let Some(node1) = self.nodes.get_mut(&node1_id) {
            node1.neighbors.insert(node2_id);
        }
        if let Some(node2) = self.nodes.get_mut(&node2_id) {
            node2.neighbors.insert(node1_id);
        }
    }

    async fn visualize(&self) {
        let mut tasks = Vec::new();
        for (_, node) in &self.nodes {
            let task = tokio::spawn(async move {
                println!("Visualizing node {}:", node.id);
                for &neighbor in &node.neighbors {
                    println!("  -> Node {}", neighbor);
                }
            });
            tasks.push(task);
        }
        for task in tasks {
            task.await.expect("Error running task");
        }
    }
}

#[tokio::main]
async fn main() {
    let mut tracker = DecentralizedTracker::new();
    tracker.add_node(1, vec![1.0, 2.0, 3.0]);
    tracker.add_node(2, vec![4.0, 5.0, 6.0]);
    tracker.add_node(3, vec![7.0, 8.0, 9.0]);
    tracker.connect_nodes(1, 2);
    tracker.connect_nodes(2, 3);
    tracker.connect_nodes(1, 3);
    tracker.visualize().await;
}