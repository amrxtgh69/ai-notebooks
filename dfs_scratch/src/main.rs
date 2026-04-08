#[derive(Clone, PartialEq, Debug)]
enum Color {
    White,
    Gray,
    Black,
}
fn dfs(
    start: usize,
    graph: &Vec<Vec<usize>>,
    color: &mut Vec<Color>,
    time: &mut i32,
    d: &mut Vec<i32>,
    r: &mut Vec<i32>,
) {
    *time += 1;
    d[start] = *time;
    color[start] = Color::Gray;
    for &neighbour_node in &graph[start] {
        if color[neighbour_node] == Color::White {
            dfs(neighbour_node, graph, color, time, d, r);
        }
    }
    color[start] = Color::Black;
    *time += 1;
    r[start] = *time;
}

fn main() {
    let graph: Vec<Vec<usize>> = vec![vec![1, 2], vec![2], vec![3], vec![1]];
    let n = graph.len();
    let mut color = vec![Color::White; n];
    let mut d = vec![0; n];
    let mut r = vec![0; n];
    let mut time = 0;

    let start = 0;

    dfs(start, &graph, &mut color, &mut time, &mut d, &mut r);

    println!("Node -> Discovery Time / Finishing Time:");
    for i in 0..n {
        println!("Node {} -> {} / {}", i, d[i], r[i]);
    }
}
