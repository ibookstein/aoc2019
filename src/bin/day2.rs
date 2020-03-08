use aoc2019::aoc_input::get_input;
use aoc2019::intcode::{IntcodeMachine, Tape};

fn run_program(original: &Tape, noun: usize, verb: usize) -> usize {
    let mut tape = original.clone();
    tape[1] = noun;
    tape[2] = verb;

    let mut machine = IntcodeMachine::new(tape);
    machine.run();

    machine.tape[0]
}

fn find_preimage(original: &Tape, preimage: usize) -> Option<(usize, usize)> {
    for noun in 0..100 {
        for verb in 0..100 {
            if run_program(&original, noun, verb) == preimage {
                return Some((noun, verb));
            }
        }
    }
    None
}

fn main() {
    let input = get_input(2);
    let original_tape: Tape = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    println!(
        "run_program(12, 2) = {}",
        run_program(&original_tape, 12, 2)
    );

    let res = find_preimage(&original_tape, 19690720);
    let (noun, verb) = res.expect("Failed finding preimage");
    println!("100 * noun + verb = {}", 100 * noun + verb);
}
