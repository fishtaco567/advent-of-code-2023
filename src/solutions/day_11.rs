use std::collections::HashSet;

use itertools::Itertools;

pub fn solve(input: &str) {
    let mut column_occupied = HashSet::new();
    let mut row_occupied = HashSet::new();

    let mut galaxies = Vec::new();

    for (j, line) in input.lines().enumerate() {
        for (i, char) in line.chars().enumerate() {
            if char == '#' {
                column_occupied.insert(i);
                row_occupied.insert(j);
                galaxies.push((i as i64, j as i64));
            }
        }
    }

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let mut galaxies_pt_2 = galaxies.clone();

    for j in (0..height).rev() {
        if !row_occupied.contains(&j) {
            for galaxy in &mut galaxies {
                if galaxy.1 > j as i64 {
                    galaxy.1 += 1;
                }
            }

            for galaxy in &mut galaxies_pt_2 {
                if galaxy.1 > j as i64 {
                    galaxy.1 += 999999;
                }
            }
        }
    }

    for i in (0..width).rev() {
        if !column_occupied.contains(&i) {
            for galaxy in &mut galaxies {
                if galaxy.0 > i as i64 {
                    galaxy.0 += 1;
                }
            }

            for galaxy in &mut galaxies_pt_2 {
                if galaxy.0 > i as i64 {
                    galaxy.0 += 999999;
                }
            }
        }
    }

    let mut sum_len = 0;
    for (g1, g2) in galaxies.iter().tuple_combinations() {
        let dx = (g2.0 - g1.0).abs();
        let dy = (g2.1 - g1.1).abs();
        let len = dx + dy;
        sum_len += len;
    }

    let mut sum_len_pt_2 = 0;
    for (g1, g2) in galaxies_pt_2.iter().tuple_combinations() {
        let dx = (g2.0 - g1.0).abs();
        let dy = (g2.1 - g1.1).abs();
        let len = dx + dy;
        sum_len_pt_2 += len;
    }

    println!("The distance between all galaxy pairs was estimated to be {sum_len}, but was actually {sum_len_pt_2}");
}
