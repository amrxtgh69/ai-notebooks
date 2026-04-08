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
    discovery_time: &mut Vec<i32>,
    finished_time: &mut Vec<i32>,
) {
    *time += 1;
    discovery_time[start] = *time;
    color[start] = Color::Gray;
    for &neighbour_node in &graph[start] {
        if color[neighbour_node] == Color::White {
            dfs(
                neighbour_node,
                graph,
                color,
                time,
                discovery_time,
                finished_time,
            );
        }
    }
    color[start] = Color::Black;
    *time += 1;
    finished_time[start] = *time;
}

fn main() {
    let graph: Vec<Vec<usize>> = vec![vec![1, 2], vec![2], vec![3], vec![1]];
    let n = graph.len();
    let mut color = vec![Color::White; n];
    let mut discovery_time = vec![0; n];
    let mut finished_time = vec![0; n];
    let mut time = 0;

    let start = 0;

    dfs(
        start,
        &graph,
        &mut color,
        &mut time,
        &mut discovery_time,
        &mut finished_time,
    );

    for (i, neighbours) in graph.iter().enumerate() {
        println!("{} -> {:?}", i, neighbours);
    }
    println!("Node -> Discovery Time / Finishing Time:");
    for i in 0..n {
        println!("Node {} -> {} / {}", i, discovery_time[i], finished_time[i]);
    }
}
