use aoc2019::aoc_input::get_input;
use regex::Regex;
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

#[derive(Debug)]
struct Simulation {
    time: usize,
    bodies: Vec<Option<Body>>,
}

impl Simulation {
    fn new(bodies: Vec<Body>) -> Self {
        Simulation {
            time: 0,
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
        self.time += 1;
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
}

fn parse_input(input: &str) -> Vec<Body> {
    input
        .trim()
        .lines()
        .map(|line| Body::new_at_rest(line.parse().unwrap()))
        .collect()
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
}
