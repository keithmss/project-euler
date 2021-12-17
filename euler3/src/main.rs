mod math;

use math::factor::PrimeFactor;

static MAX: usize = 600_851_475_143;
use rayon::prelude::*;

fn main() {
    let factors = PrimeFactor::new(MAX);
    let factor = factors.into_iter().par_bridge().min();
    println!("{}", factor.unwrap());
}
