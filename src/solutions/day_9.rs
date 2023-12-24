use itertools::Itertools;

use crate::iter_helper::IterHelper;

pub fn solve(input: &str) {
    let input_sequences: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.split_ascii_whitespace().parse_all().collect())
        .collect();

    let mut sum_forward = 0;
    let mut sum_backward = 0;

    for seq in input_sequences {
        let mut sub_seqs = Vec::new();
        sub_seqs.push(seq);

        loop {
            let prev_seq = sub_seqs.last().unwrap();

            let mut next_seq = Vec::with_capacity(prev_seq.len() - 1);

            for (prev, next) in prev_seq.iter().tuple_windows() {
                next_seq.push(*next - *prev);
            }

            sub_seqs.push(next_seq);

            if sub_seqs.last().unwrap().iter().all(|x| *x == 0) {
                break;
            }
        }

        let mut new_last = 0;
        for i in (0..(sub_seqs.len() - 1)).rev() {
            let last = *sub_seqs[i].last().unwrap();
            new_last = last + new_last;
        }

        let mut new_first = 0;
        for i in (0..(sub_seqs.len() - 1)).rev() {
            let first = *sub_seqs[i].first().unwrap();
            new_first = first - new_first;
        }

        sum_forward += new_last;
        sum_backward += new_first;
    }

    println!("{sum_forward}");
    println!("{sum_backward}");
}
