use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) {
    let mut sum = 0;

    let grid = CharGrid::new(input);

    let lines_len = grid.source.len();

    let mut gears = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let mut state: Option<(usize, Vec<char>)> = None;

        for (ic, char) in line.char_indices() {
            if char.is_numeric() {
                if let Some((_, x)) = &mut state {
                    x.push(char);
                } else {
                    state = Some((ic, vec![char]));
                }
            } else {
                if let Some((j, xx)) = &state {
                    let num = xx.iter().collect::<String>().parse::<i32>().unwrap();

                    let mut near = false;
                    let mut near_star = HashSet::new();
                    for q in *j..(*j + xx.len()) {
                        for x in q.saturating_sub(1)..=(q + 1).min(grid.width - 1) {
                            for y in i.saturating_sub(1)..=(i + 1).min(lines_len - 1) {
                                let c = grid.at(x, y);
                                if !c.is_numeric() && c != '.' {
                                    near |= true;
                                }

                                if c == '*' {
                                    near_star.insert((x, y));
                                }
                            }
                        }
                    }

                    if near {
                        sum += num;
                    }

                    for (x, y) in near_star {
                        gears
                            .entry((x, y))
                            .and_modify(|x: &mut Vec<i32>| x.push(num))
                            .or_insert(vec![num]);
                    }
                }

                state = None;
            }
        }

        if let Some((j, xx)) = &state {
            let num = xx.iter().collect::<String>().parse::<i32>().unwrap();

            let mut near = false;
            let mut near_star = HashSet::new();
            for q in *j..(*j + xx.len()) {
                for x in q.saturating_sub(1)..=(q + 1).min(grid.width - 1) {
                    for y in i.saturating_sub(1)..=(i + 1).min(lines_len - 1) {
                        let c = grid.at(x, y);
                        if !c.is_numeric() && c != '.' {
                            near |= true;
                        }

                        if c == '*' {
                            near_star.insert((x, y));
                        }
                    }
                }
            }

            if near {
                sum += num;
            }

            for (x, y) in near_star {
                gears
                    .entry((x, y))
                    .and_modify(|x: &mut Vec<i32>| x.push(num))
                    .or_insert(vec![num]);
            }
        }
    }

    println!("{sum}");

    let sum_pt2: i64 = gears
        .values()
        .filter(|x| x.len() == 2)
        .map(|x| x[0] as i64 * x[1] as i64)
        .sum();
    println!("{sum_pt2}");
}

struct CharGrid<'a> {
    source: Vec<&'a str>,
    width: usize,
}

impl<'a> CharGrid<'a> {
    pub fn new(source: &'a str) -> Self {
        let lines: Vec<&'a str> = source.lines().collect();
        let wid = lines[0].len();
        Self {
            source: lines,
            width: wid,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> char {
        self.source[y].chars().nth(x).unwrap()
    }
}
