use aoc2019::aoc_input::get_input;

type Digit = i8;

fn parse_string_digits(s: &str) -> Vec<Digit> {
    s.chars()
        .map(|c| c.to_digit(10).unwrap() as Digit)
        .collect()
}

fn digits_to_string(digits: &[Digit]) -> String {
    digits
        .iter()
        .map(|n| std::char::from_digit(*n as u32, 10).unwrap())
        .collect()
}

fn fft(signal: &[Digit], transformed: &mut [Digit]) {
    assert_eq!(signal.len(), transformed.len());

    let pattern = [0 as Digit, 1, 0, -1];

    for i in 0..transformed.len() {
        let repetitions = i + 1;

        let mut result: isize = 0;
        for j in i..signal.len() {
            let pattern_idx = ((j + 1) / repetitions) % pattern.len();
            result += (pattern[pattern_idx] as isize) * (signal[j] as isize);
        }

        transformed[i] = (result.abs() % 10) as Digit;
    }
}

fn fft_iterations(start_signal: Vec<Digit>, iterations: usize) -> Vec<Digit> {
    let mut cur_signal = start_signal;
    let mut next_signal: Vec<Digit> = vec![0; cur_signal.len()];

    for _i in 0..iterations {
        fft(&cur_signal, &mut next_signal);
        std::mem::swap(&mut cur_signal, &mut next_signal);
    }
    cur_signal
}

fn find_real_message(
    start_signal: Vec<Digit>,
    iterations: usize,
    message_offset: usize,
    message_length: usize,
) -> Vec<Digit> {
    let n = start_signal.len();
    let mut cur_signal = start_signal;
    let mut next_signal: Vec<Digit> = vec![0; n];

    assert!(message_offset >= n / 2);

    for _i in 0..iterations {
        next_signal[n - 1] = cur_signal[n - 1];

        for j in ((n + 1) / 2..n - 1).rev() {
            next_signal[j] = (next_signal[j + 1] + cur_signal[j]) % 10;
        }

        std::mem::swap(&mut cur_signal, &mut next_signal);
    }

    cur_signal[message_offset..message_offset + message_length].to_vec()
}

fn main() {
    let input = get_input(16);
    let input = input.trim();
    let start_signal = parse_string_digits(input);

    let i = 100;
    let ith_phase = fft_iterations(start_signal, i);
    let digit_count = 8;
    let digits = digits_to_string(&ith_phase[..digit_count]);
    println!("Initial message: {}", digits);

    let message_offset: usize = input[..7].parse().unwrap();
    let start_signal = parse_string_digits(&input.repeat(10000));
    let real_message = find_real_message(start_signal, i, message_offset, digit_count);
    println!("Real message: {}", digits_to_string(&real_message));
}
