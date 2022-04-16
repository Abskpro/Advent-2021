use std::fs;

//link to question
//https://adventofcode.com/2021/day/1

pub fn day_1() {
    let mut increased = 0;
    let mut old_value = 0;
    let report = match fs::read_to_string("./datas/sonar_sweep.txt") {
        Ok(data) => data,
        Err(error) => {
            panic!("something went wrong {}", error);
        },
    };
    let formatted :Vec<&str> = report.trim().split("\n").collect();
    for (pos, data) in formatted.iter().enumerate(){
        let parsed_string =  data.parse().unwrap();
        if pos == 0{
            old_value = parsed_string;
        }else{
            if parsed_string > old_value {
                increased += 1;
            } 
            old_value = parsed_string;
        }
    }
    println!("Measurement increased by {}", increased)
}
