use std::{collections::HashMap, sync::{mpsc::channel, Mutex}};
use rayon::prelude::*;
use colored::Colorize;
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 11".bright_green().bold()); 
    let mut stones = input.split(" ").map(|c| c.parse::<u64>().expect("Could not parse char to digit")).collect::<Vec<u64>>();
    // Part 1
    for _ in 0..25{
        step(&mut stones);
    }
    let part1 = stones.len();

    //* Part2  Part2 actually runs way quicker than part1, but for the sake of history it is included
    let visited: Mutex<HashMap<(u64,u8), usize>> = Mutex::new(HashMap::new());  // Hashmap((stone nr, steps left), stone_outcome).
    let (sender, receiver) = channel();
    stones.into_par_iter().for_each_with(sender,|s, stone| {
        s.send(determine_stone_outcome(stone, 50,&visited)).unwrap();
    });
    let part2 = receiver.iter().sum();
    return (part1, part2); 
} 

/// Part2 recursion based alternative to reduce memory usage  <br>
/// Takes a stone and returns how many stones it will split into
fn determine_stone_outcome(stone: u64, steps_left: u8, visited: &Mutex<HashMap<(u64,u8), usize>>) -> usize {
    if steps_left == 0 { return 1; }

    // Check if visited
    if let Ok(is_visited) = visited.lock() {
        if let Some(outcome) = is_visited.get(&(stone,steps_left)) {
            return *outcome;
        }
    }

    // Go over the stone rules:
    let outcome = match stone {
        0 => determine_stone_outcome(1, steps_left-1,visited),
        stone  => {
            let digits = get_largest_power10(stone);
            // If it has an even number of digits, split into two
            if let Some(factor) = digits {
                let left_side = stone/factor;
                let right_side = stone - left_side*factor;
                determine_stone_outcome(left_side, steps_left-1,visited) + determine_stone_outcome(right_side, steps_left-1,visited)
            } else{
                determine_stone_outcome(stone*2024, steps_left-1,visited)
            }
        },
    };

    // Add to the visited hashmap
    if let Ok(mut is_visited) = visited.lock() {
        is_visited.insert((stone,steps_left), outcome);
    }

    return outcome;

}

fn step(stones: &mut Vec<u64>) {
    let mut to_add: Vec<u64> = Vec::new();
    for i in 0..stones.len(){
        match stones[i] {
            0 => stones[i] = 1,
            stone  => {
                let digits = get_largest_power10(stone);
                // If it has an even number of digits, split into two
                if let Some(factor) = digits {
                    let left_side = stone/factor;
                    let right_side = stone - left_side*factor;
                    // Not like I really care about the order...
                    stones[i] = left_side;
                    to_add.push(right_side);
                } else{
                    stones[i] *= 2024;
                }
            },
        }
    }

    for i in 0..to_add.len(){
        stones.push(to_add[i]);
    }
}

/// Returns the largest power of 10 if it has an even number of digits (base 10)
/// <br> Modified from: https://github.com/FrankBosman/Advent_Of_Code_2024
fn get_largest_power10(num: u64) -> Option<u64> {
    // Calculate the amount of digits
    let mut magnitude = 1;
    while num >= 10u64.pow(magnitude) {
        magnitude += 1;
    }

    // Return the largest 10^n if it has an even number of digits
    return if magnitude % 2 == 0 { Some(10u64.pow(magnitude / 2)) } else { None }
}