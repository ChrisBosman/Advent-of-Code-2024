use std::collections::HashMap;
use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 5".bright_green().bold()); 
    let (safety_rules, mut manuals) = parse_input(input);

    //* Part 1
    let mut part1 = 0;
    let mut to_remove: Vec<usize> = Vec::new();
    for (i, manual) in manuals.iter().enumerate() {
        let safety_id = find_page_id(&safety_rules, manual);
        if is_correct(&safety_id) {
            part1 += manual[manual.len()/2];
            to_remove.push(i);
        }
    }

    //* Part 2
    let mut part2 = 0;
    // Remove the correctly sorted manuals
    for i in to_remove.iter().rev() {
        manuals.swap_remove(*i);
    }
    for manual in manuals.iter_mut() {
        let safety_id = find_page_id(&safety_rules, manual);
        fix_manual(&safety_id, manual, &safety_rules);
        part2 += manual[manual.len()/2];
    }

    return (part1 as usize, part2 as usize); 
} 

fn parse_input(input: String) -> (Vec<(u32,u32)>,Vec<Vec<u32>>) {
    let mut safety_rules = Vec::new();
    let mut manuals: Vec<Vec<u32>> = Vec::new();
    let mut is_collecting_manuals = false;

    for line in input.lines() {
        if line.is_empty() {
            is_collecting_manuals = true;
            continue;
        }

        if is_collecting_manuals {
            manuals.push(line.split(",").map(|str1| str1.parse::<u32>().expect("Could parse to u32")).collect::<Vec<u32>>());
        } else {
            let parts = line.split("|").map(|str1| str1.parse::<u32>().expect("Could parse to u32")).collect::<Vec<u32>>();
            safety_rules.push((parts[0], parts[1]));
        }
    }
    return (safety_rules, manuals);
}


/// Find page ids
fn find_page_id(safety_rules: &Vec<(u32, u32)>, manual: &Vec<u32>) -> Vec<(usize,usize)> {
    let mut page_ids = Vec::new();
    let mut page_hash = HashMap::new();
    for (i, page) in manual.iter().enumerate() {
        page_hash.insert(*page, i);
    }

    for safety_rule in safety_rules {
        if page_hash.contains_key(&safety_rule.0) && page_hash.contains_key(&safety_rule.1) {
            page_ids.push((page_hash[&safety_rule.0], page_hash[&safety_rule.1]));
        }
    }
    return page_ids;
}

/// Check if the manual is correct, by looking if the safety_rule_id.0 is before safety_rule_id.1
fn is_correct(safety_id: &Vec<(usize, usize)>) -> bool {
    safety_id.iter().all(|(s1,s2)| s1 < s2)
}

/// Fix the ordering by swapping the incorrect elements
fn fix_manual(safety_id: &Vec<(usize, usize)>, manual: &mut Vec<u32>, safety_rules: &Vec<(u32, u32)>) {
    let mut safety_id = safety_id.clone();
    while !is_correct(&safety_id) {
        for i in 0..safety_id.len() {
            if safety_id[i].0 > safety_id[i].1 {
                manual.swap(safety_id[i].0, safety_id[i].1);
                safety_id = find_page_id(&safety_rules, manual);
                break;
            }
        }
    }
}