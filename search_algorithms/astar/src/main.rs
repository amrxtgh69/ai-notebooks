// This is the implementation of the A Star algorithm which is one informed search strategy too.

use std::collections::HashMap;

type Graph<V, E> = HashMap<V, Vec<(V, E)>>;

#[derive(PartialEq, Eq)]
struct State<V, E> {
    f: E,
    vertex: V,
}
impl<V, E: Ord> Ord for State<V, E> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f)
    }
}
impl<V, E: Ord> PartialOrd for State<V, E> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn astar<V, E>(
    graph: Graph<V, E>,
    start: V,
    goal: V,
)
fn main() {}
