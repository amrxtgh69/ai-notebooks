use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
struct Instance {
    features: HashMap<String, String>,
}

#[derive(Debug)]
struct ConceptLearner {
    hypothesis: HashSet<String>,
}

impl ConceptLearner {
    fn new() -> Self {
        Self {
            hypothesis: HashSet::new(),
        }
    }

    fn fit(&mut self, examples: &[(Instance, bool)]) {
        self.hypothesis.clear();

        for (instance, label) in examples {
            if !label {
                continue;
            }

            if self.hypothesis.is_empty() {
                self.hypothesis = instance.features.keys().cloned().collect();
            } else {
                self.generalize(instance);
            }
        }
    }

    fn generalize(&mut self, instance: &Instance) {
        self.hypothesis
            .retain(|feature| instance.features.contains_key(feature));
    }

    fn predict(&self, instance: &Instance) -> bool {
        self.hypothesis
            .iter()
            .all(|feature| instance.features.contains_key(feature))
    }
}

fn create_test_data() -> Vec<(Instance, bool)> {
    let mut data = Vec::new();

    data.push((
        Instance {
            features: [
                ("outlook".to_string(), "sunny".to_string()),
                ("temp".to_string(), "hot".to_string()),
                ("humidity".to_string(), "high".to_string()),
                ("wind".to_string(), "weak".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        },
        true,
    ));

    data.push((
        Instance {
            features: [
                ("outlook".to_string(), "sunny".to_string()),
                ("temp".to_string(), "hot".to_string()),
                ("humidity".to_string(), "high".to_string()),
                ("wind".to_string(), "strong".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        },
        false,
    ));

    data.push((
        Instance {
            features: [
                ("outlook".to_string(), "overcast".to_string()),
                ("temp".to_string(), "hot".to_string()),
                ("humidity".to_string(), "high".to_string()),
                ("wind".to_string(), "weak".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        },
        true,
    ));

    data.push((
        Instance {
            features: [
                ("outlook".to_string(), "rain".to_string()),
                ("temp".to_string(), "mild".to_string()),
                ("humidity".to_string(), "high".to_string()),
                ("wind".to_string(), "strong".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        },
        true,
    ));

    data.push((
        Instance {
            features: [
                ("outlook".to_string(), "rain".to_string()),
                ("temp".to_string(), "cool".to_string()),
                ("humidity".to_string(), "normal".to_string()),
                ("wind".to_string(), "strong".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        },
        true,
    ));

    data.push((
        Instance {
            features: [
                ("outlook".to_string(), "rain".to_string()),
                ("temp".to_string(), "cool".to_string()),
                ("humidity".to_string(), "normal".to_string()),
                ("wind".to_string(), "weak".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        },
        true,
    ));

    data.push((
        Instance {
            features: [
                ("outlook".to_string(), "overcast".to_string()),
                ("temp".to_string(), "mild".to_string()),
                ("humidity".to_string(), "high".to_string()),
                ("wind".to_string(), "strong".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        },
        true,
    ));

    data.push((
        Instance {
            features: [
                ("outlook".to_string(), "overcast".to_string()),
                ("temp".to_string(), "hot".to_string()),
                ("humidity".to_string(), "normal".to_string()),
                ("wind".to_string(), "weak".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        },
        true,
    ));

    data.push((
        Instance {
            features: [
                ("outlook".to_string(), "rain".to_string()),
                ("temp".to_string(), "mild".to_string()),
                ("humidity".to_string(), "normal".to_string()),
                ("wind".to_string(), "weak".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        },
        false,
    ));

    data
}

fn main() {
    let mut learner = ConceptLearner::new();
    let test_data = create_test_data();

    println!("Training with {} examples...", test_data.len());
    learner.fit(&test_data);

    println!("Final hypothesis: {:?}", learner.hypothesis);
    println!("\nPredictions:");

    for (instance, label) in test_data {
        let prediction = learner.predict(&instance);
        println!(
            "  {:?} -> True: {}, Predicted: {}",
            instance.features, label, prediction
        );
    }
}
