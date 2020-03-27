use aoc2019::aoc_input::get_input;
use itertools::iproduct;
use num_integer::gcd;
use num_rational::Rational;
use std::cmp::{Ord, Ordering};
use std::collections::{HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
enum Position {
    Empty,
    Asteroid,
}

impl TryFrom<char> for Position {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Position::Empty),
            '#' => Ok(Position::Asteroid),
            _ => Err("Invalid character encountered"),
        }
    }
}

struct Map {
    grid: Vec<Vec<Position>>,
}

type Coord = (isize, isize);

impl Map {
    fn height(&self) -> isize {
        self.grid.len() as isize
    }

    fn width(&self) -> isize {
        self.grid[0].len() as isize
    }

    fn get(&self, (x, y): Coord) -> Option<Position> {
        if x < 0 || y < 0 {
            return None;
        }

        let (x, y) = (x as usize, y as usize);
        let row = self.grid.get(y)?;
        let pos = row.get(x)?;
        Some(*pos)
    }

    fn iter_coordinates(&self) -> impl Iterator<Item = Coord> {
        iproduct!(0..self.width(), 0..self.height())
    }

    fn iter_coordinates_containing<'a>(&'a self, p: Position) -> impl Iterator<Item = Coord> + 'a {
        self.iter_coordinates()
            .filter(move |&coord| self.get(coord).unwrap() == p)
    }
}

impl TryFrom<&str> for Map {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let lines: Vec<_> = s.trim().lines().collect();
        let height = lines.len();
        if height == 0 {
            return Err("No lines provided");
        }

        let mut grid: Vec<Vec<Position>> = Vec::with_capacity(height);
        for line in lines.iter() {
            let row: Result<Vec<Position>, _> = line.chars().map(|c| c.try_into()).collect();
            grid.push(row?);
        }

        let width = grid[0].len();
        if !grid.iter().all(|row| row.len() == width) {
            return Err("Line length is not uniform");
        }

        Ok(Map { grid })
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum RayDirectionType {
    YAxisUp = 0,
    RightHalfPlane = 1,
    YAxisDown = 2,
    LeftHalfPlane = 3,
}

impl PartialOrd for RayDirectionType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RayDirectionType {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
}

impl TryFrom<&Coord> for RayDirectionType {
    type Error = &'static str;

    fn try_from(ray: &Coord) -> Result<Self, Self::Error> {
        // Note: the Y coordinate grows downwards visually
        match (ray.0.signum(), ray.1.signum()) {
            (0, -1) => Ok(RayDirectionType::YAxisUp),
            (1, _) => Ok(RayDirectionType::RightHalfPlane),
            (0, 1) => Ok(RayDirectionType::YAxisDown),
            (-1, _) => Ok(RayDirectionType::LeftHalfPlane),
            _ => Err("Invalid direction"),
        }
    }
}

fn compare_rays_clockwise_top(lhs: &Coord, rhs: &Coord) -> Ordering {
    let lhs_dir: RayDirectionType = lhs.try_into().unwrap();
    let rhs_dir: RayDirectionType = rhs.try_into().unwrap();

    // Let the coarse RayDirectionType comparison do the heavy lifting, then do
    // the sub-comparisons within the half-planes when the result calls for it
    match (lhs_dir.cmp(&rhs_dir), lhs_dir) {
        (Ordering::Equal, RayDirectionType::RightHalfPlane)
        | (Ordering::Equal, RayDirectionType::LeftHalfPlane) => {
            let lhs_slope = Rational::new(lhs.1, lhs.0);
            let rhs_slope = Rational::new(rhs.1, rhs.0);
            // Lower slopes should come earlier (note that the Y coordinate
            // grows downwards visually)
            lhs_slope.cmp(&rhs_slope)
        }
        (dir_cmp_res, _) => dir_cmp_res,
    }
}

fn rays(map: &Map) -> Vec<Coord> {
    let h = map.height() as isize;
    let w = map.width() as isize;

    let mut res = HashSet::<Coord>::new();
    for (x, y) in iproduct!(-h + 1..h, -w + 1..w).filter(|t| t != &(0, 0)) {
        let g = gcd(x, y);
        res.insert((x / g, y / g));
    }

    let mut res: Vec<_> = res.iter().cloned().collect();
    res.sort_by(compare_rays_clockwise_top);
    res
}

fn asteroids_in_direction(map: &Map, ray: Coord, loc: Coord) -> VecDeque<Coord> {
    let mut asteroids = VecDeque::<Coord>::new();
    for step in 1.. {
        let coord = (loc.0 + step * ray.0, loc.1 + step * ray.1);
        match map.get(coord) {
            None => break,
            Some(Position::Asteroid) => asteroids.push_back(coord),
            Some(Position::Empty) => (),
        }
    }
    asteroids
}

fn count_asteroids_in_line_of_sight<'a>(
    map: &Map,
    rays: impl IntoIterator<Item = &'a Coord>,
    loc: Coord,
) -> usize {
    rays.into_iter()
        .filter(|&&ray| !asteroids_in_direction(&map, ray, loc).is_empty())
        .count()
}

fn find_ith_annihilated_asteroid(
    map: &Map,
    rays: &Vec<Coord>,
    station_location: Coord,
    idx: usize,
) -> Coord {
    let mut asteroids_by_ray: Vec<_> = rays
        .iter()
        .map(|r| asteroids_in_direction(&map, *r, station_location))
        .collect();
    let mut laser_round_robin = Vec::<Coord>::new();

    loop {
        let mut found_anything = false;
        for ray_asteroids in asteroids_by_ray.iter_mut() {
            if ray_asteroids.is_empty() {
                continue;
            }

            found_anything = true;
            laser_round_robin.push(ray_asteroids.pop_front().unwrap())
        }
        if !found_anything {
            break;
        }
    }

    laser_round_robin[idx]
}

fn main() {
    let input = get_input(10);

    let map: Map = input.as_str().try_into().expect("Failed parsing map");

    let rays = rays(&map);
    let (station_location, asteroids_in_los) = map
        .iter_coordinates_containing(Position::Asteroid)
        .map(|loc| (loc, count_asteroids_in_line_of_sight(&map, &rays, loc)))
        .max_by_key(|(_loc, count)| *count)
        .unwrap();
    println!("Station location: {:?}", station_location);
    println!("Station asteroids in line of sight: {}", asteroids_in_los);

    let one_based_idx = 200;
    println!(
        "Asteroid #{} to be annihilated: {:?}",
        one_based_idx,
        find_ith_annihilated_asteroid(&map, &rays, station_location, one_based_idx - 1)
    );
}
