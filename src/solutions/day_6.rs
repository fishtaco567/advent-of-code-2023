use itertools::Itertools;

use crate::iter_helper::IterHelper;

pub fn solve(input: &str) {
    let lines = input.lines().collect_vec();
    let line_1 = lines[0];
    let line_2 = lines[1];

    let times = process(line_1);
    let distances = process(line_2);
    let ways_pt_1: usize = times
        .iter()
        .zip(&distances)
        .map(|(time, distance)| ways(*time, *distance))
        .product();

    let time_pt_2 = process_pt_2(line_1);
    let distance_pt_2 = process_pt_2(line_2);
    let ways_pt_2 = ways(time_pt_2, distance_pt_2);

    println!("{ways_pt_1}");
    println!("{ways_pt_2}");
}

fn ways(time: u64, record: u64) -> usize {
    (0..time).filter(|x| x * (x - time) > record).count()
}

fn process(x: &str) -> Vec<u64> {
    let (_, rest) = x.split_once(':').unwrap();
    rest.split_ascii_whitespace()
        .parse_all::<u64>()
        .collect_vec()
}

fn process_pt_2(x: &str) -> u64 {
    let (_, rest) = x.split_once(':').unwrap();
    rest.split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}
