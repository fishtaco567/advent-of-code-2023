#![feature(vec_push_within_capacity, int_roundings)]

use chrono::prelude::*;
use std::{env, time::Instant};
pub mod iter_helper;
mod solutions;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: usize = if args.len() > 1 {
        str::parse(&args[1]).expect("Could not parse day")
    } else {
        let this_day = Local::now().day() as usize;
        if 1 <= this_day && this_day <= 25 {
            this_day
        } else {
            panic!("Not a valid day to invoke this on!");
        }
    };

    let test: bool = if args.len() > 2 {
        args[2] == "t"
    } else {
        false
    };

    println!(
        "  Advent of Code running day: {day} with sample mode: {test}\n\
        =========================================================="
    );

    let start_time = Instant::now();
    solutions::run_day(day, test);
    let duration = start_time.elapsed().as_secs_f64();
    println!("This day took {duration} seconds");
}
