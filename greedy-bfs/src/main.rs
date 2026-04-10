use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]

struct Node {
    name: String,
    heuristic: f64,
}

struct Edge {
    node: Node,
    cost: f64,
}

struct Graph {
    adj: HashMap<Node, Vec<Edge>>,
}

let mut graph: HashMap<Node, Vec<Edge>> = HashMap::new();


fn main() {

}
