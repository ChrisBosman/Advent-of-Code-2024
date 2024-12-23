use std::collections::{HashMap, HashSet};

use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 23".bright_green().bold()); 
    let links = parse_input(input);
    // print_connectivity(&links);
    
    let mut triplets: Vec<[String;3]> = Vec::new();
    let mut max_intersect = 0;
    // For every computer, check which other linked computer has a linked computor in common 
    for (pc1, set1) in links.iter(){
        for pc2 in links.get(pc1).unwrap(){
            let set2 = links.get(pc2).unwrap();
            let intersect = set1.intersection(set2);
            // Check the interconnectivity
            let mut pcs = intersect.clone().collect::<Vec<&String>>();
            pcs.push(pc1);
            pcs.push(pc2);
            if pcs.len() > max_intersect && check_is_interconnected(&pcs, &links){
                max_intersect = pcs.len();
                pcs.sort();
                print!("{}",pcs[0]);
                for pc in pcs.iter().skip(1){
                    print!(",{pc}");
                }
                println!("");
            }
            // These are the loops of three
            for pc3 in intersect{
                push_unique(&mut triplets, [pc1.to_string(),pc2.to_string(),pc3.to_string()]);
            }
        }
    }
    println!("max length: {}",max_intersect);
    let triplets = triplets.iter().filter(|t| contains_prefix_t(t)).collect::<Vec<&[String;3]>>();
    let part1 = triplets.len();

    return (part1, 0); 
} 

fn push_unique(vec1: &mut Vec<[String;3]>, mut el: [String;3]){
    // Check if it already exists in the vector
    el.sort();
    let mut success = true;
    for triplet in vec1.iter(){
        // Check if the same, ordering might be off, so it was sorted
        if *triplet == el{
            success = false;
        }
    }
    if success{
        vec1.push(el);
    }
}

fn parse_input(input: String) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.lines(){
        let (lhs,rhs) = line.split_once("-").unwrap();
        match map.get_mut(&lhs.to_string()) {
            Some(vec) => {vec.insert(rhs.to_string());},
            None => {map.insert(lhs.to_string(), HashSet::from([rhs.to_string()]));},
        }
        match map.get_mut(&rhs.to_string()) {
            Some(vec) => {vec.insert(lhs.to_string());},
            None => {map.insert(rhs.to_string(), HashSet::from([lhs.to_string()]));},
        }
    }
    return map;
}

/// Contains an element starting with a t
fn contains_prefix_t(triplet: &[String;3]) -> bool {
    for el in triplet{
        if el.starts_with('t') {
            return true;
        }
    }
    return false;
}

/// Calculate how well each 
fn check_is_interconnected(nodes: &Vec<&String>, links: &HashMap<String, HashSet<String>>) -> bool{
    // Go over every node and see if it connected to every other node
    for i in 0..nodes.len(){
        for j in i+1..nodes.len(){
            // Check if i and j are connected
            if !links.get(nodes[i]).unwrap().contains(nodes[j]) {
                return false;
            }
        }
    }

    return true
}