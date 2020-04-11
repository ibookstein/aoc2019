use aoc2019::aoc_input::get_input;
use itertools::iproduct;
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::ops::Index;
use std::str::FromStr;

type Vertex = usize;
type Label = (usize, usize);
type LabelMap = HashMap<Label, Vertex>;
type VertexSet = HashSet<Vertex>;
type AdjacencyList = Vec<VertexSet>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BfsReply {
    Halt,
    Continue,
}

#[derive(Debug)]
struct Graph {
    label_map: LabelMap,
    adj_list: AdjacencyList,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            label_map: LabelMap::new(),
            adj_list: AdjacencyList::new(),
        }
    }

    fn get_or_insert_vertex(&mut self, label: &Label) -> Vertex {
        match self.label_map.get(label) {
            Some(v) => *v,
            None => {
                let new_vertex = self.adj_list.len();
                self.adj_list.push(VertexSet::new());
                self.label_map.insert(label.to_owned(), new_vertex);
                new_vertex
            }
        }
    }

    fn add_edge_by_labels(&mut self, from_label: &Label, to_label: &Label) {
        let from_vertex = self.get_or_insert_vertex(from_label);
        let to_vertex = self.get_or_insert_vertex(to_label);
        self.adj_list[from_vertex].insert(to_vertex);
    }

    fn add_bidirectional_edge_by_labels(&mut self, label1: &Label, label2: &Label) {
        self.add_edge_by_labels(label1, label2);
        self.add_edge_by_labels(label2, label1);
    }

    fn bfs_layers(&self, origin: Vertex, mut func: impl FnMut(usize, &VertexSet) -> BfsReply) {
        let mut visited = VertexSet::new();
        visited.insert(origin);
        let mut current_layer = VertexSet::new();
        current_layer.insert(origin);
        let mut depth = 0usize;

        while !current_layer.is_empty() {
            match func(depth, &current_layer) {
                BfsReply::Halt => return,
                BfsReply::Continue => (),
            }

            let new_layer: VertexSet = current_layer
                .iter()
                .flat_map(|v| &self.adj_list[*v])
                .cloned()
                .filter(|v| !visited.contains(v))
                .collect();

            visited.extend(&new_layer);
            current_layer = new_layer;
            depth += 1;
        }
    }
}

#[derive(Debug)]
struct AsciiGrid {
    grid: Vec<u8>,
    width: usize,
}

impl AsciiGrid {
    fn height(&self) -> usize {
        self.grid.len() / self.width
    }

    fn width(&self) -> usize {
        self.width
    }
}

impl Index<Label> for AsciiGrid {
    type Output = u8;

    fn index(&self, index: Label) -> &Self::Output {
        self.grid.index(self.width * index.1 + index.0)
    }
}

impl FromStr for AsciiGrid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: Vec<u8> = Vec::new();
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
            grid.extend(line.bytes());
        }

        if grid.len() == 0 {
            return Err("No lines");
        }

        Ok(AsciiGrid {
            grid,
            width: width.unwrap(),
        })
    }
}

#[derive(Debug)]
struct Maze {
    graph: Graph,
    start: Vertex,
    end: Vertex,
}

impl TryFrom<&AsciiGrid> for Maze {
    type Error = &'static str;

