use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
struct LineStats {
    source: i64,
    dest: i64,
    len: i64
}

struct MyRange {
    current: i64,
    target: i64,
    step: i64,
  }
  
impl Iterator for MyRange {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        if self.current == self.target {
            None
        } else {
            if self.current < self.target {
                self.current += self.step;
            } else {
                self.current -= self.target;
            }
            Some(self.current)
        }
    }
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let sections = contents.split("\n\n").collect::<Vec<&str>>();
    // let sections_len = sections.len();

    let seed_line = sections[0];
    let mut seeds: Vec<i64> = vec![];
    for x in 1..seed_line.split(" ").collect::<Vec<&str>>().len(){
        seeds.push(seed_line.split(" ").collect::<Vec<&str>>()[x].parse::<i64>().unwrap())
    }
    println!("{:?}", seeds);

    let mut seed_to_soil_map: Vec<LineStats> = vec![];
    let mut soil_to_fertilizer_map: Vec<LineStats> = vec![];
    let mut fertilizer_to_water_map: Vec<LineStats> = vec![];
    let mut water_to_light_map: Vec<LineStats> = vec![];
    let mut light_to_temperature_map: Vec<LineStats> = vec![];
    let mut temperature_to_humidity_map: Vec<LineStats> = vec![];
    let mut humidity_to_location_map: Vec<LineStats> = vec![];


    let section_seed_to_soil_map: Vec<&str> = sections[1].split("\n").collect::<Vec<&str>>();
    let section_soil_to_fertilizer_map: Vec<&str> = sections[2].split("\n").collect::<Vec<&str>>();
    let section_fertilizer_to_water_map: Vec<&str> = sections[3].split("\n").collect::<Vec<&str>>();
    let section_water_to_light_map: Vec<&str> = sections[4].split("\n").collect::<Vec<&str>>();
    let section_light_to_temperature_map: Vec<&str> = sections[5].split("\n").collect::<Vec<&str>>();
    let section_temperature_to_humidity_map: Vec<&str> = sections[6].split("\n").collect::<Vec<&str>>();
    let section_humidity_to_location_map: Vec<&str> = sections[7].split("\n").collect::<Vec<&str>>();
    for x in 1..section_seed_to_soil_map.len(){
        let row_contents = section_seed_to_soil_map[x].split(" ").collect::<Vec<&str>>();
        // println!("{:?}", row_contents);
        seed_to_soil_map.push(LineStats{source:row_contents[1].parse::<i64>().unwrap(), dest:row_contents[0].parse::<i64>().unwrap(), len:row_contents[2].parse::<i64>().unwrap()})
    }
    for x in 1..section_soil_to_fertilizer_map.len(){
        let row_contents = section_soil_to_fertilizer_map[x].split(" ").collect::<Vec<&str>>();
        // println!("{:?}", row_contents);
        soil_to_fertilizer_map.push(LineStats{source:row_contents[1].parse::<i64>().unwrap(), dest:row_contents[0].parse::<i64>().unwrap(), len:row_contents[2].parse::<i64>().unwrap()})
    }
    for x in 1..section_fertilizer_to_water_map.len(){
        let row_contents = section_fertilizer_to_water_map[x].split(" ").collect::<Vec<&str>>();
        // println!("{:?}", row_contents);
        fertilizer_to_water_map.push(LineStats{source:row_contents[1].parse::<i64>().unwrap(), dest:row_contents[0].parse::<i64>().unwrap(), len:row_contents[2].parse::<i64>().unwrap()})
    }
    for x in 1..section_water_to_light_map.len(){
        let row_contents = section_water_to_light_map[x].split(" ").collect::<Vec<&str>>();
        // println!("{:?}", row_contents);
        water_to_light_map.push(LineStats{source:row_contents[1].parse::<i64>().unwrap(), dest:row_contents[0].parse::<i64>().unwrap(), len:row_contents[2].parse::<i64>().unwrap()})
    }
    for x in 1..section_light_to_temperature_map.len(){
        let row_contents = section_light_to_temperature_map[x].split(" ").collect::<Vec<&str>>();
        light_to_temperature_map.push(LineStats{source:row_contents[1].parse::<i64>().unwrap(), dest:row_contents[0].parse::<i64>().unwrap(), len:row_contents[2].parse::<i64>().unwrap()})
    }

    for x in 1..section_temperature_to_humidity_map.len(){
        let row_contents = section_temperature_to_humidity_map[x].split(" ").collect::<Vec<&str>>();
        temperature_to_humidity_map.push(LineStats{source:row_contents[1].parse::<i64>().unwrap(), dest:row_contents[0].parse::<i64>().unwrap(), len:row_contents[2].parse::<i64>().unwrap()})
    }
    for x in 1..section_humidity_to_location_map.len(){
        let row_contents = section_humidity_to_location_map[x].split(" ").collect::<Vec<&str>>();
        humidity_to_location_map.push(LineStats{source:row_contents[1].parse::<i64>().unwrap(), dest:row_contents[0].parse::<i64>().unwrap(), len:row_contents[2].parse::<i64>().unwrap()})
    }

    // println!("{:?}", seed_to_soil_map);
    // println!("{:?}", soil_to_fertilizer_map);
    // println!("{:?}", fertilizer_to_water_map);
    // println!("{:?}", water_to_light_map);
    // println!("{:?}", light_to_temperature_map);
    // println!("{:?}", temperature_to_humidity_map);
    // println!("{:?}", humidity_to_location_map);
    // println!("{:?}", seed_to_soil_map);
    let mut lowest = 28580589;
