mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_2;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

static INPUT_1: &'static str = include_str!("../input/day1.txt");
static INPUT_1_TEST: &'static str = include_str!("../input/day1test.txt");
static INPUT_2: &'static str = include_str!("../input/day2.txt");
static INPUT_2_TEST: &'static str = include_str!("../input/day2test.txt");
static INPUT_3: &'static str = include_str!("../input/day3.txt");
static INPUT_3_TEST: &'static str = include_str!("../input/day3test.txt");
static INPUT_4: &'static str = include_str!("../input/day4.txt");
static INPUT_4_TEST: &'static str = include_str!("../input/day4test.txt");
static INPUT_5: &'static str = include_str!("../input/day5.txt");
static INPUT_5_TEST: &'static str = include_str!("../input/day5test.txt");
static INPUT_6: &'static str = include_str!("../input/day6.txt");
static INPUT_6_TEST: &'static str = include_str!("../input/day6test.txt");
static INPUT_7: &'static str = include_str!("../input/day7.txt");
static INPUT_7_TEST: &'static str = include_str!("../input/day7test.txt");
static INPUT_8: &'static str = include_str!("../input/day8.txt");
static INPUT_8_TEST: &'static str = include_str!("../input/day8test.txt");
static INPUT_9: &'static str = include_str!("../input/day9.txt");
static INPUT_9_TEST: &'static str = include_str!("../input/day9test.txt");
static INPUT_10: &'static str = include_str!("../input/day10.txt");
static INPUT_10_TEST: &'static str = include_str!("../input/day10test.txt");
static INPUT_11: &'static str = include_str!("../input/day11.txt");
static INPUT_11_TEST: &'static str = include_str!("../input/day11test.txt");
static INPUT_12: &'static str = include_str!("../input/day12.txt");
static INPUT_12_TEST: &'static str = include_str!("../input/day12test.txt");
static INPUT_13: &'static str = include_str!("../input/day13.txt");
static INPUT_13_TEST: &'static str = include_str!("../input/day13test.txt");
static INPUT_14: &'static str = include_str!("../input/day14.txt");
static INPUT_14_TEST: &'static str = include_str!("../input/day14test.txt");
static INPUT_15: &'static str = include_str!("../input/day15.txt");
static INPUT_15_TEST: &'static str = include_str!("../input/day15test.txt");
static INPUT_16: &'static str = include_str!("../input/day16.txt");
static INPUT_16_TEST: &'static str = include_str!("../input/day16test.txt");
static INPUT_17: &'static str = include_str!("../input/day17.txt");
static INPUT_17_TEST: &'static str = include_str!("../input/day17test.txt");
static INPUT_18: &'static str = include_str!("../input/day18.txt");
static INPUT_18_TEST: &'static str = include_str!("../input/day18test.txt");
static INPUT_19: &'static str = include_str!("../input/day19.txt");
static INPUT_19_TEST: &'static str = include_str!("../input/day19test.txt");
static INPUT_20: &'static str = include_str!("../input/day20.txt");
static INPUT_20_TEST: &'static str = include_str!("../input/day20test.txt");
static INPUT_21: &'static str = include_str!("../input/day21.txt");
static INPUT_21_TEST: &'static str = include_str!("../input/day21test.txt");
static INPUT_22: &'static str = include_str!("../input/day22.txt");
static INPUT_22_TEST: &'static str = include_str!("../input/day22test.txt");
static INPUT_23: &'static str = include_str!("../input/day23.txt");
static INPUT_23_TEST: &'static str = include_str!("../input/day23test.txt");
static INPUT_24: &'static str = include_str!("../input/day24.txt");
static INPUT_24_TEST: &'static str = include_str!("../input/day24test.txt");
static INPUT_25: &'static str = include_str!("../input/day25.txt");
static INPUT_25_TEST: &'static str = include_str!("../input/day25test.txt");

pub fn run_day(day: usize, test: bool) {
    match day {
        1 => {
            let input = if test { INPUT_1_TEST } else { INPUT_1 };
            day_1::solve(input);
        }
        2 => {
            let input = if test { INPUT_2_TEST } else { INPUT_2 };
            day_2::solve(input);
        }
        3 => {
            let input = if test { INPUT_3_TEST } else { INPUT_3 };
            day_3::solve(input);
        }
        4 => {
            let input = if test { INPUT_4_TEST } else { INPUT_4 };
            day_4::solve(input);
        }
        5 => {
            let input = if test { INPUT_5_TEST } else { INPUT_5 };
            day_5::solve(input);
        }
        6 => {
            let input = if test { INPUT_6_TEST } else { INPUT_6 };
            day_6::solve(input);
        }
        7 => {
            let input = if test { INPUT_7_TEST } else { INPUT_7 };
            day_7::solve(input);
        }
        8 => {
            let input = if test { INPUT_8_TEST } else { INPUT_8 };
            day_8::solve(input);
        }
        9 => {
            let input = if test { INPUT_9_TEST } else { INPUT_9 };
            day_9::solve(input);
        }
        10 => {
            let input = if test { INPUT_10_TEST } else { INPUT_10 };
            day_10::solve(input);
        }
        11 => {
            let input = if test { INPUT_11_TEST } else { INPUT_11 };
            day_11::solve(input);
        }
        12 => {
            let input = if test { INPUT_12_TEST } else { INPUT_12 };
            day_12::solve(input);
        }
        13 => {
            let input = if test { INPUT_13_TEST } else { INPUT_13 };
            day_13::solve(input);
        }
        14 => {
            let input = if test { INPUT_14_TEST } else { INPUT_14 };
            day_14::solve(input);
        }
        15 => {
            let input = if test { INPUT_15_TEST } else { INPUT_15 };
            day_15::solve(input);
        }
        16 => {
            let input = if test { INPUT_16_TEST } else { INPUT_16 };
            day_16::solve(input);
        }
        17 => {
            let input = if test { INPUT_17_TEST } else { INPUT_17 };
            day_17::solve(input);
        }
        18 => {
            let input = if test { INPUT_18_TEST } else { INPUT_18 };
            day_18::solve(input);
        }
        19 => {
            let input = if test { INPUT_19_TEST } else { INPUT_19 };
            day_19::solve(input);
        }
        20 => {
            let input = if test { INPUT_20_TEST } else { INPUT_20 };
            day_20::solve(input);
        }
        21 => {
            let input = if test { INPUT_21_TEST } else { INPUT_21 };
            day_21::solve(input);
        }
        22 => {
            let input = if test { INPUT_22_TEST } else { INPUT_22 };
            day_22::solve(input);
        }
        23 => {
            let input = if test { INPUT_23_TEST } else { INPUT_23 };
            day_23::solve(input);
        }
        24 => {
            let input = if test { INPUT_24_TEST } else { INPUT_24 };
            day_24::solve(input);
        }
        25 => {
            let input = if test { INPUT_25_TEST } else { INPUT_25 };
            day_25::solve(input);
        }
        _ => panic!("Day outside of 1 - 25"),
    }
}
