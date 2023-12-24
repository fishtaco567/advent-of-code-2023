pub fn solve(input: &str) {
    let mut sum = 0;

    for set in input.split(',') {
        let mut hash = 0;
        for char in set.bytes() {
            if char == b'\n' || char == b'\r' {
                continue;
            }

            hash += char as u32;
            hash *= 17;
            hash %= 256;
        }

        sum += hash;
    }

    println!("{sum}");

    let mut map = Vec::new();
    for _ in 0..256 {
        map.push(Vec::new());
    }

    for set in input.split(',') {
        if set.contains('=') {
            let (label, focal) = set.trim().split_once('=').unwrap();
            let focal: usize = focal.parse().unwrap();
            let hash = hash(label);

            if let Some(i) = map[hash as usize].iter().position(|(l, _)| *l == label) {
                map[hash as usize][i] = (label, focal);
            } else {
                map[hash as usize].push((label, focal));
            }
        } else {
            let (label, _) = set.trim().split_once('-').unwrap();
            let hash = hash(label);

            if let Some(i) = map[hash as usize].iter().position(|(l, _)| *l == label) {
                map[hash as usize].remove(i);
            }
        }
    }

    let mut sum_2 = 0;
    for (ii, b) in map.iter().enumerate() {
        for (i, (_, focal)) in b.iter().enumerate() {
            sum_2 += (ii + 1) * (i + 1) * focal;
        }
    }

    println!("{sum_2}");
}

fn hash(l: &str) -> u32 {
    let mut hash = 0;
    for char in l.bytes() {
        if char == b'\n' || char == b'\r' {
            continue;
        }

        hash += char as u32;
        hash *= 17;
        hash %= 256;
    }
    hash
}
