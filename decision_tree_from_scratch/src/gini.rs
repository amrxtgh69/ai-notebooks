use std::collections::HashMap;

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
pub fn gini_impurity(label: &[usize], indices: &[usize]) -> f64 {
    let mut counts: HashMap<usize, usize>= HashMap::new();

    // creating the frequency table
    for &i in indices {
        let class = label[i];
        *counts.entry(class).or_insert(0) += 1;
    }
    let total = indices.len() as f64;
    if total <= 1.0 { return 0.0 }

    let mut sum_prob: f64 = 0.0;
    
    for &count in counts.values() {
        let p = count as f64 / total as f64;
        sum_prob += p * p;
    }
    1.0 - sum_prob
}
