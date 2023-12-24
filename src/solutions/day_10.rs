use std::{collections::HashSet, hint::black_box, iter};

use itertools::Itertools;

pub fn solve(input: &str) {
    let grid: Grid = input.into();

    let mut visited = iter::repeat(None)
        .take(grid.height * grid.width)
        .collect_vec();

    let mut cur_x = grid.start_x;
    let mut cur_y = grid.start_y;
    let mut step_x = 0;
    let mut step_y = 0;
    let mut cur_walked = 0;
    let mut chosen = false;

    // Figure out a direction to go
    let mut matched = Vec::with_capacity(2);

    for i in cur_x.wrapping_add_signed(-1)..=grid.width.min(cur_x + 1) {
        for j in cur_y.wrapping_add_signed(-1)..=(grid.height.min(cur_y + 1)) {
            let tile = grid.at(i, j);
            let connectivity = tile.get_connectivity();
            let mut matches = connectivity
                .iter()
                .map(|(x, y)| (i.wrapping_add_signed(*x), j.wrapping_add_signed(*y)))
                .filter(|q| *q == (grid.start_x, grid.start_y));

            if let Some(_) = matches.next() {
                matched.push((
                    i as isize - grid.start_x as isize,
                    j as isize - grid.start_y as isize,
                ));
                if !chosen {
                    chosen = true;
                    step_x = i as isize - cur_x as isize;
                    step_y = j as isize - cur_y as isize;
                    cur_x = i;
                    cur_y = j;
                    cur_walked += 1;
                }
            }
        }
    }

    assert_eq!(matched.len(), 2);
    let start = if matched
        .iter()
        .all(|x| Tile::NS.get_connectivity().contains(x))
    {
        Tile::NS
    } else if matched
        .iter()
        .all(|x| Tile::EW.get_connectivity().contains(x))
    {
        Tile::EW
    } else if matched
        .iter()
        .all(|x| Tile::NE.get_connectivity().contains(x))
    {
        Tile::NE
    } else if matched
        .iter()
        .all(|x| Tile::NW.get_connectivity().contains(x))
    {
        Tile::NW
    } else if matched
        .iter()
        .all(|x| Tile::SW.get_connectivity().contains(x))
    {
        Tile::SW
    } else if matched
        .iter()
        .all(|x| Tile::SE.get_connectivity().contains(x))
    {
        Tile::SE
    } else {
        panic!()
    };

    visited[grid.start_x + grid.start_y * grid.width] = Some(start);

    loop {
        let tile = grid.at(cur_x, cur_y);
        let con = tile.get_connectivity();
        let (dx, dy) = if con[0] == (-step_x, -step_y) {
            con[1]
        } else {
            con[0]
        };

        visited[cur_x + cur_y * grid.width] = Some(tile);
        step_x = dx;
        step_y = dy;
        cur_x = cur_x.wrapping_add_signed(dx);
        cur_y = cur_y.wrapping_add_signed(dy);
        cur_walked += 1;

        if (cur_x, cur_y) == (grid.start_x, grid.start_y) {
            break;
        }
    }

    let mut in_area = if cfg!(debug_assertions) {
        Some(HashSet::new())
    } else {
        None
    };
    let mut area = 0;
    for j in 0..grid.height {
        let mut open_north = false;
        for i in 0..grid.width {
            if let Some(tile) = visited[i + j * grid.width] {
                if matches!(tile, Tile::NS | Tile::NE | Tile::NW) {
                    open_north = !open_north;
                }
            } else if open_north {
                if cfg!(debug_assertions) {
                    write_in_area(&mut in_area, (i, j));
                }
                area += 1;
            }
        }
    }

    if cfg!(debug_assertions) {
        write_visuals(&grid, &visited, &in_area);
    }

    let result = cur_walked / 2;
    // println!("The furthest is {result} away, the area is {area}");
    black_box((result, area));
}

fn write_visuals(
    grid: &Grid,
    visited: &Vec<Option<Tile>>,
    in_area: &Option<HashSet<(usize, usize)>>,
) {
    let in_area = if let Some(a) = in_area {
        a
    } else {
        unreachable!()
    };

    for j in 0..grid.height {
        for i in 0..grid.width {
            if in_area.contains(&(i, j)) {
                print!("■");
            } else if visited[i + j * grid.width].is_some() {
                print!("{}", grid.at(i, j).to_char());
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn write_in_area(in_area: &mut Option<HashSet<(usize, usize)>>, pos: (usize, usize)) {
    let in_area = if let Some(a) = in_area {
        a
    } else {
        unreachable!()
    };

    in_area.insert(pos);
}

struct Grid {
    tiles: Vec<Tile>,
    height: usize,
    width: usize,
    start_x: usize,
    start_y: usize,
}

impl Grid {
    #[inline(always)]
    fn at(&self, x: usize, y: usize) -> Tile {
        return self.tiles[x + y * self.width];
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let mut tiles = Vec::with_capacity(value.len());

        let mut width = None;
        let mut height = 0;
        let mut start_x = 0;
        let mut start_y = 0;

        let mut ai = 0;
        for (i, char) in value.bytes().enumerate() {
            if char == b'\r' {
                if width.is_none() {
                    width = Some(i);
                }
                height += 1;
                continue;
            } else if char == b'\n' {
                continue;
            }

            unsafe {
                let _ = tiles.push_within_capacity(std::mem::transmute(char));
            }

            if char == b'S' {
                if let Some(width) = width {
                    start_x = ai % width;
                    start_y = ai / width;
                } else {
                    let width = value.lines().next().unwrap().len();
                    start_x = ai % width;
                    start_y = ai / width;
                }
            }
            ai += 1;
        }

        let width = if let Some(w) = width { w } else { panic!() };

        Self {
            tiles,
            height,
            width,
            start_x,
            start_y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
enum Tile {
    NS = b'|',
    EW = b'-',
    NE = b'L',
    NW = b'J',
    SW = b'7',
    SE = b'F',
    G = b'.',
    S = b'S',
}

impl Tile {
    #[inline(always)]
    fn get_connectivity(&self) -> &[(isize, isize)] {
        match self {
            Tile::NS => &[(0, -1), (0, 1)],
            Tile::EW => &[(1, 0), (-1, 0)],
            Tile::NE => &[(0, -1), (1, 0)],
            Tile::NW => &[(0, -1), (-1, 0)],
            Tile::SW => &[(0, 1), (-1, 0)],
            Tile::SE => &[(0, 1), (1, 0)],
            Tile::G => &[],
            Tile::S => &[],
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::NS => '│',
            Tile::EW => '─',
            Tile::NE => '└',
            Tile::NW => '┘',
            Tile::SW => '┐',
            Tile::SE => '┌',
            Tile::G => '.',
            Tile::S => 'S',
        }
    }
}
