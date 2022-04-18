use core::panic;
use std::env;
mod days;


//link to question
//https://adventofcode.com/2021/day/1


fn main() {
    let arg = env::args().nth(1).unwrap();
    match arg.as_str() {
        "day-1" => days::sonar_sweep::day_1(),
        "day-2" => days::dive::day_2(),
        _ => panic!("Invalid arguments")
    }
}
