use aoc2019::aoc_input::get_input;
use num_integer::lcm;
use regex::Regex;
use std::cmp::max;
use std::num::ParseIntError;
use std::ops::{Add, AddAssign};
use std::str::FromStr;
#[macro_use]
extern crate lazy_static;

const DIMENSIONS: usize = 3;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Velocity {
    dx: [isize; DIMENSIONS],
}

impl Velocity {
    fn zero() -> Self {
        Self {
            dx: [0; DIMENSIONS],
        }
    }

    fn kinetic_energy(&self) -> isize {
        self.dx.iter().map(|c| c.abs()).sum()
    }
}

impl AddAssign<Velocity> for Velocity {
    fn add_assign(&mut self, rhs: Velocity) {
        for i in 0..rhs.dx.len() {
            self.dx[i] += rhs.dx[i];
        }
    }
}

impl Add<Velocity> for Velocity {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    x: [isize; DIMENSIONS],
}

impl Position {
    fn potential_energy(&self) -> isize {
        self.x.iter().map(|c| c.abs()).sum()
    }
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, rhs: Velocity) {
        for i in 0..self.x.len() {
            self.x[i] += rhs.dx[i]
        }
    }
}

impl Add<Velocity> for Position {
    type Output = Self;

    fn add(mut self, rhs: Velocity) -> Self::Output {
        self += rhs;
        self
    }
}

#[derive(Debug)]
enum PositionError {
    RegexUnmatched,
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for PositionError {
    fn from(err: ParseIntError) -> Self {
        PositionError::ParseIntError(err)
    }
}

impl FromStr for Position {
    type Err = PositionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"<x=(?P<x>.*), y=(?P<y>.*), z=(?P<z>.*)>").unwrap();
        }
        let caps = RE.captures(s).ok_or(PositionError::RegexUnmatched)?;
        let x = caps.name("x").unwrap().as_str().parse()?;
        let y = caps.name("y").unwrap().as_str().parse()?;
        let z = caps.name("z").unwrap().as_str().parse()?;
        Ok(Self { x: [x, y, z] })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Body {
    position: Position,
    velocity: Velocity,
}

impl Body {
    fn new_at_rest(position: Position) -> Self {
        Body {
            position,
            velocity: Velocity::zero(),
        }
    }

    fn add_gravity_from(&mut self, other: &Body) {
        let mut velocity = Velocity::zero();
        for i in 0..self.position.x.len() {
            velocity.dx[i] = (other.position.x[i] - self.position.x[i]).signum();
        }
        self.velocity += velocity;
    }

    fn move_by_velocity(&mut self) {
        self.position += self.velocity;
    }

    fn total_energy(&self) -> isize {
        self.position.potential_energy() * self.velocity.kinetic_energy()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Simulation {
    bodies: Vec<Option<Body>>,
}

impl Simulation {
    fn new(bodies: Vec<Body>) -> Self {
        Simulation {
            bodies: bodies.iter().map(|b| Some(*b)).collect(),
        }
    }

    fn tick(&mut self) {
        for i in 0..self.bodies.len() {
            let mut current = self.bodies[i].take().unwrap();
            for other in self.bodies.iter().filter_map(|b| b.as_ref()) {
                current.add_gravity_from(other);
            }
            self.bodies[i].replace(current);
        }
        for body in self.bodies.iter_mut() {
            body.as_mut().unwrap().move_by_velocity();
        }
    }

    fn ticks(&mut self, n: usize) {
        for _ in 0..n {
            self.tick();
        }
    }

    fn total_energy(&self) -> isize {
        self.bodies
            .iter()
            .map(|b| b.as_ref().unwrap().total_energy())
            .sum()
    }

    fn project_axis(&self, axis: usize) -> Self {
        let mut bodies = self.bodies.clone();
        for body in bodies.iter_mut() {
            let body = body.as_mut().unwrap();
            for i in (0..DIMENSIONS).filter(|d| *d != axis) {
                body.position.x[i] = 0;
            }
        }
        Simulation { bodies }
    }
}

fn parse_input(input: &str) -> Vec<Body> {
    input
        .trim()
        .lines()
        .map(|line| Body::new_at_rest(line.parse().unwrap()))
        .collect()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct CycleInfo {
    start_index: usize,
    cycle_length: usize,
}

// https://en.wikipedia.org/wiki/Cycle_detection#Brent's_algorithm
fn brent_cycle_detect(initial_state: &Simulation) -> CycleInfo {
    let mut tortoise = initial_state.clone();
    let mut hare = initial_state.clone();
    hare.tick();

    let mut power = 1usize;
    let mut cycle_length = 1usize;

    while tortoise != hare {
        if power == cycle_length {
            tortoise = hare.clone();
            power *= 2;
            cycle_length = 0;
        }
        hare.tick();
        cycle_length += 1;
    }

    tortoise = initial_state.clone();
    hare = initial_state.clone();
    hare.ticks(cycle_length);

    let mut start_index = 0usize;
    while tortoise != hare {
        tortoise.tick();
        hare.tick();
        start_index += 1;
    }

    CycleInfo {
        start_index: start_index,
        cycle_length: cycle_length,
    }
}

fn compute_time_until_repetition(initial_state: &Simulation) -> usize {
    let cycle_infos: Vec<_> = (0..DIMENSIONS)
        .map(|d| brent_cycle_detect(&initial_state.project_axis(d)))
        .collect();

    let start_index = cycle_infos.iter().fold(0, |acc, x| max(acc, x.start_index));
    let cycle_length = cycle_infos
        .iter()
        .fold(1, |acc, x| lcm(acc, x.cycle_length));
    start_index + cycle_length
}

fn main() {
    let input = get_input(12);

    let mut sim = Simulation::new(parse_input(&input));
    let time_steps = 1000;
    sim.ticks(time_steps);
    println!(
        "Total energy after {} time steps: {}",
        time_steps,
        sim.total_energy()
    );

    let sim = Simulation::new(parse_input(&input));
    println!(
        "Time until repetition: {}",
        compute_time_until_repetition(&sim)
    );
}
