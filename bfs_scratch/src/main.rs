use std::{usize, vec};

struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { items: Vec::new() }
    }
    fn enqueue(&mut self, value: T) {
        self.items.push(value);
    }
    fn dequeue(&mut self) -> Option<T> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> (Vec<usize>, Vec<Option<usize>>) {
    let v = graph.len();
    // the white indicates that the vertex has not been discovered yet!!
    // grey indicates that the vertex has been discoverd but the neighbour is not explored
    // black means vertex and the neighbours are fully explored
    let mut color = vec!["white"; v];
    let mut distance = vec![usize::MAX; v];
    let mut parent = vec![None; v];

    let mut queue = Queue::new();
    color[start] = "gray";
    distance[start] = 0;
    queue.enqueue(start);

    while !queue.is_empty() {
        let u = queue.dequeue().unwrap();
        for &v in &graph[u] {
            if color[v] == "white" {
                color[v] = "grey";
                distance[v] = distance[u] + 1;
                parent[v] = Some(u);
                queue.enqueue(v);
            }
        }
        color[u] = "black";
    }
    (distance, parent)
}

fn main() {
    let graph = vec![
        vec![1, 2],
        vec![0, 3, 4],
        vec![0, 5],
        vec![1],
        vec![1],
        vec![2],
    ];

    println!("Graph adjacency list:");
    for (i, neighbors) in graph.iter().enumerate() {
        println!("  {} -> {:?}", i, neighbors);
    }

    let (distances, parents) = bfs(&graph, 0);

    println!("\nBFS starting from node 0:");
    println!("  Distances: {:?}", distances);
    println!("  Parents: {:?}", parents);
}
