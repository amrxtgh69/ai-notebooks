use std::collections::HashMap;

struct Node {
    heuristic: u32,
}

struct Graph {
    nodes: HashMap<u32, Node>,              // node_id -> node data
    adj: HashMap<u32, Vec<(u32, u32)>>,    // node_id -> [(neighbor_id, cost)]
}

fn main() {
    let mut nodes = HashMap::new();
    nodes.insert(0, Node { heuristic: 7 });
    nodes.insert(1, Node { heuristic: 3 });
    nodes.insert(2, Node { heuristic: 1 });

    let mut adj = HashMap::new();
    adj.insert(0, vec![(1, 2), (2, 5)]);   // 0 -> 1 (cost 2), 0 -> 2 (cost 5)
    adj.insert(1, vec![(2, 1)]);            // 1 -> 2 (cost 1)
    adj.insert(2, vec![]);                  // goal node

    let graph = Graph { nodes, adj };

    for (id, neighbors) in &graph.adj {
        let h = graph.nodes[id].heuristic;
        let formatted: Vec<String> = neighbors
            .iter()
            .map(|(to, cost)| format!("{}(cost:{})", to, cost))
            .collect();
        println!("Node {}(h={}) -> [{}]", id, h, formatted.join(", "));
    }
}
