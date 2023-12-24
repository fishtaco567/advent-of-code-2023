use std::collections::HashMap;

pub fn solve(input: &str) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut map = vec!['.'; width * height];

    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            map[j + i * height] = c; // COLUMN MAJOR
        }
    }

    let start_map = map.clone();
    let mut map_2 = map.clone();

    let mut force = 0;

    for i in 0..width {
        let mut next_open = 0;
        for j in 0..height {
            match map[j + i * height] {
                '.' => {}
                '#' => next_open = j + 1,
                'O' => {
                    map[next_open + i * height] = 'O';
                    if j != next_open {
                        map[j + i * height] = '.';
                    }
                    let from_bottom = height - next_open;
                    force += from_bottom;
                    next_open += 1;
                }
                _ => panic!(),
            }
        }
    }
    let mut hm: HashMap<usize, Vec<i32>> = HashMap::new();

    let mut repeat_start = 0;
    let mut repeat_period = 0;
    'outer: for q in 0..1000000000 {
        let hash = simulate(&mut map_2, width, height);

        if let Some(other) = hm.get_mut(&hash) {
            for k in other.iter() {
                if check(start_map.clone(), width, height, *k, &map_2) {
                    repeat_start = *k;
                    repeat_period = q - *k;
                    break 'outer;
                }
            }
            other.push(q);
        } else {
            hm.insert(hash, vec![q]);
        }
    }

    let total = 1000000000;
    let without_start = total - repeat_start;
    let past_repeat = without_start % repeat_period;

    let mut final_map = start_map.clone();
    for _ in 0..(repeat_start + past_repeat) {
        simulate(&mut final_map, width, height);
    }

    let mut force_2 = 0;
    for i in 0..width {
        for j in 0..height {
            if final_map[j + i * height] == 'O' {
                force_2 += height - j;
            }
        }
    }

    println!("{force}");
    println!("{force_2}");
}

fn check(
    start_map: Vec<char>,
    width: usize,
    height: usize,
    times: i32,
    check_map: &Vec<char>,
) -> bool {
    let mut map = start_map;
    for _ in 0..=times {
        simulate(&mut map, width, height);
    }

    map.iter()
        .zip(check_map.iter())
        .map(|(i, j)| *i == *j)
        .all(|b| b)
}

fn simulate(map: &mut Vec<char>, width: usize, height: usize) -> usize {
    for i in 0..width {
        let mut next_open = 0;
        for j in 0..height {
            match map[j + i * height] {
                '.' => {}
                '#' => next_open = j + 1,
                'O' => {
                    map[next_open + i * height] = 'O';
                    if j != next_open {
                        map[j + i * height] = '.';
                    }
                    next_open += 1;
                }
                _ => panic!(),
            }
        }
    }

    for j in 0..height {
        let mut next_open = 0;
        for i in 0..width {
            match map[j + i * height] {
                '.' => {}
                '#' => next_open = i + 1,
                'O' => {
                    map[next_open * height + j] = 'O';
                    if i != next_open {
                        map[j + i * height] = '.';
                    }
                    next_open += 1;
                }
                _ => panic!(),
            }
        }
    }

    for i in 0..width {
        let mut next_open = height - 1;
        for j in (0..height).rev() {
            match map[j + i * height] {
                '.' => {}
                '#' => next_open = j.saturating_sub(1),
                'O' => {
                    map[next_open + i * height] = 'O';
                    if j != next_open {
                        map[j + i * height] = '.';
                    }
                    next_open = next_open.saturating_sub(1);
                }
                _ => panic!(),
            }
        }
    }

    for j in 0..height {
        let mut next_open = width - 1;
        for i in (0..width).rev() {
            match map[j + i * height] {
                '.' => {}
                '#' => next_open = i.saturating_sub(1),
                'O' => {
                    map[next_open * height + j] = 'O';
                    if i != next_open {
                        map[j + i * height] = '.';
                    }
                    next_open = next_open.saturating_sub(1);
                }
                _ => panic!(),
            }
        }
    }

    map.iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if *c == 'O' {
                Some(fxhash::hash(&i))
            } else {
                None
            }
        })
        .fold(199933, |f, i| f ^ i)
}
