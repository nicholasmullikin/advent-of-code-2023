use core::fmt;
use std::fs::File;
use std::io::prelude::*;

// #[derive(Debug)]
struct LineStats {
    number_of_wins: u32,
    original_point_value: u32,
    cards_to_parse: Vec<u32>
}

impl fmt::Debug for LineStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LS [wins: {} OPV: {} Cards:{:?}]\n", self.number_of_wins, self.original_point_value, self.cards_to_parse)
    }
}



// fn search_and_call<T>(obj_list: &mut Vec<T>,
//     mut condition: impl FnMut(&mut T) -> bool,
//     mut func: impl FnMut(&mut T) -> ()) {
//     for x in obj_list {
//         if condition(x) {
//             func(x);
//         }
//     }
// }




fn main() {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let mut sum: u32 = 0;
    let mut games_original: Vec<LineStats> = vec![];

    let split_string = contents.split("\n");
    let split_len = split_string.collect::<Vec<&str>>().len();

    for (_, _) in contents.split("\n").enumerate() {
        games_original.push(LineStats { number_of_wins: 0, original_point_value: 0, cards_to_parse: vec![] });
    }


    for (row, line) in contents.split("\n").enumerate() {
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
            my_numbers.push(line_1.parse::<u32>().unwrap());

        }
        // println!("{:?}", their_numbers);
        let mut points: u32 = 0;
        let mut points_count: u32 = 0;
        let base:u32 = 2;
        for my_num in my_numbers {
            if their_numbers.contains(&my_num) { 
                // points = base.pow(points_count);
                points_count += 1;
            }
        }
        // if points_count > 0 {
        //     line_parsed[row].number_of_wins += 1;
        // }
        // games_original[row].original_point_value = points;


        for row_to_add in row+1..(row+points_count as usize + 1) {
            println!("rows to add {} {}", row, row_to_add);
            if row_to_add >= split_len {
                break
            }
            games_original[row].cards_to_parse.push(row_to_add as u32);
        }
    }
    println!("{:?}", games_original);


    fn check_optional(optional: Option<u32>) -> usize {
        match optional {
            Some(p) => p as usize,
            None => 10000,
        }
    }

    let mut games_copy = vec![games_original];





    // let mut index = 0;
    // while line_parsed.iter().any(|x| x.cards_to_parse.len() > 0){
    //     println!("looking at {:?}/{}", index, split_len);

    //     if line_parsed[index].original_point_value > 0{
    //         line_parsed[index].number_of_wins += 1;
    //     }

    //     for x in 0..line_parsed[index].cards_to_parse.len() {
    //         println!("looking at {} children#{:?}", index,  x);
        
    //         let element_i = check_optional(line_parsed[index].cards_to_parse.pop());

    //         if line_parsed[element_i].original_point_value > 0{
    //             line_parsed[element_i].number_of_wins += 1;
    //         }
    //         println!("{:?}", line_parsed);

    //         println!("element_i {} {:?}", element_i, line_parsed[element_i]);

    //         for row_to_add in element_i+1..(element_i+line_parsed[element_i].original_point_value as usize + 1) {
    //             println!("row_to_add {} wins {}", row_to_add, line_parsed[row_to_add].number_of_wins);

    //             if line_parsed[row_to_add].original_point_value > 0{
    //                 line_parsed[row_to_add].number_of_wins += 1;
    //             }
                
    //             println!("{:?}", line_parsed);


    //             for row_to_add_nums in row_to_add+1..(row_to_add+line_parsed[row_to_add].original_point_value as usize + 1) {
    //                 println!("row_to_add_nums {} {}", row_to_add, row_to_add_nums);
    //                 // if row_to_add_nums >= split_len {
    //                 //     break;
    //                 // }
    //                 line_parsed[row_to_add].cards_to_parse.push(row_to_add_nums as u32);
    //                 println!("{:?}", line_parsed);

    //             }
    //         }
    //         // for row_to_add in index+x+1..(index+x+line_parsed[element_i].original_point_value as usize + 1) {
    //         //     println!("rows to add 1 {} {}", x, row_to_add);
    //         //     if row_to_add >= split_len {
    //         //         break
    //         //     }
    //         //     line_parsed[x].cards_to_parse.push(row_to_add as u32);
    //         // }
    //     }
    //     index += 1;
    //     if index >= split_len {
    //         index = 0;
    //     }
    //     println!("{:?}", line_parsed);
        
    // }

    // for stat in line_parsed {
    //     println!("sum {:?} {:?}", stat.number_of_wins , stat.original_point_value);
    //     sum += stat.number_of_wins * stat.original_point_value;
    // }
    // println!("{}", sum);
}