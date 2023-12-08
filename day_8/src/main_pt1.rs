use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use petgraph::graph::{EdgeReference, Edges, Graph};
use petgraph::Direction;

use petgraph::dot::{Config, Dot};
use petgraph::matrix_graph::NodeIndex;
use petgraph::visit::{EdgeRef, IntoNodeIdentifiers, IntoNodeReferences};
use regex::Regex;

// #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
// struct LineStats {
//     bid_amount: u32,
//     hand: String,
// }

fn main() {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let sections = contents.split("\n\n").collect::<Vec<&str>>();

    let mut graph: Graph<&str, &str> = Graph::new(); // directed and unlabeled

    let LR = sections[0];

    let nodes_str = sections[1].split("\n").collect::<Vec<&str>>();
    let mut node_index_map: HashMap<&str, NodeIndex<u32>> = HashMap::new();
    let re = Regex::new(r"[A-Z]{3}").unwrap();
    for x in 0..nodes_str.len() {
        let matches: Vec<&str> = re.find_iter(nodes_str[x]).map(|m| m.as_str()).collect();
        println!("{:?}", matches);

        let t = graph.add_node(matches[0]);
        node_index_map.insert(matches[0], t);
        //println!("{:?}", graph);
        // hands.push(LineStats{bid_amount: data[1].parse::<u32>().unwrap(), hand: data[0].to_string()});
    }
    // println!("{:?}", graph.node_references());

    for x in 0..nodes_str.len() {
        let matches: Vec<&str> = re.find_iter(nodes_str[x]).map(|m| m.as_str()).collect();
        println!("{:?}", matches);
        graph.add_edge(
            *node_index_map.get(matches[0]).unwrap(),
            *node_index_map.get(matches[1]).unwrap(),
            "L",
        );
        graph.add_edge(
            *node_index_map.get(matches[0]).unwrap(),
            *node_index_map.get(matches[2]).unwrap(),
            "R",
        );

        // println!("{:?}", graph);
        // hands.push(LineStats{bid_amount: data[1].parse::<u32>().unwrap(), hand: data[0].to_string()});
    }

    let start = "AAA";
    let mut node = *node_index_map.get(start).unwrap();
    println!("{:?}", node);

    let chars = LR.chars().collect::<Vec<char>>();
    let mut not_done = true;
    let mut i = 0;
    let mut total_steps = 0;
    while not_done {
        let direction = chars[i];
        // println!("{:?}", i);

        let edges = graph
            .edges_directed(node, Direction::Outgoing)
            .collect::<Vec<EdgeReference<'_, &str>>>();
        for edge in edges {
            if *edge.weight() == direction.to_string() {
                node = edge.target();

                let is_final = graph
                    .node_references()
                    .filter(|(x, a)| **a == "ZZZ" && *x == node)
                    .collect::<Vec<_>>();
                if is_final.len() > 0 {
                    let node = is_final[0];
                    // println!("{:?}, {:?}, {:?}", *node.1, "ZZZ", (*node.1).eq("ZZZ"));
                    if (*node.1).eq("ZZZ") {
                        not_done = false;
                        break;
                    }
                }
            }
        }
        i += 1;
        total_steps += 1;
        if i == chars.len() {
            // break;
            i = 0;
        }
        println!("{}", total_steps);
    }
}
