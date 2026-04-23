use std::collections::HashMap;

struct Graph {
    adjacency_list: Vec<Vec<usize>>,
    heuristic: HashMap<usize, u32>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Graph {
            adjacency_list: vec![Vec::new(); size],
            heuristic: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adjacency_list[u].push(v);
    }

    fn add_heuristic(&mut self, node: usize, h: u32) {
        self.heuristic.insert(node, h);
    }

    fn get_heuristic(&self, node: usize) -> Option<u32> {
        self.heuristic.get(&node).copied()
    }

    fn neighbours(&self, u: usize) -> &Vec<usize> {
        &self.adjacency_list[u]
    }
}
fn main() {
    let mut graph = Graph::new(5);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);

    graph.add_heuristic(0, 5);
    graph.add_heuristic(1, 3);
    graph.add_heuristic(2, 4);
    graph.add_heuristic(3, 1);
    graph.add_heuristic(4, 0);

    for v in 0..5 {
        println!("Neighbors of vertex {}: {:?}", v, graph.neighbours(v));
        println!(
            "Heuristic of vertex {}: {}",
            v,
            graph.get_heuristic(v).unwrap_or(0)
        );
    }
}
