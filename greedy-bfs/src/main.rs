struct Node {
    name: &'static str,
    heuristic: i32,
}

struct Edge {
    to: usize,   // index into Graph.nodes
    cost: i32,
}

struct Graph {
    nodes: Vec<Node>,
    adj: Vec<Vec<Edge>>,
}

fn main() {
    let graph = Graph {
        nodes: vec![
            Node { name: "A", heuristic: 7 },
            Node { name: "B", heuristic: 3 },
            Node { name: "C", heuristic: 1 },
        ],
        adj: vec![
            vec![Edge { to: 1, cost: 2 }, Edge { to: 2, cost: 5 }], // A → B, A → C
            vec![],  // B has no outgoing edges
            vec![],  // C has no outgoing edges
        ],
    };
}
