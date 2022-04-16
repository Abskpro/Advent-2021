use std::fs;

//link to question
//https://adventofcode.com/2021/day/1

// struct ICommand{
//     Forward:String,
//     Down:String,
//     Up:String
// }

enum Commands{
    Forward,
    Down,
    Up
}

pub fn day_2() {
    let report = match fs::read_to_string("./datas/dive.txt") {
        Ok(data) => data,
        Err(error) => {
            panic!("something went wrong {}", error);
        },
    };
    println!("{:?}", report);
    // let formatted :Vec<&str> = report.trim().split("\n").collect();
    // for (pos, data) in formatted.iter().enumerate(){
    //     let parsed_string =  data.parse().unwrap();
    //     if pos == 0{
    //         old_value = parsed_string;
    //     }else{
    //         if parsed_string > old_value {
    //             increased += 1;
    //         } 
    //         old_value = parsed_string;
    //     }
    // }
    // println!("Measurement increased by {}", increased)
}
