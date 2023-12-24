use std::collections::{BinaryHeap, HashSet};

pub fn solve(input: &str) {
    let grid: Grid = input.into();

    let mut queue = BinaryHeap::new();

    queue.push(Pos::new(0, 0, 0, None, 0));

    let mut visited = HashSet::new();

    let mut final_cost = u32::MAX;

    let f = 3;
    let t = 10;

    while let Some(pos) = queue.pop() {
        if pos.x == grid.width - 1 && pos.y == grid.height - 1 && pos.direction_steps > f {
            if pos.cost < final_cost {
                final_cost = pos.cost;
                break;
            }
        }

        if let Some(dir) = pos.last_direction {
            match dir {
                Direction::Up => {
                    if pos.direction_steps > f {
                        try_enqueue(pos.step(Direction::Left), &grid, &mut queue, &mut visited);
                        try_enqueue(pos.step(Direction::Right), &grid, &mut queue, &mut visited);
                    }
                    if pos.direction_steps < t {
                        try_enqueue(pos.step(Direction::Up), &grid, &mut queue, &mut visited);
                    }
                }
                Direction::Down => {
                    if pos.direction_steps > f {
                        try_enqueue(pos.step(Direction::Left), &grid, &mut queue, &mut visited);
                        try_enqueue(pos.step(Direction::Right), &grid, &mut queue, &mut visited);
                    }
                    if pos.direction_steps < t {
                        try_enqueue(pos.step(Direction::Down), &grid, &mut queue, &mut visited);
                    }
                }
                Direction::Left => {
                    if pos.direction_steps > f {
                        try_enqueue(pos.step(Direction::Up), &grid, &mut queue, &mut visited);
                        try_enqueue(pos.step(Direction::Down), &grid, &mut queue, &mut visited);
                    }
                    if pos.direction_steps < t {
                        try_enqueue(pos.step(Direction::Left), &grid, &mut queue, &mut visited);
                    }
                }
                Direction::Right => {
                    if pos.direction_steps > f {
                        try_enqueue(pos.step(Direction::Up), &grid, &mut queue, &mut visited);
                        try_enqueue(pos.step(Direction::Down), &grid, &mut queue, &mut visited);
                    }
                    if pos.direction_steps < t {
                        try_enqueue(pos.step(Direction::Right), &grid, &mut queue, &mut visited);
                    }
                }
            }
        } else {
            try_enqueue(pos.step(Direction::Left), &grid, &mut queue, &mut visited);
            try_enqueue(pos.step(Direction::Right), &grid, &mut queue, &mut visited);
            try_enqueue(pos.step(Direction::Up), &grid, &mut queue, &mut visited);
            try_enqueue(pos.step(Direction::Down), &grid, &mut queue, &mut visited);
        }
    }

    println!("{final_cost}");
}

fn try_enqueue(
    pos: Option<Pos>,
    grid: &Grid,
    queue: &mut BinaryHeap<Pos>,
    visited: &mut HashSet<(usize, usize, Direction, u8)>,
) {
    if let Some(mut pos) = pos {
        if pos.x >= grid.width || pos.y >= grid.height {
            return;
        }

        if visited.contains(&(
            pos.x,
            pos.y,
            pos.last_direction.unwrap(),
            pos.direction_steps,
        )) {
            return;
        }

        let add = grid.heat_loss[pos.x + pos.y * grid.width];
        pos.cost += add;

        visited.insert((
            pos.x,
            pos.y,
            pos.last_direction.unwrap(),
            pos.direction_steps,
        ));
        queue.push(pos);
    }
}

#[derive(Eq, PartialEq, Clone, Hash)]
struct Pos {
    cost: u32,
    x: usize,
    y: usize,
    last_direction: Option<Direction>,
    direction_steps: u8,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let c1 = other.cost + 242 - self.x as u32 - self.y as u32;
        let c2 = self.cost + 242 - self.x as u32 - self.y as u32;

        c1.cmp(&c2)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Pos {
    fn new(
        cost: u32,
        x: usize,
        y: usize,
        last_direction: Option<Direction>,
        direction_steps: u8,
    ) -> Self {
        Self {
            cost,
            x,
            y,
            last_direction,
            direction_steps,
        }
    }

    fn step(&self, direction: Direction) -> Option<Self> {
        let mut c = self.clone();

        match direction {
            Direction::Up => {
                if c.y == 0 {
                    return None;
                }

                c.y -= 1
            }
            Direction::Down => c.y += 1,
            Direction::Left => {
                if c.x == 0 {
                    return None;
                }

                c.x -= 1
            }
            Direction::Right => c.x += 1,
        }

        if let Some(ld) = c.last_direction {
            if ld == direction {
                c.direction_steps += 1;
            } else {
                c.last_direction = Some(direction);
                c.direction_steps = 1;
            }
        } else {
            c.last_direction = Some(direction);
            c.direction_steps = 1;
        }

        Some(c)
    }
}

struct Grid {
    heat_loss: Vec<u32>,
    width: usize,
    height: usize,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let width = value.lines().next().unwrap().len();
        let height = value.lines().count();

        let mut heat_loss = Vec::new();

        for line in value.lines() {
            for char in line.chars() {
                heat_loss.push(char.to_digit(10).unwrap());
            }
        }

        Self {
            heat_loss,
            width,
            height,
        }
    }
}
