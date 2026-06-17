use crate::gini::impurity;

/// ~amrxtgh
/// Finds the best split threshold for ONE feature at the current node
/// using the CART algorithm (Classification And Regression Tree).
///
/// Context:
/// We are currently standing at a single node of the decision tree.
/// This node already owns a subset of rows from the original dataset.
///
/// Example dataset:
///
/// row   age   salary   label
/// 0      20     30       0
/// 1      25     35       0
/// 2      40     80       1
/// 3      45     90       1
///
/// Suppose the current node contains:
///
/// indices = [0, 1, 2, 3]
///
/// If feature = 0 (age),
/// we evaluate possible thresholds:
///
/// age <= threshold  → left child
/// age > threshold   → right child
///
/// For every valid threshold:
/// - Partition rows into left and right groups
/// - Compute weighted Gini impurity
/// - Keep the threshold with the lowest impurity
///
/// Example:
/// threshold = 32.5
///
/// left:
/// rows [0,1]
///
/// right:
/// rows [2,3]
///
/// weighted_gini = 0.0
///
/// Returns:
/// Some((best_threshold, lowest_gini))
///
/// Returns None when:
/// - all values for the feature are identical
/// - no valid split exists
fn best_split_for_feature(
    data: &[Vec<f64>],
    labels: &[usize],
    indices: &[usize],
    feature: usize
) -> Option<(f64, f64)> {

    let mut values = Vec::new();
    for &rows in indices {
        let feature_values = data[rows][feature];

        values.push((
            feature_values,
            rows
        ));
    }
    values.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());
    let mut best_threshold = None;
    let mut best_gini = f64::INFINITY;

    let n = indices.len();

    for i in 0..values.len()-1 {
        let current = values[i].0;
        let next = values[i+1].0;

        if current == next { continue; }
        
        let threshold = current+next / 2.0;
        
        let mut left = Vec::new();
        let mut right = Vec::new();
        
        for &(values, row) in &values {
            if values <= threshold {
                left.push(row);
            } else {
                right.push(row);
            }
        }
        if left.is_empty() || right.is_empty() {
            continue;
        }
        let left_gini = impurity(labels, &left);
        let right_gini = impurity(labels, &right);

        let score = (left.len() as f64 / n as f64) * left_gini + (right.len() as f64 / n as f64) * right_gini;

        if score < best_gini {
            best_gini = score;
            best_threshold = Some(threshold);
        }
    }
    best_threshold.map(|t| (t, best_gini))
}



