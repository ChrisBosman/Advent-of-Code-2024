use colored::Colorize; 
use std::sync::mpsc::channel;
use rayon::prelude::*;
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 7".bright_green().bold()); 
    let (test_vals, equations) = parse_input(input);
    //* Part 1 & 2  
    // See which equations are true when either + or * is inserted between the numbers
    // For part two, it is with +,* and concat numbers together
    let mut part1: Vec<usize> = vec![];
    let mut part2: Vec<usize> = vec![];
    let (sender, receiver) = channel();
    test_vals.into_par_iter().zip(equations.par_iter()).for_each_with(sender,|s,(answer, equation)| {
        s.send((if check_eq(&answer, equation) {answer} else {0},
                if check_eq_part2(&answer, equation) {answer} else {0})).unwrap();
    });
    receiver.iter().for_each(|(p1, p2)| {part1.push(p1); part2.push(p2);});
    return (part1.iter().sum(), part2.iter().sum()); 
}

fn parse_input(input: String) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut test_vals = vec![];
    let mut equations = vec![];
    let lines = input.lines().map(|line| line.split(": ").collect::<Vec<&str>>());
    for line in lines{
        let mut vec1 = vec![];
        test_vals.push(line[0].parse::<usize>().expect("Could not parse to usize"));
        let mut parts = line[1].split(" ").map(|str1| str1.parse::<usize>().expect("Could not parse to usize"));
        while let Some(part) = parts.next() {
            vec1.push(part);
        }
        equations.push(vec1);
    }
    (test_vals, equations)
} 

fn check_eq(answer: &usize, equation: &Vec<usize>) -> bool {
    if equation.is_empty() { return false;}
    let max_nr_spots = equation.len()-1;
    // Max number of options
    let max_num = f32::exp2(max_nr_spots as f32) as usize;
    for option in 0..max_num{
        let mut result = equation[0];
        // Convert option to binary
        let bin = (0..max_nr_spots).map(|i| (option & (1 << i)) != 0).collect::<Vec<bool>>();
        for (&num, &b) in equation.iter().skip(1).zip(bin.iter()) {
            result = if b {result + num} else {result * num};
        }
        if result == *answer {return true;}
    }
    return false;
}

fn check_eq_part2(answer: &usize, equation: &Vec<usize>) -> bool {
    if equation.is_empty() { return false;}
    let max_nr_spots = equation.len()-1;    
    // Max number of options
    let max_num = f32::powi(3_f32,max_nr_spots as i32) as usize;
    for option in 0..max_num{
        let mut result = equation[0];
        // Convert to base 3
        let tri = to_base_3(option, max_nr_spots);
        for (&num, &tri) in equation.iter().skip(1).zip(tri.iter()) {
            result = match tri {
                0 => result + num,
                1 => result * num,
                2 => (result.to_string() + &num.to_string()).parse().expect("[check_eq_part2] Could not parse to usize"),
                _ => {println!("Something went wrong in conversion to base 3"); return false;},
            }
        }
        if result == *answer {return true;}
    }
    return false;
}

pub fn to_base_3(mut num: usize, length: usize) -> Vec<usize> {
    let mut base_3 = vec![0; length];
    for i in 0..length {
        base_3[i] = num % 3;
        num /= 3;
    }
    base_3
}