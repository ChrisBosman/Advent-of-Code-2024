use std::collections::HashMap;

use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 1".bright_green().bold()); 
    // Parse input
    let mut input_arr = parse_input(input);
    
    // Part 1
    input_arr.0.sort_unstable();
    input_arr.1.sort_unstable();
    let mut part1 = 0;
    for (&i, j) in input_arr.0.iter().zip(input_arr.1.iter()) {
        part1 += j.abs_diff(i);
    }

    // Calculate similarity score
    let hist1 = hist(input_arr.0);
    let hist2 = hist(input_arr.1);
    let mut part2 = 0;
    for (k,v) in hist1.iter() {
        part2 += k * hist2.get(k).unwrap_or(&0) * v;
    }
    
    return (part1 as usize, part2 as usize); 
}

/// Count how often every number appears (input must be sorted)
fn hist(input: Vec<u32>) -> HashMap<u32, u32>{
    let mut current_nr = input[0];
    let mut count = 1;
    let mut hist = HashMap::new();
    for i in input.iter().skip(1){
        if *i == current_nr {
            count += 1;
            continue;
        } 
        hist.insert(current_nr, count);
        current_nr = *i;
        count = 1;
    }
    hist.insert(current_nr, count);
    return hist;
}

fn parse_input(input: String) -> (Vec<u32>, Vec<u32>) {
    let input = input.lines().map(|line| line.split("   ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    let mut input_arr: (Vec<u32>, Vec<u32>) = (Vec::new(),Vec::new());
    for line in input {
        input_arr.0.push(line[0]);
        input_arr.1.push(line[1]);
    }
    input_arr
    } 
