use std::collections::HashMap;

pub fn solve(input: &str) {
    let mut iter = input.lines();

    let instructions = iter.next().unwrap();

    let mut map = HashMap::new();

    while let Some(line) = iter.next() {
        if line.is_empty() {
            continue;
        }

        let (origin, dest) = line.split_once(" = ").unwrap();
        let (left, right) = dest[1..dest.len() - 1].split_once(", ").unwrap();

        map.insert(origin, (left, right));
    }

    let all_a: Vec<&str> = map
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| *s)
        .collect();

    let mut count = 0;
    let mut cur = "AAA";

    for instr in std::iter::repeat(instructions.chars()).flatten() {
        let (left, right) = *map.get(cur).unwrap();
        cur = match instr {
            'R' => right,
            'L' => left,
            _ => panic!(),
        };
        count += 1;

        if cur == "ZZZ" {
            break;
        }
    }

    let mut cur = Vec::new();
    let mut all_hits = HashMap::new();
    let mut hit_gaps = HashMap::new();
    for a in &all_a {
        cur.push((*a, *a));
        all_hits.insert(*a, Vec::new());
        hit_gaps.insert(*a, Vec::new());
    }

    let mut loops = Vec::new();

    let mut total_steps = 0u64;
    for (i_c, instr) in std::iter::repeat(instructions.chars().enumerate()).flatten() {
        total_steps += 1;
        let mut to_rem = Vec::new();

        for (i, (start, c)) in cur.iter_mut().enumerate() {
            let (left, right) = *map.get(c).unwrap();
            *c = match instr {
                'R' => right,
                'L' => left,
                _ => panic!(),
            };

            if c.ends_with('Z') {
                let v = all_hits.get_mut(start).unwrap();
                v.push(total_steps);
                let v2 = hit_gaps.get_mut(start).unwrap();
                v2.push((i_c, *c));

                let mut l = None;
                for jmp in 1..=(v2.len() / 2) {
                    let mut failed = false;
                    'outer: for j in (v2.len() - jmp)..v2.len() {
                        if v2[j] != v2[j - jmp] {
                            failed = true;
                            break 'outer;
                        }
                    }
                    if !failed {
                        let js = v2.len() - jmp;
                        let jm = v.last().unwrap() - v[js - jmp];
                        l = Some((js, jm));
                        break;
                    }
                }

                if let Some((loop_start, jmp)) = l {
                    loops.push((*start, loop_start, jmp));
                    to_rem.push(i);
                }
            }
        }

        to_rem.sort();
        to_rem.reverse();

        for i in to_rem {
            cur.remove(i);
        }

        if cur.len() == 0 {
            break;
        }
    }

    let mut seen = HashMap::new();
    for (start, _, _) in &loops {
        for h in all_hits.get(start).unwrap().iter() {
            seen.insert(*h, 1);
        }
    }
    println!("{:?}", loops);
    println!("{:?}", all_hits);

    let mut ii = 1;
    let ccc;
    'outer: loop {
        for (start, s_j, j) in &loops {
            let v = all_hits.get(start).unwrap();
            for i in *s_j..v.len() {
                let q = v[i] + *j * ii;

                if let Some(c) = seen.get_mut(&q) {
                    *c += 1;
                    if *c >= all_a.len() {
                        ccc = q;
                        break 'outer;
                    }
                } else {
                    seen.insert(q, 1);
                }
            }
        }
        ii += 1;
    }

    println!("{count}");
    println!("{ccc}");
}
