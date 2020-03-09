use aoc2019::aoc_input::get_input;

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

fn parse_range(range_str: &str) -> (usize, usize) {
    let nums: Vec<usize> = range_str.split('-').map(|s| s.parse().unwrap()).collect();
    match nums[..] {
        [start, end] => (start, end),
        _ => panic!("Invalid amount of integers in range string"),
    }
}

fn two_same_adjacent_digits(digits: &[usize]) -> bool {
    for w in digits.windows(2) {
        if w[0] == w[1] {
            return true;
        }
    }
    false
}

fn digits_ascending_ltr(digits: &[usize]) -> bool {
    for w in digits.windows(2) {
        if w[0] < w[1] {
            return false;
        }
    }
    true
}

fn two_same_adjacent_digits_exact(digits: &[usize]) -> bool {
    let mut padded = Vec::<Option<usize>>::new();
    padded.push(None);
    for digit in digits {
        padded.push(Some(*digit));
    }
    padded.push(None);

    for w in padded.windows(4) {
        if w[0] != w[1] && w[1] == w[2] && w[2] != w[3] {
            return true;
        }
    }
    false
}

fn main() {
    let input = get_input(4);
    let (start, end) = parse_range(input.trim());

    let mut password_count: usize = 0;
    let mut extra_rule_password_count: usize = 0;

    for current in start..=end {
        let digits: Vec<usize> = DigitsIterator::new(current, 10).collect();
        assert_eq!(digits.len(), 6);
        if !two_same_adjacent_digits(&digits) {
            continue;
        }
        if !digits_ascending_ltr(&digits) {
            continue;
        }

        password_count += 1;

        if !two_same_adjacent_digits_exact(&digits) {
            continue;
        }

        extra_rule_password_count += 1;
    }

    println!("Password candidate count: {}", password_count);
    println!(
        "Password candidate count with extra rule: {}",
        extra_rule_password_count
    );
}
