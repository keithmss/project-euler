mod palindrome;

use palindrome::Palindrome;
use rayon::prelude::*;

static FACTOR_LENGTH: usize = 3;
static MAXIMUM: u32 = 98_0001;
static MINIMUM: u32 = 10_000;

fn main() {
    // Find, in parallel, the highest value palindrome which can be made from
    // the product of two `FACTOR_LENGTH` values.
    let palindrome = (MINIMUM..MAXIMUM)
        .into_par_iter()
        .filter(|n| !is_prime(*n))
        .filter_map(|n| Palindrome::from(n, Some(FACTOR_LENGTH)))
        .find_last(|palindrome| palindrome.verify());

    // Print the value of the palindrome.
    if let Some(palindrome) = palindrome {
        println!("{}", palindrome.value());
    }
}

/// Determine whether or not a number, `input`, is prime.
fn is_prime(input: u32) -> bool {
    (2..input).all(|n| input % n != 0)
}
