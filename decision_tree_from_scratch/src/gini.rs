// In supervised learning we have dataset (X, y)
// where:
// X = feature vectors (x1, x2, x3, ...)
// y = labels (output classes)
//
// Example:
// label = [0, 1, 1, 0, 1]
// where label[i] is the class of i-th sample
//
// `indices` represents a subset of rows in the dataset
// (used to define the current node in the decision tree)
//
// This function computes the Gini impurity of that subset:
// It measures how mixed the class labels are in that node,
// not classification correctness.
pub fn impurity(label: &[usize], indices: &[usize]) -> f64 {
    let mut counts = Vec::new();

    // creating the frequency table
    for &i in indices {
        let class = label[i];
        if class >= counts.len() {
            counts.resize(class + 1,0);
        }
        counts[class] += 1;
    }
    if indices.len() <= 1 { return 0.0; }
    let total = indices.len() as f64;

    let mut sum_squares: f64 = 0.0;
    
    for &count in counts.iter() {
        let p = count as f64 / total as f64;
        sum_squares += p * p;
    }
    1.0 - sum_squares
}
