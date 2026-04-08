#[derive(Clone, PartialEq, Debug)]
enum Color {
    White,
    Gray,
    Black,
}

fn dfs(start: usize, graph: &Vec<Vec<usize>>, color: &mut Vec<Color>) {
    color[start] = Color::Gray;
    let neighbors = graph[start].clone();
    for &neighbour_node in &neighbors {
        if color[neighbour_node] == Color::White {
            dfs(neighbour_node, graph, color);
        }
    }
    color[start] = Color::Black;
}

fn main() {
    let graph: Vec<Vec<usize>> = vec![vec![1, 2], vec![2], vec![3], vec![1]];
    let n = graph.len();
    let start = 0;
    let mut color = vec![Color::White; n];

    dfs(start, &graph, &mut color);

    println!("Final colors:");
    for i in 0..n {
        println!("Node {} -> {:?}", i, color[i]);
    }
}
