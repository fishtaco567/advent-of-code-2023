use std::{cell, collections::HashSet, os::windows};

pub fn solve(input: &str) {
    let map: Map = input.into();

    let n = find_all_in(&map, 64);

    println!("{n}");

    let m = find_all_in_inf(&map, 1000);

    println!("{m}");
}

fn find_all_in(map: &Map, steps: u32) -> usize {
    let mut open = Vec::new();
    let mut next = Vec::new();

    next.push((map.sx as i32, map.sy as i32));

    let mut closed = HashSet::new();

    for _ in 0..steps {
        std::mem::swap(&mut open, &mut next);

        closed.clear();

        for (x, y) in &open {
            try_walk(map, *x + 1, *y, &mut next, &mut closed);
            try_walk(map, *x - 1, *y, &mut next, &mut closed);
            try_walk(map, *x, *y + 1, &mut next, &mut closed);
            try_walk(map, *x, *y - 1, &mut next, &mut closed);
        }

        open.clear();
    }

    next.len()
}

fn find_all_in_inf(map: &Map, steps: u32) -> usize {
    let mut open = Vec::new();
    let mut next = Vec::new();

    next.push((map.sx as i32, map.sy as i32));

    let mut closed_1 = HashSet::new();
    let mut closed_2 = HashSet::new();

    let mut visited_cells = HashSet::new();

    for i in 0..steps {
        std::mem::swap(&mut open, &mut next);
        std::mem::swap(&mut closed_1, &mut closed_2);

        let mut entered_new = false;

        for (x, y) in &open {
            let (cell_x, cell_y) = (x.div_floor(map.width as i32), y.div_floor(map.height as i32));

            if visited_cells.insert((cell_x, cell_y)) {
                //println!("s{}, r{}, {}, a{}, {}, c{}, {}", i, x.rem_euclid(map.width as i32), y.rem_euclid(map.height as i32), x, y, cell_x, cell_y);
                entered_new = true;
            }

            try_walk_inf(map, *x + 1, *y, &mut next, &mut closed_1);
            try_walk_inf(map, *x - 1, *y, &mut next, &mut closed_1);
            try_walk_inf(map, *x, *y + 1, &mut next, &mut closed_1);
            try_walk_inf(map, *x, *y - 1, &mut next, &mut closed_1);
        }

        if entered_new {
            //println!("count: {}", closed_1.len());
        }

        if (i - 64) % 131 == 0 {
            print!("({i}, {}), ", closed_1.len());
        }

        open.clear();
    }

    closed_1.len()
}

fn try_walk(
    map: &Map,
    x: i32,
    y: i32,
    vec: &mut Vec<(i32, i32)>,
    closed: &mut HashSet<(i32, i32)>,
) {
    if closed.contains(&(x, y)) {
        return;
    };

    match map.at(x, y) {
        Some(Tile::Open) => {
            vec.push((x, y));
            closed.insert((x, y));
        }
        _ => {}
    }
}

fn try_walk_inf(
    map: &Map,
    x: i32,
    y: i32,
    vec: &mut Vec<(i32, i32)>,
    closed: &mut HashSet<(i32, i32)>,
) {
    if closed.contains(&(x, y)) {
        return;
    };

    match map.at_i(x, y) {
        Tile::Open => {
            vec.push((x, y));
            closed.insert((x, y));
        }
        _ => {}
    }
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    sx: usize,
    sy: usize,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len();
        let height = value.lines().count();

        let mut tiles = Vec::new();

        let mut sx = 0;
        let mut sy = 0;

        for (y, line) in value.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                tiles.push(char.into());

                if char == 'S' {
                    sx = x;
                    sy = y;
                }
            }
        }

        Self {
            tiles,
            width,
            height,
            sx,
            sy,
        }
    }
}

impl Map {
    fn at(&self, x: i32, y: i32) -> Option<Tile> {
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;

        if x >= self.width || y >= self.height {
            return None;
        }

        return Some(self.tiles[x + y * self.width]);
    }

    fn at_i(&self, x: i32, y: i32) -> Tile {
        let x = x.rem_euclid(self.width as i32);
        let y = y.rem_euclid(self.height as i32);

        let x = x as usize;
        let y = y as usize;

        return self.tiles[x + y * self.width];
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Rock,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Open,
            '#' => Tile::Rock,
            'S' => Tile::Open,
            _ => panic!(),
        }
    }
}
