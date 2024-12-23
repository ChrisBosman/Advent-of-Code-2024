use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 19".bright_green().bold()); 
    let (towels, patterns) = parse_input(&input);
    //* Part1      See if a pattern can be made with the available towels
    let mut part1 = 0;
    let mut part2 = 0;

    for pattern in patterns.iter(){
        let count = check_if_possible(&towels, pattern, 0);
        if count {
            part1 += 1;
        }
        println!("Finished: {}",pattern);
    }

    return (part1, part2); 
} 

/// Return if something is even possible
fn check_if_possible(towels: &Vec<&str>, pattern: &str, i: usize) -> bool {
    if i == pattern.len() { return true; }
    // Check which patterns can be used on this element, if none: return false
    for towel in towels.iter(){
        if towel.starts_with(pattern.get(i..i+1).unwrap()) {
            // Found a potential towel, check the rest
            if i+towel.len() > pattern.len() { continue; }
            if *towel == pattern.get(i..i+towel.len()).unwrap(){
                // It is the same
                if check_if_possible(towels, pattern, i+towel.len()) {return true;};
            }
        }
    }
    return false;
}

fn parse_input<'a>(input: &'a String) -> (Vec<&'a str>, Vec<&'a str>) { 
    let mut towels = Vec::new();
    let mut patterns = Vec::new();

    let mut lines = input.lines();
    // Extract the towel type (r, wr, b, g, ...)
    if let Some(line) = lines.next(){
        towels = line.split(", ").collect::<Vec<&str>>();
    }
    

    // Extract patterns (brwrr(newline)bggr)
    while let Some(line) = lines.next(){
        if line.is_empty() {continue;}
        patterns.push(line);
    }

    return (towels,patterns); 
}