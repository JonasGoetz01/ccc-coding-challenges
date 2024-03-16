use std::env;

// The objective here is to calculate the perimeter of the
// resulting koch snowflake in each iteration for the given length
// of equilateral triangle.
fn calculate_perimeter(length: f64, iterations: u32) -> f64 {
    let mut perimeter = 3.0 * length;
    for _ in 0..iterations {
        perimeter *= 4.0 / 3.0;
    }
    perimeter
}

// The objective here is to calculate the perimeter of the
// resulting koch snowflake in each iteration for the given length
// with a square instead of a triangle.
// of equilateral square.
fn calculate_perimeter_sq(length: f64, iterations: u32) -> f64 {
    let mut perimeter = 4.0 * length;
    for _ in 0..iterations {
        perimeter *= 5.0 / 3.0;
    }
    perimeter
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let length: f64 = args[1].parse().expect("Invalid length");
    let iterations: u32 = args[2].parse().expect("Invalid iterations");

    let mut perimeter = calculate_perimeter(length, iterations);
    println!("Perimeter: {}", perimeter);

    perimeter = calculate_perimeter_sq(length, iterations);
    println!("Perimeter with square: {}", perimeter);
}
