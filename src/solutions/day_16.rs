pub fn solve(input: &str) {
    let mut map: Map = input.into();

    simulate(
        State {
            dir: Direction::Right,
            pos: (0, 0),
        },
        &mut map,
    );

    let sum_1 = sum(&map);

    map.reset_energy();

    let mut sum_2 = 0;

    let width = map.width;
    let height = map.height;
    let iter = (0..width)
        .map(|x| State {
            dir: Direction::Down,
            pos: (x as i16, 0),
        })
        .chain((0..width).map(|x| State {
            dir: Direction::Up,
            pos: (x as i16, height as i16 - 1),
        }))
        .chain((0..height).map(|y| State {
            dir: Direction::Right,
            pos: (0, y as i16),
        }))
        .chain((0..height).map(|y| State {
            dir: Direction::Left,
            pos: (width as i16 - 1, y as i16),
        }));

    for s in iter {
        simulate(s, &mut map);
        let sum = sum(&map);
        sum_2 = sum_2.max(sum);
        map.reset_energy();
    }

    println!("{sum_1}");
    println!("{sum_2}");
}

fn simulate(state: State, map: &mut Map) {
    let mut states = Vec::new();
    states.push(state);

    while let Some(s) = states.pop() {
        let next = s.next(&map);
        match next {
            NextState::Zero => continue,
            NextState::One(s) => states.push(s),
            NextState::Two((s1, s2)) => {
                states.push(s1);
                states.push(s2);
            }
        }

        map.energize(s.pos, s.dir);
    }
}

fn sum(map: &Map) -> u32 {
    let mut sum = 0;
    for e in &map.energized {
        if *e != 0 {
            sum += 1;
        }
    }

    sum
}

#[derive(Clone, Copy)]
enum Tile {
    Space,
    DiagBack,
    DiagForward,
    HorizontalSpitter,
    VerticalSplitter,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Space,
            '\\' => Self::DiagBack,
            '/' => Self::DiagForward,
            '-' => Self::HorizontalSpitter,
            '|' => Self::VerticalSplitter,
            _ => panic!(),
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Space => '.',
            Tile::DiagBack => '\\',
            Tile::DiagForward => '/',
            Tile::HorizontalSpitter => '-',
            Tile::VerticalSplitter => '|',
        }
    }
}

struct Map {
    map: Vec<Tile>,
    energized: Vec<u8>,
    width: usize,
    height: usize,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut map = Vec::with_capacity(value.len());

        for line in value.lines() {
            for char in line.chars() {
                map.push(char.into());
            }
        }

        let width = value.lines().next().unwrap().len();
        let height = value.lines().count();

        Self {
            map,
            energized: vec![0; width * height],
            width,
            height,
        }
    }
}

impl Map {
    fn get(&self, (x, y): (i16, i16)) -> Option<Tile> {
        if x < 0 || y < 0 {
            return None;
        }

        let x = x as usize;
        let y = y as usize;
        if x >= self.width {
            None
        } else if y >= self.height {
            None
        } else {
            Some(self.map[x + y * self.width])
        }
    }

    fn energize(&mut self, (x, y): (i16, i16), dir: Direction) {
        if x < 0 || y < 0 {
            return;
        }

        let x = x as usize;
        let y = y as usize;
        let i = x + y * self.width;

        self.energized[i] = self.energized[i] | 1 << dir as u8;
    }

    fn reset_energy(&mut self) {
        self.energized.fill(0);
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self, (x, y): (i16, i16)) -> (i16, i16) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}

#[derive(Clone)]
struct State {
    dir: Direction,
    pos: (i16, i16),
}

enum NextState {
    Zero,
    One(State),
    Two((State, State)),
}

impl State {
    fn next(&self, map: &Map) -> NextState {
        let (x, y) = self.pos;
        if x < 0 || y < 0 || x >= map.width as i16 || y >= map.height as i16 {
            return NextState::Zero;
        }

        let x = x as usize;
        let y = y as usize;
        if map.energized[x + y * map.width] & (1 << self.dir as u8) != 0 {
            return NextState::Zero;
        }

        let at = map.get(self.pos);
        match at {
            Some(tile) => match tile {
                Tile::Space => NextState::One(State {
                    dir: self.dir,
                    pos: self.dir.next(self.pos),
                }),
                Tile::DiagBack => {
                    let next_dir = match self.dir {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };

                    NextState::One(Self {
                        dir: next_dir,
                        pos: next_dir.next(self.pos),
                    })
                }
                Tile::DiagForward => {
                    let next_dir = match self.dir {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };

                    NextState::One(Self {
                        dir: next_dir,
                        pos: next_dir.next(self.pos),
                    })
                }
                Tile::HorizontalSpitter => match self.dir {
                    Direction::Right | Direction::Left => NextState::One(Self {
                        dir: self.dir,
                        pos: self.dir.next(self.pos),
                    }),
                    Direction::Down | Direction::Up => NextState::Two((
                        Self {
                            dir: Direction::Right,
                            pos: Direction::Right.next(self.pos),
                        },
                        Self {
                            dir: Direction::Left,
                            pos: Direction::Left.next(self.pos),
                        },
                    )),
                },
                Tile::VerticalSplitter => match self.dir {
                    Direction::Up | Direction::Down => NextState::One(Self {
                        dir: self.dir,
                        pos: self.dir.next(self.pos),
                    }),
                    Direction::Right | Direction::Left => NextState::Two((
                        Self {
                            dir: Direction::Up,
                            pos: Direction::Up.next(self.pos),
                        },
                        Self {
                            dir: Direction::Down,
                            pos: Direction::Down.next(self.pos),
                        },
                    )),
                },
            },
            None => NextState::Zero,
        }
    }
}
