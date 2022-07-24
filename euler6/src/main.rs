use rayon::prelude::*;

static LIMIT: u32 = 100;

fn main() {
    // Determine, in parallel, the sum of 1..LIMIT and the sum of the squares
    // of 1..Limit.
    let (sum, squares) = (1..LIMIT + 1)
        .into_par_iter()
        .map(|number| (number, number.pow(2)))
        .reduce(
            || (0, 0),
            |number, accumulator| {
                let base = number.0 + accumulator.0;
                let square = number.1 + accumulator.1;
                (base, square)
            },
        );

    // Determine the difference of the sum of 1..LIMIT squared and the sum of
    // the squares of 1..LIMIT.
    let answer = sum.pow(2) - squares;
    println!("{answer}");
}