// 0/20 3429320627 235304036
// 1501654215
// 1439936536
// 531587553
//  28580589
// 2/20 1147330745 114559245
// 4/20 1684000747 468955901
// 6/20 677937579 96599505
    let mut x = 6;
    while  x <= seeds.len()-1 {
        println!("{:?}/{} {} {}", x, seeds.len(), seeds[x], seeds[x+1]);
        for seed_num in (MyRange{current:seeds[x], target:seeds[x]+seeds[x+1], step:1}){
            // println!("start {}", seed_num);

            let mut number = seed_num;
            // println!("{:?}", number);
            // println!("{:?}", seed_to_soil_map);

            for y in &seed_to_soil_map {
                if number >= y.source && number < y.source + y.len {
                    number = y.dest - y.source + number;
                    break;
                }
            }
            // println!("{:?}", number);

            for y in &soil_to_fertilizer_map {
                if number >= y.source && number < y.source + y.len {
                    number = y.dest - y.source + number;
                    break;
                }
            }

            // println!("{:?}", number);

            for y in &fertilizer_to_water_map {
                // println!("{:?}", fertilizer_to_water_map);
                if number >= y.source && number < y.source + y.len {
                    number = y.dest - y.source + number;
                    break;
                }
            }

            // println!("{:?}", number);


            for y in &water_to_light_map {
                if number >= y.source && number < y.source + y.len {
                    number = y.dest - y.source + number;
                    break;
                }
            }

        // println!("{:?}", number);


            for y in &light_to_temperature_map {
                if number >= y.source && number < y.source + y.len {
                    number = y.dest - y.source + number;
                    break;
                }
            }


            // println!("{:?}", number);

            for y in &temperature_to_humidity_map {
                if number >= y.source && number < y.source + y.len {
                    number = y.dest - y.source + number;
                    break;
                }
            }

            // println!("{:?}", number);

            for y in &humidity_to_location_map {
                if number >= y.source && number < y.source + y.len {
                    number = y.dest - y.source + number;
                    break;
                }
            }
            // println!("finish {:?} \n", number);

            if number < lowest {
                lowest = number;
                println!("{:?}", lowest);
            }
        }
        x += 2;
    }
    println!("{}", lowest)
}
