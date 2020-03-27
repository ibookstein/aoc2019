use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;

fn run_boost(program: &Tape, mode: isize) {
    let mut machine =
        IntcodeMachine::new_io(program.clone(), new_stream_ref_from(mode), new_stream_ref());
    machine
        .run_to_completion()
        .expect("Failed running BOOST program to completion");

    println!(
        "BOOST(mode={}) program output: {:?}",
        mode,
        machine.output.borrow()
    );
}

fn main() {
    let input = get_input(9);
    let program = parse_intcode_program(&input);
    run_boost(&program, 1);
    run_boost(&program, 2);
}
