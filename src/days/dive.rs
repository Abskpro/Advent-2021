use std::fs;

//link to question
//https://adventofcode.com/2021/day/2

pub fn day_2() {
    let report = match fs::read_to_string("./datas/dive.txt") {
        Ok(data) => data,
        Err(error) => {
            panic!("something went wrong {}", error);
        },
    };
    let formatted :Vec<&str> = report.trim().split("\n").collect();
    let mut position = 0;
    let mut depth = 0;
    for (pos, data) in formatted.iter().enumerate(){
        let mut iter = data.splitn(2, ' ');
        let command = iter.next().unwrap();
        let value: u32 = iter.next().unwrap().parse().unwrap();
        if command == "forward"{
            position += value;
        }else if command == "down"  {
            depth += value 
        }else{
           depth -= value 
        } 
    }
    println!("Position is {:?} and depth is {:?}", position, depth);
    let exact_positions = position * depth;
    println!("exact postions is {:?}", exact_positions);
}
