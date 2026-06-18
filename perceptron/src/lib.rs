pub struct Perceptron {
    weight: Vec<f64>,
    bias: f64,
    lr: f64,
}

impl Perceptron {
    pub fn new(n_features: usize, lr: f64) -> Self {
        Self {
            weight: vec![0.0; n_features],
            bias: 0.0,
            lr,
        }
    }

    pub fn predict(&self, inputs: &[f64]) -> i32 {
        let sum: f64 = inputs
            .iter()
            .zip(&self.weight)
            .map(|(x, w)| x * w)
            .sum::<f64>()
            + self.bias;
        if sum >= 0.0 {
            1
        } else {
            -1
        }
    }

    pub fn train(&mut self, inputs: &[f64], target: i32) {
        let pred = self.predict(inputs);
        let error = target - pred;
        if error != 0 {
            for (w, x) in self.weight.iter_mut().zip(inputs) {
                *w += self.lr * error as f64 * x;
            }
            self.bias += self.lr * error as f64;
        }
    }

    pub fn fit(&mut self, data: &[(Vec<f64>, i32)], epochs: usize) {
        for _ in 0..epochs {
            for (inputs, target) in data {
                self.train(inputs, *target);
            }
        }
    }
}

pub fn and_data() -> Vec<(Vec<f64>, i32)> {
    vec![
        (vec![0.0, 0.0], -1),
        (vec![0.0, 1.0], -1),
        (vec![1.0, 0.0], -1),
        (vec![1.0, 1.0], 1),
    ]
}

pub fn xor_data() -> Vec<(Vec<f64>, i32)> {
    vec![
        (vec![0.0, 0.0], -1),
        (vec![0.0, 1.0], 1),
        (vec![1.0, 0.0], 1),
        (vec![1.0, 1.0], -1),
    ]
}

pub fn or_data() -> Vec<(Vec<f64>, i32)> {
    vec![
        (vec![0.0, 0.0], -1),
        (vec![0.0, 1.0], 1),
        (vec![1.0, 0.0], 1),
        (vec![1.0, 1.0], 1),
    ]
}
