mod gini;

#[derive(Debug)]
enum Condition {
    Equals(usize),
    LessOrEqual(f64),
    Greater(f64),
}
#[derive(Debug)]
struct Branch {
    condition: Condition,
    child: NodeId,
}
#[derive(Debug)]
enum Node {
    Internal {
        feature: usize,
        branches: Vec<Branch>,
    },
    Leaf {
        label: usize,
    }
}
#[derive(Debug)]
struct NodeId(usize);

struct Tree { root: Option<NodeId>, nodes: Vec<Node> }

fn main() {

}
