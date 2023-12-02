pub fn solve(input: &str) {
    let mut sum_part_1 = 0;
    let mut sum_part_2 = 0;

    for line in input.lines() {
        let mut first_char = None;
        let mut last_char = None;

        // Part 1
        for (i, char) in line.chars().enumerate() {
            if first_char == None && char.is_numeric() {
                first_char = Some((i, char.to_digit(10).unwrap()));
            }

            if char.is_numeric() {
                last_char = Some((i, char.to_digit(10).unwrap()));
            }
        }

        if let (Some((_, first)), Some((_, last))) = (first_char, last_char) {
            let num = first * 10 + last;
            sum_part_1 += num;
        }

        // Part 2
        let mut first_textual = None;
        let mut second_textual = None;

        for i in 0..line.len() {
            for j in (i + 1)..=line.len().min(i + 5) {
                if let Some(n) = str_to_num(&line[i..j]) {
                    if first_textual.is_none() {
                        first_textual = Some((i, n));
                    }

                    second_textual = Some((i, n));
                }
            }
        }

        let first_digit_part_2 = first_textual.map_or(first_char.unwrap().1, |(i, n_t)| {
            first_char.map_or(n_t, |(j, n_c)| if i < j { n_t } else { n_c })
        });

        let second_digit_part_2 = second_textual.map_or(last_char.unwrap().1, |(i, n_t)| {
            last_char.map_or(n_t, |(j, n_c)| if i > j { n_t } else { n_c })
        });

        let num2 = first_digit_part_2 * 10 + second_digit_part_2;
        sum_part_2 += num2;
    }
    println!("Part 1 is {sum_part_1}");
    println!("Part 2 is {sum_part_2}");
}

fn str_to_num(s: &str) -> Option<u32> {
    match s {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}
