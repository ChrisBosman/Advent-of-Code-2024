use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 1".bright_green().bold()); 
    // Parse input
    let input = input.lines().map(|line| line.split("   ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    let input_arr: (Vec<u32>, Vec<u32>) = (Vec::new(),Vec::new());
    for line in input {
        input_arr.0.push(line[0]);
        input_arr.1.push(line[1]);
    }

    input_arr.dbgr();

    return (0, 0); 
} 
