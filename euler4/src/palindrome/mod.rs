use itertools::Itertools;
use std::fmt::Display;

/// `Palindrome` structure containing a `value` and its `factors`.
pub struct Palindrome {
    value: u32,
    factors: Vec<u32>,
}

impl Palindrome {
    /// Build a `Palindrome` from a given `value`.
    pub fn from(value: u32, factor_length: Option<usize>) -> Option<Self> {
        // Verify that the `value` is a palindrome.
        let value = is_palindrome(value).then(|| value)?;

        // Retrieve the factors (with optional length) for the `value`.
        let factors = get_factors(value, factor_length)?;

        // Return the `Palindrome`.
        let palindrome = Self { value, factors };
        Some(palindrome)
    }

    /// Retrieve the `Palindrome` `value`.
    pub fn value(&self) -> u32 {
        self.value
    }

    /// Verify that the product of *two* of the `factors` are equal to the
    /// `value` of the `Palindrome`.  
    pub fn verify(&self) -> bool {
        self.factors.iter().cartesian_product(&self.factors).any(|f| f.0 * f.1 == self.value)
    }
}

/// Retrieve the factors (with optional length) for the `input`.
fn get_factors(input: u32, length: Option<usize>) -> Option<Vec<u32>> {
    let factors = match length {
        Some(length) => (1..input + 1)
            .filter(|n| input % n == 0)
            .filter(|n| n.to_string().len() == length)
            .collect::<Vec<u32>>(),
        None => (1..input + 1)
            .filter(|n| input % n == 0)
            .collect::<Vec<u32>>(),
    };
    match factors.is_empty() {
        true => None,
        false => Some(factors),
    }
}

/// Determine whether or not `input` is a valid palindrome.
fn is_palindrome<T: Display>(input: T) -> bool {
    let forward = input.to_string();
    let reverse = forward.chars().rev().collect::<String>();
    forward == reverse
}
