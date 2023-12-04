use std::fs::File;
use std::io::prelude::*;

fn get_lines(text: &String) -> Vec<Vec<char>> {
    let lines: Vec<String> = text.lines().map(|line: &str| line.into()).collect();
    let mut total: Vec<Vec<char>> = vec![];
    for x in lines {
        let t: Vec<char> = x.chars().collect();
        total.push(t);
    }
    return total;
}

fn is_symbol(char: char) -> bool {
    if char == '*'
        || char == '%'
        || char == '-'
        || char == '#'
        || char == '='
        || char == '@'
        || char == '$'
        || char == '/'
        || char == '+'
        || char == '&'
    {
        return true;
    }
    return false;
}

fn is_number(char: char) -> bool {
    if char == '1'
        || char == '2'
        || char == '3'
        || char == '4'
        || char == '5'
        || char == '6'
        || char == '7'
        || char == '8'
        || char == '9'
        || char == '0'
    {
        return true;
    }
    return false;
}

fn si(val: usize, min: usize, max: usize) -> usize {
    // Safe indexer
    if val > max {
        return max;
    }
    if val < min {
        return min;
    }

    return val;
}

fn get_numbers(rows: &mut [Vec<char>], upward: bool, downward: bool) -> Vec<u32> {
    if rows.len() < 1 {
        return vec![];
    }
    let row_length = rows[0].len();

    let mut numbers: Vec<u32> = vec![];
    for vertical in 0..rows.len() {
        for horizontal in 0..row_length {
            if is_symbol(rows[vertical][horizontal]) {
                println!("found {}", rows[vertical][horizontal]);

                for vertical_search in 0..si(3, 0, rows.len()) {
                    for horizontal_search in
                        si(horizontal - 1, 0, row_length)..si(horizontal + 2, 0, row_length)
                    {
                        // print!("{} ", rows[vertical_search][horizontal_search]);
                        // print!("{} ", is_number(rows[vertical_search][horizontal_search]));
                        if is_number(rows[vertical_search][horizontal_search]) {
                            // take number horizontally
                            let mut number_as_char: Vec<char> =
                                vec![rows[vertical_search][horizontal_search]];
                            let mut break_left_index = horizontal_search - 1;
                            let mut break_right_index = horizontal_search + 1;
                            while horizontal_search - 2 <= break_left_index
                                && break_left_index == si(break_left_index, 0, row_length)
                            {
                                if is_number(rows[vertical_search][break_left_index]) {
                                    number_as_char.insert(0, rows[vertical_search][break_left_index]);
                                    rows[vertical_search][break_left_index] = '.';
                                    // println!("looking at {} {}",  vertical_search, break_left_index);
                                    if break_left_index == 0 {
                                        break
                                    }
                                    break_left_index -= 1;
                                } else {
                                    break;
                                }
                            }

                            while horizontal_search + 2 >= break_right_index
                                && break_right_index == si(break_right_index, 0, row_length)
                            {
                                if is_number(rows[vertical_search][break_right_index]) {
                                    number_as_char.insert(number_as_char.len(), rows[vertical_search][break_right_index]);
                                    rows[vertical_search][break_right_index] = '.';
                                    // println!("looking at {} {}",  vertical_search, break_right_index);
                                    break_right_index += 1;
                                } else {
                                    break;
                                }
                            }
                            rows[vertical_search][horizontal_search] = '.';
                            let s: String = number_as_char.into_iter().collect();
                            println!("{}", s);
                            numbers.push(s.parse::<u32>().unwrap());
                        }
                    }
                    // println!()
                }
                // println!()
            }
        }
    }
    return numbers;
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let mut lines = get_lines(&contents);
    
    let mut all_numbers: Vec<u32> = vec![];
    for x in 1..lines.len() {
        let mut nums: Vec<u32> = get_numbers(&mut lines[x-1..x+1], true, true);
        all_numbers.append(&mut nums);
    }
    let mut sum = 0;
    for number in all_numbers {
        sum += number;
    }
    println!("{}", sum);
    for x in lines{
        for y in x{
            print!("{}", y);
        }
        println!();
    }
}
