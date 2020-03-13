use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;

fn run_program(tape: &Tape, system_id: isize) -> Stream {
    let mut input = Stream::new();
    input.push_back(system_id);

    let mut machine = IntcodeMachine::new(tape.clone(), input);
    match machine.run_to_completion() {
        Ok(_) => (),
        Err(err) => panic!("IntcodeMachine error: {:?}", err),
    }

    machine.output
}

fn main() {
    let input = get_input(5);
    let tape = parse_intcode_program(&input);

    for system_id in &[1, 5] {
        let output = run_program(&tape, *system_id);
        println!(
            "IntcodeMachine output for System ID {}: {:?}",
            system_id, output
        );
    }
}
