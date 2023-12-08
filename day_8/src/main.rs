use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use petgraph::graph::{EdgeReference, Edges, Graph};
use petgraph::Direction;

use petgraph::dot::{Config, Dot};
use petgraph::matrix_graph::NodeIndex;
use petgraph::visit::{EdgeRef, IntoNodeIdentifiers, IntoNodeReferences};
use regex::Regex;


fn main() {
    let mut file = File::open("input.txt").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    let sections = contents.split("\n\n").collect::<Vec<&str>>();

    let mut graph: Graph<&str, &str> = Graph::new(); // directed and unlabeled

    let LR: &str = sections[0];

    let nodes_str: Vec<&str> = sections[1].split("\n").collect::<Vec<&str>>();
    let mut node_index_map: HashMap<&str, NodeIndex<u32>> = HashMap::new();
    let re: Regex = Regex::new(r"[A-Z0-9]{3}").unwrap();
    for x in 0..nodes_str.len() {
        let matches: Vec<&str> = re.find_iter(nodes_str[x]).map(|m| m.as_str()).collect();
        println!("{:?}", matches);

        let t: petgraph::prelude::NodeIndex = graph.add_node(matches[0]);
        node_index_map.insert(matches[0], t);
        //println!("{:?}", graph);
        // hands.push(LineStats{bid_amount: data[1].parse::<u32>().unwrap(), hand: data[0].to_string()});
    }
    //  println!("{:?}", node_index_map);

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
    }
    // println!("{:?}", Dot::with_config(&graph, &[]));
    
    let mut nodes = vec![];
    let node_keys = node_index_map.keys();
    for node_key in node_keys {
        if node_key.ends_with("A") {
            println!("{}", node_key);
            nodes.push(*node_index_map.get(node_key).unwrap());
        }
        else if node_key.ends_with("Z") {
            println!("{}", node_key);
        }
    }



    let chars = LR.chars().collect::<Vec<char>>();
    let mut not_done = true;
    let mut i = 0;
    let mut total_steps = 0;
    let node_refs = graph.node_references();
    while not_done {
        let direction = chars[i];
        // println!("{:?}", nodes);

        // println!("{:?}", i);
        for node_index in 0..nodes.len(){
            let edges = graph
                .edges_directed(nodes[node_index], Direction::Outgoing)
                .collect::<Vec<EdgeReference<'_, &str>>>();
            for edge in edges {
                if *edge.weight() == direction.to_string() {
                    nodes[node_index] = edge.target();
                }
            }
        }
        // println!("{:?}", nodes);

        // for node_index in 1..nodes.len(){
        //     let is_final = graph
        //         .node_references()
        //         .filter(|(x, a)| (**a).ends_with("Z") && *x == nodes[nodes.len() - node_index])
        //         .collect::<Vec<_>>();
        //     if is_final.len() > 0 {
        //         let node = is_final[0];
        //         println!("removing {:?},", *node.1);
        //         nodes.remove(nodes.len() - node_index);
        //         if nodes.len() == 0 {
        //             not_done = false;
        //             break;
        //         }
        //     }
        // }
        let is_final = node_refs.clone().filter(|(x, a)| (**a).ends_with("Z") && nodes.contains(x)).collect::<Vec<_>>();
        if is_final.len() == nodes.len() {
            not_done = false;
        }

        if is_final.len() > 1 {
            println!("{:?}", is_final);
        }

        if total_steps % 10000 == 0  {
            println!("{:?}", total_steps);
        }
        i += 1;
        total_steps += 1;
        if i == chars.len() {
            // break;
            i = 0;
        }
    }
    println!("{}", total_steps);
}
