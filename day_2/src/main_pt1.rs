use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
fn get_lines(text: &String) -> Vec<String> {
    text
        .lines()
        // You can use `to_string()`, `to_owned()`, or `into()` to convert a
        // string slice into a string. Either works. I find the automatic type
        // inference of `into()` most satisfying, but it's up to preference in
        // this case.
        // The type annotation for `line` is optional.
        .map(|line: &str| line.into())
        // Because the function returns a `Vec`, you can omit the type. I.e.,
        // you don't have to write `collect:<Vec<_>>()`. It's inferred.
        .collect()
}

pub struct GameStats {
    pub index: u32,
    pub total_red: u32,
    pub total_green: u32,
    pub total_blue: u32,
}

fn get_stats(game_line: &String) -> GameStats {
    let split_string:  Vec<String> = game_line.split(": ").map(|line: &str| line.into()).collect();
    
    // Get index
    let index_string = &split_string[0];
    let game_index = &index_string[5..].parse::<u32>().unwrap();


    let stats_string = &split_string[1];
    // println!("{}", stats_string);
    let mut stats = GameStats{index: *game_index, total_red:0, total_green:0, total_blue:0};
    let draws:  Vec<String> = stats_string.split("; ").map(|line: &str| line.into()).collect();

    let re = Regex::new(r"[0-9]{1,}|red|green|blue").unwrap();
    for draw in draws.iter() {
        let matches: Vec<&str> = re.find_iter(draw).map(|m| m.as_str()).collect();
        // # let mut match_iter = matches.iter();
        for x in (0..matches.len()).step_by(2) {
            let number = matches[x].parse::<u32>().unwrap();
            let color = matches[x+1];
            if color == "red" && number > stats.total_red {
                stats.total_red = number;
            }
            if color == "green" && number > stats.total_green  {
                stats.total_green = number;
            }
            if color == "blue" && number > stats.total_blue  {
                stats.total_blue = number;
            }
        }
        println!("{} {} {} {}", stats.index, stats.total_red, stats.total_green, stats.total_blue);

    }
    return stats

}


fn main() {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let lines: Vec<String> = get_lines(&contents);
    let mut total_index_count = 0;
    

    let max_allowed_reds = 12;
    let max_allowed_greens = 13;
    let max_allowed_blues = 14;

    for i in lines.iter() {
        // println!("{}", i);
        let stats = get_stats(i);
        if stats.total_red <= max_allowed_reds && stats.total_green <= max_allowed_greens && stats.total_blue <= max_allowed_blues {
            total_index_count += stats.index
        }
    }
    println!("{}", total_index_count);

}
