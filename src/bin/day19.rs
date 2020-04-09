use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;

fn main() {
    let input = get_input(19);
    let tape = parse_intcode_program(&input);

    let mut pulled_locations = 0;

    for y in 0..50 {
        for x in 0..50 {
            let mut machine = IntcodeMachine::new(tape.clone());
            machine.input.borrow_mut().extend(&[x, y]);
            machine.run().unwrap();
            pulled_locations += machine.output.borrow_mut().pop_front().unwrap();
        }
    }

    println!("Pulled locations: {}", pulled_locations);
}
