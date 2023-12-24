use core::panic;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve(input: &str) {
    let map: Map = input.into();

    let mut heads = Vec::new();   

    let mut nodes = HashMap::new();

    nodes.insert((1, 0), Node { outgoing: Vec::new() });
    heads.push(((1, 0), (1, 0), Tile::Down));
    
    while let Some((head, my_node_pos, dir)) = heads.pop() {
        let mut next = match dir {
            Tile::Up => (head.0, head.1 - 1),
            Tile::Down => (head.0, head.1 + 1),
            Tile::Left => (head.0 - 1, head.1),
            Tile::Right => (head.0 + 1, head.1),
            _ => panic!(),
        };

        let mut last = head;

        let mut next_tile = map.at(next.0, next.1);
        let mut steps = 1;
        loop {
            let next_last = next;

            //check all dirs
            let mut d = (0, 0);
            if let Some(q) = check(&map, (next.0 + 1, next.1), last) {
                next_tile = q.0;
                next = q.1;
                d = (1, 0);
            } else if let Some(q) = check(&map, (next.0 - 1, next.1), last) {
                next_tile = q.0;
                next = q.1;
                d = (-1, 0);
            } else if let Some(q) = check(&map, (next.0, next.1 + 1), last) {
                next_tile = q.0;
                next = q.1;
                d = (0, 1);
            } else if let Some(q) = check(&map, (next.0, next.1 - 1), last) {
                next_tile = q.0;
                next = q.1;
                d = (0, -1);
            }

            last = next_last;

            steps += 1;

            if next.0 == map.width - 2 && next.1 == map.height - 1 {
                nodes.insert(next, Node { outgoing: Vec::new() });
                let my_node = nodes.get_mut(&my_node_pos).unwrap();
                my_node.outgoing.push((steps, next));
                break;
            }

            if next_tile == Tile::Open {
                continue;
            }

            // node end
            let node_pos = ((next.0 as i32 + d.0) as usize, (next.1 as i32 + d.1) as usize);
            if !nodes.contains_key(&node_pos) {
                nodes.insert(node_pos, Node { outgoing: Vec::new() });
                // other outgoing heads
                if map.at(node_pos.0 + 1, node_pos.1) == Tile::Right {
                    heads.push(((node_pos.0 + 1, node_pos.1), node_pos, Tile::Right));
                }
                if map.at(node_pos.0 - 1, node_pos.1) == Tile::Left {
                    heads.push(((node_pos.0 - 1, node_pos.1), node_pos, Tile::Left));
                }
                if map.at(node_pos.0, node_pos.1 + 1) == Tile::Down {
                    heads.push(((node_pos.0, node_pos.1 + 1), node_pos, Tile::Down));
                }
                if map.at(node_pos.0, node_pos.1 - 1) == Tile::Up {
                    heads.push(((node_pos.0, node_pos.1 - 1), node_pos, Tile::Up));
                }
            }

            let my_node = nodes.get_mut(&my_node_pos).unwrap();
            my_node.outgoing.push((steps + 2, (node_pos.0, node_pos.1)));
            break;
        }
    }

    let worst = walk_worst(&nodes, (1, 0));

    add_back_refs(&mut nodes);

    let h = HashSet::new();
    let worst_with_back = walk_worst_with_back_refs(&nodes, (1, 0), &h, (map.width - 2, map.height - 1));

    println!("{worst}");
    println!("{worst_with_back}");
}

fn walk_worst(nodes: &HashMap<(usize, usize), Node>, from: (usize, usize)) -> u32 {
    let node = nodes.get(&from).unwrap();
    
    let mut max = 0;
    for (l, o) in &node.outgoing {
        max = max.max(l + walk_worst(nodes, *o));
    }

    max
}

fn walk_worst_with_back_refs(nodes: &HashMap<(usize, usize), Node>, from: (usize, usize), visited: &HashSet<(usize, usize)>, end: (usize, usize)) -> u32 {
    let node = nodes.get(&from).unwrap();

    if from == end {
        return 0;
    }

    let mut visited = visited.clone();
    visited.insert(from);

    let mut max = 0;
    for (l, o) in &node.outgoing {
        if visited.contains(o) {
            continue;
        }

        max = max.max(l + walk_worst_with_back_refs(nodes, *o, &visited, end));
    }
    
    max
}

fn add_back_refs(nodes: &mut HashMap<(usize, usize), Node>) {
    let keys = nodes.keys().map(|x| *x).collect_vec();

    for key in keys {
        let copy = nodes.get(&key).unwrap().outgoing.clone();

        for (s, c) in copy {
            nodes.get_mut(&c).unwrap().outgoing.push((s, key));
        }
    }
}

fn check(map: &Map, pos: (usize, usize), last: (usize, usize)) -> Option<(Tile, (usize, usize))> {
    if pos == last {
        return None;
    }

    let t = map.at(pos.0, pos.1);
    if t != Tile::Closed {
        Some((t, pos))
    } else {
        None
    }
}

#[derive(Debug)]
struct Node {
    outgoing: Vec<(u32, (usize, usize))>
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    fn at(&self, x: usize, y: usize) -> Tile {
        self.tiles[x + y * self.width]
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len();
        let height = value.lines().count();

        let mut tiles = Vec::new();

        for line in value.lines() {
            for char in line.chars() {
                tiles.push(char.into());
            }
        }

        Self {
            tiles,
            width,
            height
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Open,
    Closed,
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Open,
            '#' => Tile::Closed,
            '^' => Tile::Up,
            'v' => Tile::Down,
            '<' => Tile::Left,
            '>' => Tile::Right,
            _ => panic!()
        }
    }
}