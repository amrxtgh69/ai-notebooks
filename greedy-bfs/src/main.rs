use std::collections::HashMap;


type Node = usize;

struct Edge {
    node: Node,
    cost: i32,
}
struct NodeData {
    name: String,
    heuristic: i32,
}
struct Graph {
    nodes: Vec<NodeData>,
    adj: HashMap<Node, Vec<Edge>>,
}

fn main() {
}