    fn try_from(grid: &AsciiGrid) -> Result<Self, Self::Error> {
        let width = grid.width();
        let height = grid.height();

        let mid_x = width / 2;
        let mid_y = height / 2;

        let wall = '#' as u8;
        let empty = '.' as u8;
        let space = ' ' as u8;
        let tiles = [wall, empty];

        let left_width = (2..width - 2)
            .take_while(|x| tiles.contains(&grid[(*x, mid_y)]))
            .count();
        let right_width = (2..width - 2)
            .rev()
            .take_while(|x| tiles.contains(&grid[(*x, mid_y)]))
            .count();
        let top_height = (2..height - 2)
            .take_while(|y| tiles.contains(&grid[(mid_x, *y)]))
            .count();
        let bottom_height = (2..height - 2)
            .rev()
            .take_while(|y| tiles.contains(&grid[(mid_x, *y)]))
            .count();

        let mut portals = HashMap::<[u8; 2], Vec<Label>>::new();

        let outer_ys = 2..height - 2;
        let inner_ys = 4 + top_height..height - bottom_height - 4;
        let outer_xs = 2..width - 2;
        let inner_xs = 4 + left_width..width - right_width - 4;

        let leftbound1 = outer_ys.clone().map(|y| (0, y));
        let leftbound2 = inner_ys.clone().map(|y| (width - right_width - 4, y));
        for (x, y) in leftbound1.chain(leftbound2) {
            let key = [grid[(x, y)], grid[(x + 1, y)]];
            if key[0] != space {
                portals.entry(key).or_default().push((x + 2, y));
            }
        }

        let rightbound1 = inner_ys.map(|y| (2 + left_width, y));
        let rightbound2 = outer_ys.map(|y| (width - 2, y));
        for (x, y) in rightbound1.chain(rightbound2) {
            let key = [grid[(x, y)], grid[(x + 1, y)]];
            if key[0] != space {
                portals.entry(key).or_default().push((x - 1, y));
            }
        }

        let topbound1 = outer_xs.clone().map(|x| (x, 0));
        let topbound2 = inner_xs.clone().map(|x| (x, height - bottom_height - 4));
        for (x, y) in topbound1.chain(topbound2) {
            let key = [grid[(x, y)], grid[(x, y + 1)]];
            if key[0] != space {
                portals.entry(key).or_default().push((x, y + 2));
            }
        }

        let bottombound1 = inner_xs.map(|x| (x, 2 + top_height));
        let bottombound2 = outer_xs.map(|x| (x, height - 2));
        for (x, y) in bottombound1.chain(bottombound2) {
            let key = [grid[(x, y)], grid[(x, y + 1)]];
            if key[0] != space {
                portals.entry(key).or_default().push((x, y - 1));
            }
        }

        let start_portal = ['A' as u8; 2];
        let end_portal = ['Z' as u8; 2];

        let mut graph = Graph::new();
        let mut start: Option<Vertex> = None;
        let mut end: Option<Vertex> = None;

        for (key, value) in portals.iter() {
            match value[..] {
                [point] => {
                    if *key == start_portal {
                        start = Some(graph.get_or_insert_vertex(&point))
                    } else if *key == end_portal {
                        end = Some(graph.get_or_insert_vertex(&point))
                    } else {
                        return Err("Bad portal");
                    }
                }
                [point1, point2] => graph.add_bidirectional_edge_by_labels(&point1, &point2),
                _ => {
                    return Err("Bad portal");
                }
            }
        }

        if start.is_none() {
            return Err("Start portal not found");
        }
        if end.is_none() {
            return Err("End portal not found");
        }

        for coord in iproduct!(2..width - 2, 2..height - 2) {
            if grid[coord] != empty {
                continue;
            }

            let (x, y) = coord;
            let adjacents = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for adj in adjacents.iter().copied() {
                if grid[adj] != empty {
                    continue;
                }
                graph.add_bidirectional_edge_by_labels(&coord, &adj);
            }
        }

        Ok(Maze {
            graph,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

impl Maze {
    fn start_end_distance(&self) -> usize {
        let mut result: Option<usize> = None;
        self.graph.bfs_layers(self.start, |depth, layer| {
            if layer.contains(&self.end) {
                result = Some(depth);
                BfsReply::Halt
            } else {
                BfsReply::Continue
            }
        });
        result.unwrap()
    }
}

fn main() {
    let input = get_input(20);
    let grid: AsciiGrid = input.parse().unwrap();
    let maze: Maze = (&grid).try_into().unwrap();
    println!("Start-end distance: {}", maze.start_end_distance());
}
