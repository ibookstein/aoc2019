use aoc2019::aoc_input::get_input;
use aoc2019::intcode::*;
use std::collections::{HashMap, HashSet};
use std::convert::From;
use std::ops::{Add, AddAssign, Neg};

#[macro_use]
extern crate num_derive;
use num_traits::{FromPrimitive, ToPrimitive};

#[macro_use]
extern crate strum_macros;
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, ToPrimitive, EnumIter)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Neg for Direction {
    type Output = Direction;

    fn neg(self) -> Self::Output {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
enum DroidReply {
    HitWall = 0,
    MovedStep = 1,
    MovedStepFoundOxygenSystem = 2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    OxygenSystem,
}

struct Delta {
    dx: isize,
    dy: isize,
}

impl From<Direction> for Delta {
    fn from(d: Direction) -> Self {
        match d {
            Direction::North => Delta { dx: 0, dy: -1 },
            Direction::East => Delta { dx: 1, dy: 0 },
            Direction::South => Delta { dx: 0, dy: 1 },
            Direction::West => Delta { dx: -1, dy: 0 },
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

#[derive(Debug)]
struct RepairDroid {
    machine: IntcodeMachine,
    map: HashMap<Coordinate, Tile>,
    droid_location: Coordinate,
    oxygen_system_location: Option<Coordinate>,
}

#[derive(Debug)]
enum BfsReply {
    Halt,
    Continue,
}

impl RepairDroid {
    fn new(tape: Tape) -> Self {
        let droid_location = Coordinate::origin();
        let mut map = HashMap::new();
        map.insert(droid_location, Tile::Empty);
        RepairDroid {
            machine: IntcodeMachine::new(tape),
            map,
            droid_location,
            oxygen_system_location: None,
        }
    }

    fn try_move(&mut self, direction: Direction) -> Option<()> {
        let dest = self.droid_location + direction.into();

        let direction = direction.to_isize().unwrap();
        self.machine.input.borrow_mut().push_back(direction);
        self.machine.run().expect("Error running machine");
        let reply = self.machine.output.borrow_mut().pop_front().unwrap();

        match DroidReply::from_isize(reply).unwrap() {
            DroidReply::HitWall => {
                self.map.insert(dest, Tile::Wall);
                None
            }
            DroidReply::MovedStep => {
                self.map.insert(dest, Tile::Empty);
                self.droid_location = dest;
                Some(())
            }
            DroidReply::MovedStepFoundOxygenSystem => {
                self.map.insert(dest, Tile::OxygenSystem);
                self.droid_location = dest;
                self.oxygen_system_location = Some(dest);
                Some(())
            }
        }
    }

    fn discover_recurse(&mut self, return_direction: Option<Direction>) {
        let current_location = self.droid_location;

        for direction in Direction::iter() {
            let dest = current_location + direction.into();
            if self.map.get(&dest).is_some() {
                continue;
            }

            if self.try_move(direction).is_some() {
                self.discover_recurse(Some(-direction));
                assert_eq!(self.droid_location, current_location);
            }
        }

        if let Some(direction) = return_direction {
            self.try_move(direction).unwrap();
        }
    }

    fn discover(&mut self) {
        self.discover_recurse(None)
    }

    fn bfs_layers(
        &self,
        origin: Coordinate,
        mut func: impl FnMut(usize, &HashSet<Coordinate>) -> BfsReply,
    ) {
        let mut visited = HashSet::new();
        visited.insert(origin);
        let mut current_layer = HashSet::new();
        current_layer.insert(origin);
        let mut depth = 0usize;

        while !current_layer.is_empty() {
            match func(depth, &current_layer) {
                BfsReply::Halt => return,
                BfsReply::Continue => (),
            }

            let mut new_layer = HashSet::new();
            for coordinate in current_layer {
                for direction in Direction::iter() {
                    let dest = coordinate + direction.into();
                    let tile = *self.map.get(&dest).unwrap();

                    if visited.contains(&dest) || tile == Tile::Wall {
                        continue;
                    }

                    new_layer.insert(dest);
                }
            }
            visited.extend(&new_layer);
            current_layer = new_layer;
            depth += 1;
        }
    }

    fn distance_from_oxygen_system(&self) -> Option<usize> {
        let oxygen_system_location = self.oxygen_system_location?;
        let mut distance = None;

        self.bfs_layers(self.droid_location, |depth, layer| {
            if layer.contains(&oxygen_system_location) {
                distance = Some(depth);
                BfsReply::Halt
            } else {
                BfsReply::Continue
            }
        });

        distance
    }

    fn time_until_filled_with_oxygen(&self) -> Option<usize> {
        let mut distance = None;

        self.bfs_layers(self.oxygen_system_location?, |depth, _layer| {
            distance = Some(depth);
            BfsReply::Continue
        });
        distance
    }
}

fn main() {
    let tape = parse_intcode_program(&get_input(15));

    let mut droid = RepairDroid::new(tape);
    droid.discover();

    println!(
        "Droid distance from oxygen system: {}",
        droid.distance_from_oxygen_system().unwrap(),
    );

    println!(
        "Time until region fills with oxygen: {}",
        droid.time_until_filled_with_oxygen().unwrap(),
    )
}
