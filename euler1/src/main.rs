mod math;

use rayon::prelude::*;

fn main() {
    // Determine, in parallel, the sum of all numbers divisible by 3 and 5
    // within the range: 0 to 1,000.
    let sum: usize = (0..1000)
        .into_par_iter()
        .filter(|&value| math::is_divisible(value, 3) || math::is_divisible(value, 5))
        .fold(|| 0, |accumulator, value| accumulator + value)
        .sum();

    // Print the answer.
    println!("{}", sum);
}
