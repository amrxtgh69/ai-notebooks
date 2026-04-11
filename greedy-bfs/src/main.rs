use std::{collections::{BinaryHeap, HashMap}};
use std::cmp::Ordering;

struct Node {
    heuristic: u32,
}
struct Graph {
    nodes: HashMap<u32, Node>,             
    adj: HashMap<u32, Vec<(u32, u32)>>,   
}
#[derive(Eq, PartialEq)]
struct State {
    node: u32,
    heuristic: u32,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heuristic.cmp(&self.heuristic)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn gbfs(graph: &Graph, start: u32, goal: u32) -> Option<Vec<u32>> {
    let mut heap = BinaryHeap::new();

    heap.push(State {
        node: start,
        heuristic: graph.nodes.get(&start)?.heuristic,
    });
    while let Some(State { node, .. }) = heap.pop() {
        if node == goal {
            return Some(vec![node]);
        }
    }
    None
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
        if let Some(node) = graph.nodes.get(id) {
            let formatted: Vec<String> = neighbors
                .iter()
                .map(|(to, cost)| format!("{}(cost:{})", to, cost))
                .collect();
        println!("Node {}(h={}) -> [{}]", id, node.heuristic, formatted.join(", "));
        }
    }
}
