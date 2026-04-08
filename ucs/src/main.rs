use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

type Node = &'static str;
type Cost = i32;

struct Edge {
    node: Node,
    cost: Cost,
}
type Graph = HashMap<Node, Edge>;

struct State {
    cost: Cost,
    node: Node,
    path: Vec<Node>,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // reversal happen here
        other.cost.cmp(&self.cost)
    }
}
//docs says that if Ord is implemented then PartialOrd should be implemented too.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // hmm this is tricky
        Some(self.cmp(other))
    }
}

fn ucs_search(graph: Graph, start: Node, goal: Node) -> Option<Cost, Vec<Node>> {
    let mut heap = BinaryHeap::new();
    let mut visited: HashMap<Node, Cost> = HashMap::new();

    heap.push(State {
        cost: 0,
        node: start,
        path: vec![start],
    });

}

fn main() {
    let graph: Graph = HashMap::new();
    graph.insert("A", vec![ Edge { node: "B", cost: 1 },  Edge { node: "C", cost: 4 } ];
    graph.insert("B", vec![Edge { node: "C", cost: 2 }, Edge { node: "D", cost: 5 }]);
    graph.insert("C", vec![Edge { node: "D", cost: 1 }]);
    graph.insert("D", vec![]);
}
