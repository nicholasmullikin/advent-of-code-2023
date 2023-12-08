use std::cmp::Ordering;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct LineStats {
    bid_amount: u32,
    hand: String,
}


fn filter_uniq(hand: String) -> Vec<char> {
    hand.chars()
    // .filter(|c| *c != 'J')
        .collect::<HashSet<char>>()
        .into_iter()
        .collect()
}
fn get_best_hand(hand: String) -> &'static str {
    let best_hands =  ["high", "pair", "2", "3", "full", "4", "5"];
    let pos_chars =['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '1'];


    let mut best_hand = get_hand(hand.clone());
    // let mut temp_hand = hand.chars().collect::<Vec<char>>().clone();

    let mut combinations: Vec<Vec<char>> = vec![hand.chars().collect::<Vec<char>>()];
    if hand == "JJJJJ" {
        return "5"
    }

    for y in 0..5 {
        // println!("{}", y);
        for x_1 in pos_chars {
            // println!("{}", x_1);
            // println!("{:?}", combinations);
            let initial_len = combinations.len();
            for i in 0..initial_len {
                let mut temp_hand_1 = combinations[i].clone();
                if temp_hand_1[y] == 'J' {
                    temp_hand_1[y]=x_1;
                } else {
                    continue
                }
                // println!("{:?}", temp_hand_1);

                combinations.push(temp_hand_1)
            }
        }
    }

    for combination in combinations {
        let t: String = combination.clone().into_iter().collect();
        let current_hand = get_hand(t);
        // println!("{:?} ----", combination);
        if best_hands.iter().position(|&r| r == current_hand).unwrap() > best_hands.iter().position(|&r| r == best_hand).unwrap()  {
            best_hand = current_hand;

        }
    }

    //     for x_2 in pos_chars{
    //         let mut temp_hand_2: Vec<char> = temp_hand_1.clone();

    //         if temp_hand_2[1] == 'J' {
    //             temp_hand_2[1]=x_2;
    //         }
    
    //         for x_3 in pos_chars{
    //             let mut temp_hand_3: Vec<char> = temp_hand_2.clone();

    //             if temp_hand_3[2] == 'J' {
    //                 temp_hand_3[2]=x_3;
    //             }
        
    //             for x_4 in pos_chars{
    //                 let mut temp_hand_4: Vec<char> = temp_hand_3.clone();

    //                 if temp_hand_4[3] == 'J' {
    //                     temp_hand_4[3]=x_4;
    //                 }
            
    //                 for x_5 in pos_chars{
    //                     let mut temp_hand_5: Vec<char> = temp_hand_4.clone();

    //                     if temp_hand_5[4] == 'J' {
    //                         temp_hand_5[4]=x_5;
    //                     }
    //                     let t: String = temp_hand_5.clone().into_iter().collect();
    //                     let current_hand = get_hand(t);
    //                     println!("{:?} ----", temp_hand_5);
    //                     if best_hands.iter().position(|&r| r == current_hand).unwrap() > best_hands.iter().position(|&r| r == best_hand).unwrap()  {
    //                         best_hand = current_hand;

    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    return best_hand;
}


fn get_hand(hand: String) -> &'static str {
    if filter_uniq(hand.clone()).len() == 1{
        return "5"
    } else if filter_uniq(hand.clone()).len() == 2 {
        let mut max_unique = 0;
        for x in filter_uniq(hand.clone()) {
            let pairs = hand.chars().filter(|&c| c == x).count();
            if pairs > max_unique {
                max_unique = pairs;
            }
        }
        if max_unique == 4 {
            return "4"
        } else {
            return "full"
        }
    } else if filter_uniq(hand.clone()).len() == 5 {
        return "high"
    } else if filter_uniq(hand.clone()).len() == 4 {
        return "pair"
    } else if filter_uniq(hand.clone()).len() == 3 {
        let mut max_unique = 0;
        for x in filter_uniq(hand.clone()) {
            let pairs = hand.chars().filter(|&c| c == x).count();
            if pairs > max_unique {
                max_unique = pairs;
            }
        }
        if max_unique == 3 {
            return "3";
        } else {
            return "2";
        }
    }
    return "";
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
    
       
        println!("{}, {}", a.hand, b.hand);
        let a_hand = get_best_hand(a.hand.clone());
        let b_hand = get_best_hand(b.hand.clone());
        // println!("{}, {}", a_hand, b_hand);

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
                for c in vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '1',  'J']{
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