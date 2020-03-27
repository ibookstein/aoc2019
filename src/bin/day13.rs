use itertools::Itertools;
use std::collections::HashMap;
#[macro_use]
extern crate num_derive;
use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;
use num_traits::FromPrimitive;
use std::cmp::max;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

type Coordinate = (isize, isize);

#[derive(Debug)]
struct SegmentDisplay {
    input: StreamRef,
    score: isize,
    max_x: isize,
    max_y: isize,
    blocks: HashMap<Coordinate, Tile>,
}

impl SegmentDisplay {
    fn new(input: StreamRef) -> Self {
        SegmentDisplay {
            input,
            score: 0,
            max_x: 0,
            max_y: 0,
            blocks: HashMap::new(),
        }
    }

    fn update(&mut self) {
        for (x, y, b) in self.input.borrow_mut().drain(..).tuples() {
            let coordinate = (x, y);
            match coordinate {
                (-1, 0) => {
                    self.score = b;
                }
                _ => {
                    self.max_x = max(self.max_x, x);
                    self.max_y = max(self.max_y, y);
                    let tile = Tile::from_isize(b).unwrap();
                    self.blocks.insert(coordinate, tile);
                }
            };
        }
    }

    fn count_tiles_matching(&self, tile: Tile) -> usize {
        self.blocks.values().filter(|v| **v == tile).count()
    }

    fn find_location(&self, tile: Tile) -> Option<Coordinate> {
        self.blocks
            .iter()
            .filter(|(_, value)| **value == tile)
            .map(|(key, _)| *key)
            .next()
    }
}

impl fmt::Display for SegmentDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let ch = match self.blocks.get(&(x, y)) {
                    None | Some(Tile::Empty) => ' ',
                    Some(Tile::Wall) => '*',
                    Some(Tile::Block) => '+',
                    Some(Tile::Paddle) => '=',
                    Some(Tile::Ball) => 'o',
                };
                write!(f, "{}", ch)?;
            }
            if y == 0 {
                writeln!(f, " Score: {}", self.score)?;
            } else if y < self.max_y {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Joystick {
    output: StreamRef,
}

impl Joystick {
    fn new(output: StreamRef) -> Self {
        Joystick { output }
    }

    fn set(&mut self, direction: isize) {
        self.output.borrow_mut().push_back(direction.signum());
    }
}

#[derive(Debug)]
struct ArcadeMachine {
    machine: IntcodeMachine,
    display: SegmentDisplay,
    joystick: Joystick,
}

impl ArcadeMachine {
    fn new(tape: Tape) -> Self {
        let machine = IntcodeMachine::new(tape);
        let display = SegmentDisplay::new(machine.output.clone());
        let joystick = Joystick::new(machine.input.clone());
        ArcadeMachine {
            machine,
            display,
            joystick,
        }
    }

    fn run(&mut self) -> StopStatus {
        let status = self.machine.run().unwrap();
        self.display.update();
        status
    }

    fn run_to_completion(&mut self) {
        self.machine.run_to_completion().unwrap();
        self.display.update();
    }
}

struct Bot {
    current_ball_location: Coordinate,
    paddle_location: Coordinate,
}

impl Bot {
    fn new() -> Self {
        Bot {
            current_ball_location: (0, 0),
            paddle_location: (0, 0),
        }
    }

    fn update_game_state(&mut self, display: &SegmentDisplay) {
        self.current_ball_location = display.find_location(Tile::Ball).unwrap();
        self.paddle_location = display.find_location(Tile::Paddle).unwrap();
    }

    fn recommend_move(&mut self, display: &SegmentDisplay) -> isize {
        self.update_game_state(display);
        self.current_ball_location.0 - self.paddle_location.0
    }
}

fn count_block_tiles(tape: Tape) -> usize {
    let mut arcade = ArcadeMachine::new(tape);
    arcade.run_to_completion();

    arcade.display.count_tiles_matching(Tile::Block)
}

fn get_user_move() -> isize {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let ch = input.chars().next().unwrap_or(' ');
    if "qwertasdfgzxcvb".contains(ch) {
        -1
    } else if "yuiop[]\\hjkl;'nm,./".contains(ch) {
        1
    } else {
        0
    }
}

fn winning_score(mut tape: Tape, show: bool, interactive: bool) -> isize {
    // Play for free
    tape[0] = 2;
    let mut arcade = ArcadeMachine::new(tape);
    let mut bot = Bot::new();

    while arcade.run() == StopStatus::BlockedOnInput {
        if show {
            println!("{}", arcade.display);
            if !interactive {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }

        let direction = if interactive {
            get_user_move()
        } else {
            bot.recommend_move(&arcade.display)
        };

        arcade.joystick.set(direction);
    }
    arcade.display.score
}

fn main() {
    let input = get_input(13);
    let program = parse_intcode_program(&input);

    println!("Block tiles count: {}", count_block_tiles(program.clone()));
    println!("Winning score: {}", winning_score(program, false, false));
}
