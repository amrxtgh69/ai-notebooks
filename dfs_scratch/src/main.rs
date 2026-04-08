use std::collections::HashMap;

enum Color {
    White,
    Gray,
    Black,
}

fn main() {
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![2]);
    graph.insert(2, vec![0, 3]);
    graph.insert(3, vec![3]);

    let mut keys: Vec<_> = graph.keys().collect();
    keys.sort();

    for node in keys {
        let neighbors = &graph[node];
        let neighbors_str = neighbors
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("{} -> {}", node, neighbors_str);
    }
}
