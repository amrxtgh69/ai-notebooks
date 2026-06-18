use perceptron::*;

fn main() {
    println!("AND gate:");
    let mut p = Perceptron::new(2, 0.1);
    let and_data = and_data();
    let and_ref: Vec<(&[f64], i32)> = and_data.iter().map(|(x, y)| (x.as_slice(), *y)).collect();
    p.fit(&and_ref, 10);

    for (inputs, target) in &and_data {
        println!(
            "  {:?} -> {} (expected {})",
            inputs,
            p.predict(inputs),
            target
        );
    }

    println!("\nXOR gate (will fail - not linearly separable):");
    let mut p = Perceptron::new(2, 0.1);
    let xor_data = xor_data();
    let xor_ref: Vec<(&[f64], i32)> = xor_data.iter().map(|(x, y)| (x.as_slice(), *y)).collect();
    p.fit(&xor_ref, 1000);
    for (inputs, target) in &xor_data {
        println!(
            "  {:?} -> {} (expected {})",
            inputs,
            p.predict(inputs),
            target
        );
    }

    println!("\nOR gate:");
    let mut p = Perceptron::new(2, 0.1);
    let or_data = or_data();
    let or_ref: Vec<(&[f64], i32)> = or_data.iter().map(|(x, y)| (x.as_slice(), *y)).collect();
    p.fit(&or_ref, 10);

    for (inputs, target) in &or_data {
        println!(
            "  {:?} -> {} (expected {})",
            inputs,
            p.predict(inputs),
            target
        );
    }
}
