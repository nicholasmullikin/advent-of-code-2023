use std::io::prelude::*;
use std::fs::File;


fn main() {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let sections = contents.split("\n").collect::<Vec<&str>>();
    println!("{:?}", sections);
    let times: Vec<i64>= sections[0].split_whitespace().collect::<Vec<&str>>()[1..].iter().map(|x| x.parse::<i64>().unwrap()).collect();
    println!("{:?}", times);


    let distances: Vec<i64>= sections[1].split_whitespace().collect::<Vec<&str>>()[1..].iter().map(|x| x.parse::<i64>().unwrap()).collect();
    println!("{:?}", distances);

    let mut sum: i64 = 1;

    for it in times.iter().zip(distances.iter()) {
        let (time, distance) = it;
        // println!("{:?}, {:?}", time, distance);

        // for x in 0..*time {
        //     let current_distance = (time - x) * x;
        //     if current_distance > *distance {
        //         println!("{}", current_distance)
        //     }
        // }
        let lower =  ((*time as f64) - ((*time as f64) * (*time as f64) - 4.0 * (*distance as f64)).sqrt())/ 2.0;
        let higher =  ((*time as f64) + ((*time as f64) * (*time as f64) - 4.0 * (*distance as f64)).sqrt()) / 2.0;
        println!("{:?}, {:?}", lower, higher);
        println!("{:?}, {:?}", lower.floor() as i64, higher.ceil() as i64);
        let way_to_win = higher.ceil() as i64 - lower.floor() as i64;
        println!("{:?}", way_to_win - 1);
        sum *= way_to_win - 1;
    }
    println!("{:?}", sum);

}