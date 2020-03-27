use itertools::Itertools;
use std::collections::HashMap;
#[macro_use]
extern crate num_derive;
use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;
use num_traits::FromPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

fn count_block_tiles(program: Tape) -> usize {
    let mut machine = IntcodeMachine::new(program);
    machine
        .run_to_completion()
        .expect("Did not run to completion");

    let mut board = HashMap::<(isize, isize), Tile>::new();
    for (x, y, b) in machine.output.borrow().iter().tuples() {
        let key = (*x, *y);
        let value = Tile::from_isize(*b).expect("Bad tile number");
        board.insert(key, value);
    }

    board.values().filter(|v| **v == Tile::Block).count()
}

fn main() {
    let input = get_input(13);
    let program = parse_intcode_program(&input);

    println!("Block tiles count: {}", count_block_tiles(program.clone()));
}
