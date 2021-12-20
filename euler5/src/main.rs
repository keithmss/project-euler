use rayon::prelude::*;

static FACTOR: u32 = 20;

fn main() {
    // Find, in parallel, the first number that is divisible by 1..`FACTOR`.
    let answer = (FACTOR..u32::MAX)
        .into_par_iter()
        .step_by(FACTOR as usize)
        .find_first(|number| (2..FACTOR).all(|factor| number % factor == 0));

    // Print the answer.
    if let Some(a) = answer {
        println!("{}", a);
    }
}
