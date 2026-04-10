use std::collections::HashMap;

struct Node {
    heuristic: u32,
}

struct Graph {
    nodes: HashMap<u32, Node>,             
    adj: HashMap<u32, Vec<(u32, u32)>>,   
}

fn main() {
    let mut nodes = HashMap::new();
    nodes.insert(0, Node { heuristic: 7 });
    nodes.insert(1, Node { heuristic: 3 });
    nodes.insert(2, Node { heuristic: 1 });

    let mut adj = HashMap::new();
    adj.insert(0, vec![(1, 2), (2, 5)]); 
    adj.insert(1, vec![(2, 1)]);           
    adj.insert(2, vec![]);                

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
