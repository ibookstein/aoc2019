use aoc2019::aoc_input::get_input;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

fn parse_line(line: &str) -> (&str, &str) {
    let items: Vec<_> = line.split(')').collect();
    match &items[..] {
        [s1, s2] => (s1, s2),
        _ => panic!("Malformed line"),
    }
}

type Vertex = usize;
type LabelMap = HashMap<String, Vertex>;
type VertexSet = HashSet<Vertex>;
type Trajectory = Vec<Vertex>;
type AdjacencyList = Vec<VertexSet>;

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

    fn get_expect(&self, label: &str) -> Vertex {
        *self.label_map.get(label).expect("Label not found")
    }

    fn get_or_insert(&mut self, label: &str) -> Vertex {
        match self.label_map.get(label) {
            Some(v) => *v,
            None => {
                let new_vertex = self.adj_list.len();
                self.adj_list.push(VertexSet::new());
                self.label_map.insert(label.to_string(), new_vertex);
                new_vertex
            }
        }
    }

    fn add_edge(&mut self, from_label: &str, to_label: &str) {
        let from_vertex = self.get_or_insert(from_label);
        let to_vertex = self.get_or_insert(to_label);
        self.adj_list[from_vertex].insert(to_vertex);
    }

    fn bfs_layers<F>(&self, origin: Vertex, mut func: F)
    where
        F: FnMut(usize, &VertexSet),
    {
        let mut visited = VertexSet::new();
        visited.insert(origin);
        let mut current_layer = VertexSet::new();
        current_layer.insert(origin);
        let mut depth = 0usize;

        while !current_layer.is_empty() {
            func(depth, &current_layer);
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

    fn dfs_recurse<F>(
        &self,
        visited: &mut VertexSet,
        trajectory: &mut Trajectory,
        v: Vertex,
        func: &mut F,
    ) where
        F: FnMut(&Trajectory),
    {
        if visited.contains(&v) {
            return;
        }

        visited.insert(v);
        trajectory.push(v);
        func(&trajectory);
        for w in self.adj_list[v].iter() {
            self.dfs_recurse(visited, trajectory, *w, func);
        }

        trajectory.pop();
    }

    fn dfs_trajectory<F>(&self, origin: Vertex, mut func: F)
    where
        F: FnMut(&Trajectory),
    {
        let mut visited = VertexSet::new();
        let mut trajectory = vec![];
        self.dfs_recurse(&mut visited, &mut trajectory, origin, &mut func);
    }
}

fn main() {
    //let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN\n";
    let input = get_input(6);
    let edges: Vec<_> = input.trim().lines().map(parse_line).collect();

    let mut graph = Graph::new();

    for (from_label, to_label) in &edges {
        graph.add_edge(from_label, to_label);
    }

    let com_vertex = graph.get_expect("COM");
    let you_vertex = graph.get_expect("YOU");
    let mut you_trajectory: Option<Trajectory> = None;
    let santa_vertex = graph.get_expect("SAN");
    let mut santa_trajectory: Option<Trajectory> = None;

    let mut indirect_orbits = 0usize;

    graph.bfs_layers(com_vertex, |depth, vertices| {
        indirect_orbits += depth * vertices.len();
    });

    println!("Indirect orbits: {}", indirect_orbits);

    graph.dfs_trajectory(com_vertex, |trajectory| {
        let current = *trajectory.last().unwrap();
        if current == you_vertex {
            you_trajectory = Some(trajectory.clone())
        } else if current == santa_vertex {
            santa_trajectory = Some(trajectory.clone())
        }
    });

    let you_trajectory = you_trajectory.unwrap();
    let santa_trajectory = santa_trajectory.unwrap();
    let min_len = min(you_trajectory.len(), santa_trajectory.len());
    let mut mismatch: Option<usize> = None;

    for i in 0..min_len {
        if you_trajectory[i] != santa_trajectory[i] {
            mismatch = Some(i);
            break;
        }
    }

    let mismatch = mismatch.unwrap_or(min_len);
    let transfers = (santa_trajectory.len() - 1 - mismatch) + (you_trajectory.len() - 1 - mismatch);
    println!("Orbital transfers required: {}", transfers);
}
