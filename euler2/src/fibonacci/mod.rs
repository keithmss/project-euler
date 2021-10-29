use std::iter::Iterator;

/// Structure containing two Fibonacci values.
pub(super) struct Fibonacci {
    a: usize,
    b: usize,
}

impl Fibonacci {
    /// Create a new `Fibonacci` instance with values initiated: 1, 2
    pub(super) fn new() -> Self {
        let a = 2;
        let b = 1;
        Self { a, b }
    }
}

/// Iterator implementation for Fibonacci
impl Iterator for Fibonacci {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.b;
        self.b = self.a;
        self.a += next;
        Some(next)
    }
}
