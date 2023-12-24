pub fn solve(input: &str) {
    let mut sum_1 = 0;
    let mut sum_2 = 0;
    for block in input.split("\r\n\r\n") {
        let chars: Vec<char> = block.lines().map(|s| s.chars()).flatten().collect();
        let width = block.lines().next().unwrap().len();
        let height = block.lines().count();

        'outer: for j in 0..height {
            let mut mistakes = 0;
            for (j1, j2) in (0..j).rev().zip(j..height) {
                for i in 0..width {
                    if chars[i + j1 * width] != chars[i + j2 * width] {
                        mistakes += 1;
                    }

                    if mistakes > 1 {
                        continue 'outer;
                    }
                }
            }

            if mistakes == 0 {
                sum_1 += 100 * j;
            } else if mistakes == 1 {
                sum_2 += 100 * j;
            }
        }

        'outer: for i in 0..width {
            let mut mistakes = 0;
            for (i1, i2) in (0..i).rev().zip(i..width) {
                for j in 0..height {
                    if chars[i1 + j * width] != chars[i2 + j * width] {
                        mistakes += 1;
                    }

                    if mistakes > 1 {
                        continue 'outer;
                    }
                }
            }

            if mistakes == 0 {
                sum_1 += i;
            } else if mistakes == 1 {
                sum_2 += i;
            }
        }
    }
    println!("{sum_1}");
    println!("{sum_2}");
}
