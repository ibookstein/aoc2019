use std::cmp::{max, min};
use std::collections::HashMap;
use std::convert::From;
use std::ops::{Add, AddAssign};
#[macro_use]
extern crate num_derive;
use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive)]
enum Turn {
    Left = 0,
    Right = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self, to: Turn) -> Self {
        match (*self, to) {
            (Direction::Up, Turn::Right) | (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Up, Turn::Left) | (Direction::Down, Turn::Right) => Direction::Left,
            (Direction::Left, Turn::Right) | (Direction::Right, Turn::Left) => Direction::Up,
            (Direction::Left, Turn::Left) | (Direction::Right, Turn::Right) => Direction::Down,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive, ToPrimitive)]
enum PanelColor {
    Black = 0,
    White = 1,
}

struct Delta {
    dx: isize,
    dy: isize,
}

impl From<Direction> for Delta {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => Delta { dx: 0, dy: -1 },
            Direction::Right => Delta { dx: 1, dy: 0 },
            Direction::Down => Delta { dx: 0, dy: 1 },
            Direction::Left => Delta { dx: -1, dy: 0 },
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn origin() -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }
}

impl Add<Delta> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Delta) -> Self::Output {
        Coordinate {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy,
        }
    }
}

impl AddAssign<Delta> for Coordinate {
    fn add_assign(&mut self, rhs: Delta) {
        *self = *self + rhs;
    }
}

enum RobotRunResult {
    Done,
    Paint(PanelColor),
}

struct Robot {
    brain: IntcodeMachine,
    location: Coordinate,
    direction: Direction,
}

impl Robot {
    fn new(program: Tape) -> Robot {
        Robot {
            brain: IntcodeMachine::new(program, Stream::new()),
            location: Coordinate::origin(),
            direction: Direction::Up,
        }
    }

    fn step(&mut self, location_color: PanelColor) -> RobotRunResult {
        self.brain
            .input
            .push_back(location_color.to_isize().unwrap());
        match self.brain.run().unwrap() {
            StopStatus::Halted => RobotRunResult::Done,
            StopStatus::BlockedOnInput => {
                let paint_request = self.brain.output.pop_front().unwrap();
                let paint_request = PanelColor::from_isize(paint_request).unwrap();
                let turn = self.brain.output.pop_front().unwrap();

                self.direction = self.direction.turn(Turn::from_isize(turn).unwrap());
                self.location += self.direction.into();
                RobotRunResult::Paint(paint_request)
            }
            _ => unreachable!(),
        }
    }
}

struct Board {
    grid: HashMap<Coordinate, PanelColor>,
    robot: Robot,
}

impl Board {
    fn new(robot: Robot, origin_color: PanelColor) -> Board {
        let mut board = Board {
            grid: HashMap::new(),
            robot,
        };
        board.grid.insert(Coordinate::origin(), origin_color);
        board
    }

    fn run_robot(&mut self) {
        loop {
            let entry = self.grid.entry(self.robot.location);
            let current_panel = entry.or_insert(PanelColor::Black);
            match self.robot.step(*current_panel) {
                RobotRunResult::Done => break,
                RobotRunResult::Paint(new_color) => *current_panel = new_color,
            }
        }
    }

    fn painted_panels(&self) -> usize {
        self.grid.len()
    }

    fn render_grid(&self) -> String {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        for key in self.grid.keys() {
            min_x = min(min_x, key.x);
            min_y = min(min_y, key.y);
            max_x = max(max_x, key.x);
            max_y = max(max_y, key.y);
        }

        let mut rendered = String::new();
        for y in min_y..=max_y {
            let mut line = String::new();
            for x in min_x..=max_x {
                let coord = Coordinate { x, y };
                let color = *self.grid.get(&coord).unwrap_or(&PanelColor::Black);
                let ch = match color {
                    PanelColor::Black => ' ',
                    PanelColor::White => '*',
                };
                line.push(ch)
            }
            line.push('\n');
            rendered.push_str(&line);
        }
        rendered
    }
}

fn main() {
    let program = parse_intcode_program(&get_input(11));
    let mut board = Board::new(Robot::new(program.clone()), PanelColor::Black);

    board.run_robot();
    println!("Painted panels: {}", board.painted_panels());

    let mut board = Board::new(Robot::new(program.clone()), PanelColor::White);
    board.run_robot();
    println!(
        "Grid when origin starts colored white:\n{}",
        board.render_grid()
    );
}
