use crate::iter_helper::IterHelper;
use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(input: &str) {
    let mut res: i32 = 0;

    let mut scores = Vec::new();

    for line in input.lines() {
        let (_, rest) = line.split_once(": ").unwrap();
        let (winning, mine) = rest.split_once(" | ").unwrap();

        let winning: HashSet<i32> = winning.split_ascii_whitespace().parse_all().collect();
        let mine = mine
            .split_ascii_whitespace()
            .parse_all::<i32>()
            .collect_vec();

        let mut score = None;
        let mut won = 0;

        for num in mine {
            if winning.contains(&num) {
                if let Some(s) = score {
                    score = Some(s * 2);
                } else {
                    score = Some(1);
                }

                won += 1;
            }
        }

        scores.push(won);

        if let Some(score) = score {
            res += score;
        }
    }

    let mut num_cards = vec![1; scores.len()];

    for i in 0..scores.len() {
        let score = scores[i];
        let num_of = num_cards[i];

        for j in 1..=score {
            num_cards[i + j] += num_of;
        }
    }

    let part_2 = num_cards.iter().sum::<i64>();

    println!("Part 1 is: {res}");
    println!("Part 2 is: {part_2}");
}
