use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

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

    fn gbfs(&self, start: usize, goal: usize) -> Option<Vec<usize>>{
        let mut visited = vec![false; self.adjacency_list.len()];
        let mut heap = BinaryHeap::new();

        heap.push(Reverse((self.get_heuristic(start).unwrap_or(0), start)));

        while let Some(entry) = heap.pop() {
            let Reverse((_heuristic, node)) = entry;
            if visited[node] {
                continue;
            }
            println!("Visiting : {}", node);
            visited[node] = true;

            if node == goal {
                println!("Reached goal");
                return;
            }
            for &nbr in self.neighbours(node) {
                if !visited[nbr] {
                    heap.push(Reverse((self.get_heuristic(nbr).unwrap_or(0), nbr)));
                }
            }
        }
        println!("Goal not reachable");
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
    graph.gbfs(0, 4);
}
