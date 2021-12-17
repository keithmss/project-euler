use std::iter::Iterator;

pub struct PrimeFactor {
    remainder: usize,
    divisor: usize,
    factor: usize,
}

impl PrimeFactor {
    pub fn new(remainder: usize) -> Self {
        let divisor = 2;
        let factor = 0;
        Self {
            remainder,
            divisor,
            factor,
        }
    }
}

impl Iterator for PrimeFactor {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let previous = self.factor;

        while self.factor == previous {
            if self.divisor.pow(2) <= self.remainder {
                if self.remainder % self.divisor == 0 {
                    self.remainder /= self.divisor;
                    self.factor = self.divisor;
                } else {
                    self.divisor += 1;
                }
            } else {
                return None;
            }
        }

        if self.remainder > self.factor {
            self.factor = self.remainder;
        }
        Some(self.factor)
    }
}
