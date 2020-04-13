use aoc2019::aoc_input::get_input;
use itertools::iproduct;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

type Coordinate = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Bug,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Bug),
            _ => Err("Invalid character"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    grid: Vec<Tile>,
    width: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![Tile::Empty; width * height];
        Grid { grid, width }
    }

    fn height(&self) -> usize {
        self.grid.len() / self.width
    }

    fn width(&self) -> usize {
        self.width
    }

    fn biodiversity_rating(&self) -> usize {
        let mut rating = 0usize;
        for i in 0..self.grid.len() {
            if self.grid[i] == Tile::Bug {
                rating |= 1 << i;
            }
        }
        rating
    }

    fn tick(&self) -> Self {
        let mut new_grid = Self::new(self.width(), self.height());
        for (x, y) in iproduct!(0..self.width(), 0..self.height()) {
            let mut adjacents = Vec::<Coordinate>::with_capacity(4);
            if x != 0 {
                adjacents.push((x - 1, y));
            }
            if x != self.width() - 1 {
                adjacents.push((x + 1, y));
            }
            if y != 0 {
                adjacents.push((x, y - 1));
            }
            if y != self.height() - 1 {
                adjacents.push((x, y + 1));
            }

            let coord = (x, y);
            let tile = self[coord];
            let adjacent_bug_count = adjacents
                .iter()
                .filter(|coord| self[**coord] == Tile::Bug)
                .count();
            match (tile, adjacent_bug_count) {
                (Tile::Bug, n) if n != 1 => {
                    new_grid[coord] = Tile::Empty;
                }
                (Tile::Empty, n) if n == 1 || n == 2 => {
                    new_grid[coord] = Tile::Bug;
                }
                _ => {
                    new_grid[coord] = tile;
                }
            }
        }
        new_grid
    }
}

impl Index<Coordinate> for Grid {
    type Output = Tile;

    fn index(&self, index: Coordinate) -> &Self::Output {
        self.grid.index(self.width * index.1 + index.0)
    }
}

impl IndexMut<Coordinate> for Grid {
    fn index_mut(&mut self, index: Coordinate) -> &mut Self::Output {
        self.grid.index_mut(self.width * index.1 + index.0)
    }
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<Tile> = Vec::new();
        let mut width: Option<usize> = None;

        for line in s.lines() {
            if !line.is_ascii() {
                return Err("Non-ASCII line");
            }
            if line.is_empty() {
                return Err("Empty line");
            }
            if width.is_some() && width.unwrap() != line.len() {
                return Err("Non-uniform line length");
            }

            width = Some(line.len());
            let tiles: Result<Vec<Tile>, _> = line.chars().map(|c| c.try_into()).collect();
            grid.extend(tiles?);
        }

        if grid.len() == 0 {
            return Err("No lines");
        }

        Ok(Grid {
            grid,
            width: width.unwrap(),
        })
    }
}

fn find_first_repeating_state(grid: &Grid) -> Grid {
    let mut grid = grid.clone();
    let mut states = HashSet::<Grid>::new();

    while states.insert(grid.clone()) {
        grid = grid.tick();
    }
    grid
}

fn main() {
    let input = get_input(24);
    let grid: Grid = input.parse().unwrap();
    let first_repeating_state = find_first_repeating_state(&grid);
    println!(
        "First repeating state biodiversity rating: {}",
        first_repeating_state.biodiversity_rating()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick() {
        let input = "....#\n\
                           #..#.\n\
                           #..##\n\
                           ..#..\n\
                           #....";
        let expected_str = "#..#.\n\
                                  ####.\n\
                                  ###.#\n\
                                  ##.##\n\
                                  .##..";

        let grid: Grid = input.parse().unwrap();
        let grid_tick = grid.tick();
        let grid_expected: Grid = expected_str.parse().unwrap();
        assert_eq!(grid_tick, grid_expected);
    }

    #[test]
    fn test_biodiversity_rating() {
        let input = ".....\n\
                           .....\n\
                           .....\n\
                           #....\n\
                           .#...";
        let grid: Grid = input.parse().unwrap();
        assert_eq!(grid.biodiversity_rating(), 2129920);
    }
}
