use std::collections::HashMap;

use crate::iter_helper::IterHelper;

pub fn solve(input: &str) {
    let lines: Vec<Line> = input.lines().map(|s| Line::new(s, 5)).collect();

    let mut sum = 0;
    let mut decided_positions = Vec::new();

    for line in &lines {
        let mut pre_processed = Vec::new();

        let mut run = 0;
        let mut state = *line.springs.last().unwrap() != Spring::Operational;

        for spring in line.springs.iter().rev() {
            let next = match spring {
                Spring::Operational => false,
                Spring::Damaged | Spring::Unknown => true,
            };

            if state == next {
                run += 1;
                pre_processed.push((next, run));
            } else {
                pre_processed.push((next, 1));
                state = next;
                run = 1;
            }
        }

        pre_processed.reverse();

        let mut memo = HashMap::new();
        let line_sum = num_layouts(line, &mut decided_positions, &pre_processed, &mut memo, 0);
        sum += line_sum;
    }

    println!("{sum}");
}

fn num_layouts(
    line: &Line,
    decided_positions: &mut Vec<u32>,
    pre_processed: &Vec<(bool, u32)>,
    memo: &mut HashMap<(usize, u32), u64>,
    head: u32,
) -> u64 {
    if decided_positions.len() == line.sums.len() {
        return if check_layout(line, decided_positions, true) {
            1
        } else {
            0
        };
    } else {
        let mut sum = 0;
        let next_len = line.sums[decided_positions.len()];
        for i in head..=(line.springs.len() as u32 - next_len) {
            if let Some((true, available)) = pre_processed.get(i as usize) {
                if next_len > *available {
                    continue;
                }
            } else {
                continue;
            }

            let next_head = i + next_len + 1;
            if let Some(count) = memo.get(&(decided_positions.len() + 1, next_head)) {
                if check_layout(line, decided_positions, false) {
                    sum += *count;
                } else {
                    return 0;
                }
            } else {
                decided_positions.push(i);
                let layouts = num_layouts(
                    line,
                    decided_positions,
                    pre_processed,
                    memo,
                    i + next_len + 1,
                );
                if check_layout(line, decided_positions, false) {
                    memo.insert((decided_positions.len(), next_head), layouts);
                } else {
                    decided_positions.pop();
                    if !check_layout(line, decided_positions, false) {
                        return 0;
                    } else {
                        continue;
                    }
                }
                sum += layouts;
                decided_positions.pop();
            }
        }
        return sum;
    }
}

fn check_layout(line: &Line, decided_positions: &Vec<u32>, check_end: bool) -> bool {
    let mut head = 0;
    for (pos, len) in decided_positions.iter().zip(&line.sums) {
        for _ in head..(*pos as usize) {
            if line.springs[head] == Spring::Damaged {
                return false;
            }

            head += 1;
        }

        for _ in 0..*len {
            if line.springs[head] == Spring::Operational {
                return false;
            }
            head += 1;
        }
    }

    if check_end {
        for _ in head..line.springs.len() {
            if line.springs[head] == Spring::Damaged {
                return false;
            }
            head += 1;
        }
    }

    return true;
}

struct Line {
    springs: Vec<Spring>,
    sums: Vec<u32>,
}

impl Line {
    fn new(value: &str, repeats: usize) -> Self {
        let (map, sums) = value.split_once(' ').unwrap();
        let mut map = map.to_owned();
        map.push('?');
        map = map.repeat(repeats);
        map.pop();

        let mut sums = sums.to_owned();
        sums.push(',');
        sums = sums.repeat(repeats);
        sums.pop();
        Self {
            springs: map.chars().map(|x| x.into()).collect(),
            sums: sums.split(',').parse_all().collect(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!(),
        }
    }
}
