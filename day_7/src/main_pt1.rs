use std::cmp::Ordering;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct LineStats {
    bid_amount: u32,
    hand: String,
}

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut hands: Vec<LineStats> = vec![];
    for x in 0..lines.len()-1 {
        let data = lines[x].split(" ").collect::<Vec<&str>>();
        // println!("{:?}", data);

        hands.push(LineStats{bid_amount: data[1].parse::<u32>().unwrap(), hand: data[0].to_string()});
    }


    fn sort(a: &LineStats, b: &LineStats) -> Ordering {
        fn filter_uniq(hand: String) -> Vec<char> {
            hand.chars()
                .collect::<HashSet<char>>()
                .into_iter()
                .collect()
        }


        let mut a_hand = "";
        let mut b_hand = "";
        if filter_uniq(a.hand.clone()).len() == 1{
            a_hand = "5"
        } else if filter_uniq(a.hand.clone()).len() == 2 {
            let mut max_unique = 0;
            for x in filter_uniq(a.hand.clone()) {
                let pairs = a.hand.chars().filter(|&c| c == x).count();
                // println!("{:?} {:?}", a.hand.chars(), x);

                if pairs > max_unique {
                    max_unique = pairs;
                    // println!("num_a_unique {:?}", max_unique)
                }
            }
            if max_unique == 4 {
                a_hand = "4"
            } else {
                a_hand = "full"
            }
        } else if filter_uniq(a.hand.clone()).len() == 5 {
            a_hand = "high"
        } else if filter_uniq(a.hand.clone()).len() == 4 {
            a_hand = "pair"
        } else if filter_uniq(a.hand.clone()).len() == 3 {
            let mut max_unique = 0;
            for x in filter_uniq(a.hand.clone()) {
                let pairs = a.hand.chars().filter(|&c| c == x).count();
                //println!("{:?} {:?}", a.hand.chars(), x);
                if pairs > max_unique {
                    max_unique = pairs;
                    // println!("num_a_unique {:?}", max_unique)
                }
            }
            if max_unique == 3 {
                a_hand = "3"
            } else {
                a_hand = "2"
            }
        }


        if filter_uniq(b.hand.clone()).len() == 1{
            b_hand = "5"
        } else if filter_uniq(b.hand.clone()).len() == 2 {
            let mut max_unique = 0;
            for x in filter_uniq(b.hand.clone()) {
                let pairs = b.hand.chars().filter(|&c| c == x).count();
                // println!("{:?} {:?}", b.hand.chars(), x);

                if pairs > max_unique {
                    max_unique = pairs;
                    // println!("num_b_unique {:?}", max_unique)
                }
            }
            if max_unique == 4 {
                b_hand = "4"
            } else {
                b_hand = "full"
            }
        } else if filter_uniq(b.hand.clone()).len() == 5 {
            b_hand = "high"
        } else if filter_uniq(b.hand.clone()).len() == 4 {
            b_hand = "pair"
        } else if filter_uniq(b.hand.clone()).len() == 3 {
            let mut max_unique = 0;
            for x in filter_uniq(b.hand.clone()) {
                let pairs = b.hand.chars().filter(|&c| c == x).count();
                //println!("{:?} {:?}", b.hand.chars(), x);
                if pairs > max_unique {
                    max_unique = pairs;
                    // println!("num_b_unique {:?}", max_unique)
                }
            }
            if max_unique == 3 {
                b_hand = "3"
            } else {
                b_hand = "2"
            }
        }
        // println!("{:?} {:?}",a.hand, b.hand);
        // println!("a {:?}, b {:?}", a_hand, b_hand);

        if a_hand != b_hand{
            for c in vec!["5", "4", "full", "3", "2", "pair", "high"]{
                if a_hand == c {
                    return Ordering::Greater
                }
                else if b_hand == c {
                    return Ordering::Less
                }
            }
        }

        

        // println!("a {:?}, b {:?}", a_hand, b_hand);
        // println!("{:?}", a_hand);
        let a_chars = a.hand.chars().collect::<Vec<char>>();
        let b_chars = b.hand.chars().collect::<Vec<char>>();
        if a_hand == b_hand {
            for x in 0..5{
                // println!("{:?} {:?}",a_chars[x], b_chars[x]);

                if a_chars[x] == b_chars[x] {
                    continue;
                }
                for c in vec!['A', 'K', 'Q','J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '1' ]{
                    if a_chars[x] == c {
                        return Ordering::Greater
                    }
                    else if b_chars[x] == c {
                        return Ordering::Less
                    }
                }
            }

        }
        println!("Probably not here");
        return Ordering::Greater

    }
    hands.sort_by(sort);

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        //  println!("{:?} {}", hand.bid_amount, i);

        sum += hand.bid_amount * (i as u32 + 1);
    }
    println!("{:?}", sum)

}