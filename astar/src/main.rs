// This is the implementation of the A Star algorithm which is one informed search strategy too.

use std::collections::HashMap;

type Graph<V, E> = HashMap<V, Vec<(V, E)>>;

struct State<V, E> {
    f: E,
    vertex: V,
}

fn main() {
}
