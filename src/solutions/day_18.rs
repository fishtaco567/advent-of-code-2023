use itertools::Itertools;

pub fn solve(input: &str) {
    let mut dig_instr: Vec<(Dir, i64)> = Vec::new();

    for line in input.lines() {
        let (dir, rest) = line.split_once(' ').unwrap();
        let (len, _) = rest.split_once(' ').unwrap();

        dig_instr.push((dir.into(), len.parse().unwrap()));
    }

    let mut dig_instr_2: Vec<(Dir, i64)> = Vec::new();
    for line in input.lines() {
        let (_, rest) = line.split_once(' ').unwrap();
        let (_, color) = rest.split_once(' ').unwrap();
        let color = &color[2..color.len() - 1];

        let len = i64::from_str_radix(&color[0..5], 16).unwrap();
        let dir = &color[5..];
        let dir = match dir {
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "3" => Dir::Up,
            _ => panic!(),
        };

        dig_instr_2.push((dir, len));
    }

    let area = count(&dig_instr);
    let area_2 = count(&dig_instr_2);

    println!("{area}");
    println!("{area_2}");
}

fn count(dig_instr: &Vec<(Dir, i64)>) -> i64 {
    let mut cur = (0, 0);
    let mut verts = Vec::new();
    verts.push(cur);

    let direction: i32 = dig_instr
        .iter()
        .map(|(dir, _)| dir)
        .tuple_windows()
        .map(|(l, c)| l.turn(c))
        .sum();
    let direction = (direction / 3) == 1;

    for (i, (dir, len)) in dig_instr.iter().enumerate() {
        let (ox, oy) = dir.to_xy();

        cur.0 += ox * len;
        cur.1 += oy * len;

        let next_dir = if i < dig_instr.len() - 1 {
            dig_instr[i + 1].0
        } else {
            dig_instr.first().unwrap().0
        };

        let o = match (dir, next_dir, direction) {
            (Dir::Up, Dir::Left, true) => (0, 1),
            (Dir::Up, Dir::Left, false) => (1, 0),
            (Dir::Up, Dir::Right, true) => (0, 0),
            (Dir::Up, Dir::Right, false) => (1, 1),
            (Dir::Down, Dir::Left, true) => (1, 1),
            (Dir::Down, Dir::Left, false) => (0, 0),
            (Dir::Down, Dir::Right, true) => (1, 0),
            (Dir::Down, Dir::Right, false) => (0, 1),
            (Dir::Left, Dir::Up, true) => (0, 1),
            (Dir::Left, Dir::Up, false) => (1, 0),
            (Dir::Left, Dir::Down, true) => (1, 1),
            (Dir::Left, Dir::Down, false) => (0, 0),
            (Dir::Right, Dir::Up, true) => (0, 0),
            (Dir::Right, Dir::Up, false) => (1, 1),
            (Dir::Right, Dir::Down, true) => (1, 0),
            (Dir::Right, Dir::Down, false) => (0, 1),
            _ => panic!("{:?} {:?} {direction}", dir, next_dir),
        };

        let v = (cur.0 + o.0, cur.1 + o.1);
        verts.push(v);
    }

    let mut area = 0;
    for i in 0..verts.len() - 1 {
        let j = (i + 1) % verts.len();

        area += verts[i].0 * verts[j].1;
        area -= verts[i].1 * verts[j].0;
    }

    area /= 2;

    area
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Dir {
    fn from(value: &str) -> Self {
        match value {
            "R" => Dir::Right,
            "L" => Dir::Left,
            "U" => Dir::Up,
            "D" => Dir::Down,
            _ => panic!(),
        }
    }
}

impl Dir {
    fn to_xy(&self) -> (i64, i64) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }

    fn turn(&self, other: &Dir) -> i32 {
        match (self, other) {
            (Dir::Up, Dir::Left) => -1,
            (Dir::Up, Dir::Right) => 1,
            (Dir::Down, Dir::Left) => 1,
            (Dir::Down, Dir::Right) => -1,
            (Dir::Left, Dir::Up) => 1,
            (Dir::Left, Dir::Down) => -1,
            (Dir::Right, Dir::Up) => -1,
            (Dir::Right, Dir::Down) => 1,
            _ => 0,
        }
    }
}
