use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    OpenSpace,
    Scaffolding,
}

type Coordinate = (usize, usize);

struct Map {
    grid: Vec<Tile>,
    width: usize,
}

impl Map {
    fn height(&self) -> usize {
        self.grid.len() / self.width
    }

    fn width(&self) -> usize {
        self.width
    }
}

impl Index<Coordinate> for Map {
    type Output = Tile;

    fn index(&self, index: Coordinate) -> &Self::Output {
        self.grid.index(self.width * index.1 + index.0)
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::<Tile>::new();
        let mut width: Option<usize> = None;

        for line in s.trim().lines() {
            let mut row = Vec::<Tile>::with_capacity(line.len());

            for c in line.chars() {
                let tile = match c {
                    '.' => Tile::OpenSpace,
                    '#' | 'v' | '^' | '<' | '>' => Tile::Scaffolding,
                    _ => return Err("Invalid character in string"),
                };
                row.push(tile);
            }

            if width.is_some() && width.unwrap() != row.len() {
                return Err("Line length is not uniform");
            }
            width = Some(row.len());
            grid.extend(row);
        }

        if width.is_none() || width.unwrap() == 0 {
            return Err("Empty string");
        }

        Ok(Map {
            grid,
            width: width.unwrap(),
        })
    }
}

fn sum_alignment_parameters(tape: Tape) -> usize {
    let mut machine = IntcodeMachine::new(tape);
    machine.run_to_completion().unwrap();
    let output: String = machine
        .output
        .borrow_mut()
        .drain(..)
        .map(|n| std::char::from_u32(n as u32).unwrap())
        .collect();

    print!("{}", output);

    let map: Map = output.parse().unwrap();
    let mut alignment = 0usize;
    for y in 1..map.height() - 1 {
        for x in 1..map.width() - 1 {
            if map[(x, y)] != Tile::Scaffolding {
                continue;
            }

            let adjacent = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            if adjacent
                .iter()
                .copied()
                .all(|c| map[c] == Tile::Scaffolding)
            {
                alignment += x * y;
            }
        }
    }
    alignment
}

fn main() {
    let input = get_input(17);
    let tape = parse_intcode_program(&input);

    let alignment = sum_alignment_parameters(tape.clone());
    println!("Sum of alignment parameters: {}", alignment);
}
