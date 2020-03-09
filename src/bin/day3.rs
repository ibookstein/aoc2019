use aoc2019::aoc_input::get_input;
use std::cmp::min;
use std::collections::HashMap;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

struct Segment {
    direction: Direction,
    length: usize,
}

type Wire = Vec<Segment>;
type Board = HashMap<(isize, isize), usize>;

fn parse_direction(ch: char) -> Direction {
    match ch {
        'L' => Direction::Left,
        'R' => Direction::Right,
        'U' => Direction::Up,
        'D' => Direction::Down,
        _ => panic!("Unknown direction"),
    }
}

fn parse_segment(seg_str: &str) -> Segment {
    let mut indices = seg_str.char_indices();
    let first_ch = indices.next().expect("No chars in string").1;
    let rest = &seg_str[indices.next().expect("Only one char in string").0..];

    Segment {
        direction: parse_direction(first_ch),
        length: rest.parse().expect("Non-integer after first char"),
    }
}

fn parse_line(line: &str) -> Wire {
    line.split(',').map(parse_segment).collect()
}

fn traverse_wire<F>(wire: &Wire, mut func: F)
where
    F: FnMut((isize, isize), usize),
{
    let mut cursor: (isize, isize) = (0, 0);
    let mut steps: usize = 0;
    for seg in wire {
        let delta: (isize, isize) = match seg.direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
        };
        for _ in 0..seg.length {
            cursor = (cursor.0 + delta.0, cursor.1 + delta.1);
            steps += 1;
            func(cursor, steps);
        }
    }
}

fn solve(wire1: &Wire, wire2: &Wire) {
    let mut board = Board::new();
    let mut min_distance = std::usize::MAX;
    let mut min_signal_delay = std::usize::MAX;

    traverse_wire(&wire1, |cursor, steps| {
        board.insert(cursor, steps);
    });
    traverse_wire(&wire2, |cursor, steps| {
        let other_wire_steps = match board.get(&cursor) {
            None => return,
            Some(n) => *n,
        };

        min_distance = min(min_distance, (cursor.0.abs() + cursor.1.abs()) as usize);
        min_signal_delay = min(min_signal_delay, steps + other_wire_steps);
    });

    println!("Minimum distance: {}", min_distance);
    println!("Minimum signal delay: {}", min_signal_delay);
}

fn main() {
    let input = get_input(3);
    let wires: Vec<_> = input.trim().lines().map(parse_line).collect();

    if let [wire1, wire2] = &wires[..] {
        solve(&wire1, &wire2);
    } else {
        println!("Incorrect number of wires: {}", wires.len())
    }
}
