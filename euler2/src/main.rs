mod fibonacci;
mod math;

use fibonacci::Fibonacci;
use rayon::prelude::*;

fn main() {
    // Initialize `Fibonacci` structure.
    let fibonacci = Fibonacci::new();

    // Determine, in parallel, the sum of even Fibonacci numbers < 4,000,000.
    let sum: usize = fibonacci
        .into_iter()
        .take_while(|&value| value < 4_000_000)
        .par_bridge()
        .filter(|&value| math::is_divisible(value, 2))
        .fold(|| 0, |accumulator, value| accumulator + value)
        .sum();

    // Print the answer.
    println!("{}", sum);
}
