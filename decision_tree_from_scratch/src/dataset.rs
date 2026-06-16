/// ~amrxtgh: dummy dataset with no hashmap with only two contiguous Vecs
pub struct Dataset {
    pub data: Vec<Vec<f64>>, //rows: sample, cols: feature
    pub targets: Vec<usize>, // aligned with rows
}
impl Dataset {
    pub fn new(data: Vec<Vec<f64>>, targets: Vec<usize>) -> Result<Self, &'static str> {
        if data.is_empty() {
            return Err("dataset cannot be empty");
        }
        if data[0].is_empty() {
            return Err("each sample must have at least one feature");
        }
        if data.len() != targets.len() {
            return Err("data len and the target len must match");
        }
        let feature_len = data[0].len();
        for row in &data {
            if row.len() != feature_len {
                return Err("all rows must have same number of features");
            }
        }
        Ok(Self { data, targets })
    }
}