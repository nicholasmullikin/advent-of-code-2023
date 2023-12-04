use std::fs::File;
use std::io::prelude::*;



fn main() {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let mut sum: u32 = 0;
    for (_, line) in contents.split("\n").enumerate() {
        let remove_first = line.split(":").collect::<Vec<&str>>();
        // println!("{:?}", remove_first);
        let remainer = remove_first[1];
        let split_parts = remainer.split("|").collect::<Vec<&str>>();

        let mut their_numbers: Vec<u32> = vec![];
        for (_, line_1) in split_parts[0].split_whitespace().enumerate(){
            // println!("{:?}", my_numbers);
            their_numbers.push(line_1.parse::<u32>().unwrap());
        }
        // println!("{:?}", my_numbers);

        let mut my_numbers: Vec<u32> = vec![];
        for (_, line_1) in split_parts[1].split_whitespace().enumerate(){
            // println!("{:?}", their_numbers);
            my_numbers.push(line_1.parse::<u32>().unwrap());

        }
        // println!("{:?}", their_numbers);
        let mut points: u32 = 0;
        let mut points_count: u32 = 0;
        let base:u32= 2;
        for my_num in my_numbers {
            if their_numbers.contains(&my_num) { 
                // println!("{} {:?}", my_num, their_numbers);
                points = base.pow(points_count);
                points_count += 1;
            }
        }
        sum += points;
        // println!("{}", points);
    }
    println!("{}", sum);
}