#[derive(Debug)]
struct Edge {
    to: usize,
    weight: u32,
}
type Graph = Vec<Vec<Edge>>;

fn main() {
    let graph: Graph = vec![
        vec![
            Edge { to: 1, weight: 4 },
            Edge { to: 2, weight: 2 },
        ],
        vec![
        Edge { to: 3, weight: 3 },
        ],
        vec![
            Edge { to: 1, weight: 1 },
            Edge { to: 3, weight: 5 },
        ],
        vec![],
    ];
    for (node, edges) in graph.iter().enumerate() {
        println!("Node {}", node);
        for edge in edges {
            println!(
                "  -> {} (weight {})",
                edge.to,
                edge.weight,
            );
        }
    }
}