use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;

fn run_program(tape: Tape) -> Stream {
    let mut input = Stream::new();
    input.push_back(1);

    let mut machine = IntcodeMachine::new(tape, input);
    match machine.run() {
        Ok(_) => (),
        Err(err) => panic!("IntcodeMachine error: {:?}", err),
    }

    machine.output
}

fn main() {
    let input = get_input(5);
    let original_tape = parse_intcode_program(&input);
    let output = run_program(original_tape);

    println!("IntcodeMachine output: {:?}", output);
}
