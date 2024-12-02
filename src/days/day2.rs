use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 2".bright_green().bold()); 
    let reports = input.lines().map(|line| line.split(" ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    let diff_arr = reports.iter().map(|x| diff(x.to_vec())).collect::<Vec<Vec<i32>>>();
    let is_safe = diff_arr.iter().map(|x| (x.iter().all(|&x| x > 0) || x.iter().all(|&x| x < 0)) && *x.iter().max().unwrap() <= 3 && *x.iter().min().unwrap() >= -3).collect::<Vec<bool>>();
    let part1 = is_safe.iter().fold(0, |acc,x| if *x {acc + 1} else {acc});
    //* Part 2
    let is_safe = reports.iter().map(|x| is_safe_part2_v2(x.to_vec())).collect::<Vec<bool>>();
    let part2 = is_safe.iter().fold(0, |acc,x| if *x {acc + 1} else {acc});
    return (part1, part2); 
} 

/// Brute force, try every option 
fn is_safe_part2_v2(vec1: Vec<u32>) -> bool {
    let diff = vec1.windows(2).map(|x| x[1] as i32 - x[0] as i32).collect::<Vec<i32>>();
    if (diff.iter().all(|&x| x > 0) || diff.iter().all(|&x| x < 0)) && *diff.iter().max().unwrap() <= 3 && *diff.iter().min().unwrap() >= -3 {return true};
    // println!("{:?}",vec1);
    for i in 0..vec1.len() {
        let mut vec2 = vec1.clone();
        vec2.remove(i);
        // println!("{:?}",vec2);
        let diff = vec2.windows(2).map(|x| x[1] as i32 - x[0] as i32).collect::<Vec<i32>>();
        if (diff.iter().all(|&x| x > 0) || diff.iter().all(|&x| x < 0)) && *diff.iter().max().unwrap() <= 3 && *diff.iter().min().unwrap() >= -3 {return true};
    }
    return false;
}

fn diff(vec1: Vec<u32>) -> Vec<i32> {
    vec1.windows(2).map(|x| x[1] as i32 - x[0] as i32).collect::<Vec<i32>>()
}
