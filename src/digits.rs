struct DigitsIterator {
    n: usize,
    radix: usize,
}

impl DigitsIterator {
    fn new(n: usize, radix: usize) -> DigitsIterator {
        DigitsIterator { n, radix }
    }
}

impl Iterator for DigitsIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n != 0 {
            let digit = self.n % self.radix;
            self.n /= self.radix;
            Some(digit)
        } else {
            None
        }
    }
}

pub fn digits(n: usize, radix: usize) -> Vec<usize> {
    DigitsIterator::new(n, radix).collect()
}
