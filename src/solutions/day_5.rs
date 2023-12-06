use itertools::Itertools;

use crate::iter_helper::IterHelper;

pub fn solve(input: &str) {
    let mut ranges: Vec<Vec<(i64, i64, i64)>> = Vec::new();

    let mut lines = input.lines();

    let seeds: Vec<i64> = if let Some(seeds) = lines.next() {
        let (_, rest) = seeds.split_once(": ").unwrap();
        rest.split_ascii_whitespace().parse_all().collect()
    } else {
        panic!();
    };

    let mut cur_set = 0;
    ranges.push(Vec::new());

    for line in lines {
        if let Some(c) = line.chars().next() {
            if c.is_numeric() {
                let (dst, src, len) = line.split_ascii_whitespace().parse_all::<i64>().collect_tuple().unwrap();
                
                ranges[cur_set].push((dst, src, len));
            } else if c.is_whitespace() {
            } else {
                cur_set += 1;
                ranges.push(Vec::new());
            }
        }
    }

    let mut lowest_seed = i64::MAX;

    for seed in &seeds {
        let mut cur = *seed;

        for range in &ranges {
            for (dst, src, len) in range {
                if cur >= *src && cur < *src + *len {
                    cur += *dst - *src;
                    break;
                }
            }
        }

        lowest_seed = lowest_seed.min(cur);
    }

    let mut lowest_seed_2 = i64::MAX;

    for (seed_start, seed_len) in seeds.iter().tuples() {
        let sr = *seed_start..(*seed_start + *seed_len);
        let mut cur = Vec::new();
        cur.push(sr);
        let mut next = Vec::new();

        for range in &ranges {
            println!("{:?}", cur);

            for (dst, src, len) in range {
                let mut retained = Vec::new();
                while let Some(r) = cur.pop() {
                    let src_range = *src..(*src + *len);
                    let offset = *dst - *src;

                    if !(src_range.start > r.end || r.start > src_range.end) {
                        if src_range.start < r.start {
                            if src_range.end < r.end {
                                let mapped_range = (r.start + offset)..(src_range.end + offset);
                                next.push(mapped_range);
                                let overlow_range = src_range.end..r.end;
                                retained.push(overlow_range);
                            } else {
                                let mapped_range = (r.start + offset)..(r.end + offset);
                                next.push(mapped_range);
                            }
                        } else {
                            if r.start < src_range.start {
                                let excluded = r.start..src_range.start;
                                retained.push(excluded);
                            }
                            
                            if src_range.end < r.end {
                                let included = (src_range.start + offset)..(src_range.end + offset);
                                let overflow = src_range.end..r.end;
                                next.push(included);
                                retained.push(overflow);
                            } else {
                                let included = (src_range.start + offset)..(r.end + offset);
                                next.push(included);
                            }
                        }
                    } else {
                        retained.push(r);
                    }
                }

                for r in retained.drain(..) {
                    cur.push(r);
                }
            }

            for r in next.drain(..) {
                cur.push(r);
            }
        }

        for r in cur {
            lowest_seed_2 = lowest_seed_2.min(r.start);
        }
    }
    
    let res = lowest_seed;

    println!("Part 1 is {res}");

    println!("Part 2 is {lowest_seed_2}");
}