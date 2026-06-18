use perceptron::*;

fn main() {
    println!("AND gate:");
    let mut p = Perceptron::new(2, 0.1);
    let data = and_data();
    p.fit(&data, 10);
    for (inputs, target) in &data {
        println!(
            "  {:?} -> {} (expected {})",
            inputs,
            p.predict(inputs),
            target
        );
    }

    println!("\nXOR gate (will fail - not linearly separable):");
    let mut p = Perceptron::new(2, 0.1);
    let data = xor_data();
    p.fit(&data, 1000);
    for (inputs, target) in &data {
        println!(
            "  {:?} -> {} (expected {})",
            inputs,
            p.predict(inputs),
            target
        );
    }

    println!("\nOR gate:");
    let mut p = Perceptron::new(2, 0.1);
    let data = or_data();
    p.fit(&data, 10);
    for (inputs, target) in &data {
        println!(
            "  {:?} -> {} (expected {})",
            inputs,
            p.predict(inputs),
            target
        );
    }
}
