pub fn solve(input: &str) {
    let mut sum = 0;

    let mut sum_power = 0;

    for (i, line) in input.lines().enumerate() {
        let (_, rest) = line.split_once(": ").unwrap();

        let mut failed = false;
        let mut req_red = 0;
        let mut req_blue = 0;
        let mut req_green = 0;

        for col in rest.split([';', ',']) {
            let (num, col) = col.trim().split_once(" ").unwrap();
            let num = num.parse::<i32>().unwrap();

            failed |= match col {
                "blue" => {
                    req_blue = req_blue.max(num);
                    num > 14
                }
                "green" => {
                    req_green = req_green.max(num);
                    num > 13
                }
                "red" => {
                    req_red = req_red.max(num);
                    num > 12
                }
                _ => panic!(),
            };
        }

        if !failed {
            sum += i + 1;
        }

        sum_power += req_red * req_green * req_blue;
    }

    println!("{sum}");
    println!("{sum_power}");
}
