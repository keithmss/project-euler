mod math;

use math::factor::PrimeFactor;

static MAX: usize = 600_851_475_143;
use rayon::prelude::*;

fn main() {
    let factors = PrimeFactor::new(MAX);
    let factor = factors.into_iter().par_bridge().min();
    println!("{}", factor.unwrap());

    // // Determine, in parallel, the sum of even Fibonacci numbers < 4,000,000.
    // let sum: usize = fibonacci
    //     .into_iter()
    //     .take_while(|&value| value < 4_000_000)
    //     .par_bridge()
    //     .filter(|&value| math::is_divisible(value, 2))
    //     .fold(|| 0, |accumulator, value| accumulator + value)
    //     .sum();

    // Print the answer.
    // println!("{}", sum);
}
