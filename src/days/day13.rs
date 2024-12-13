use colored::Colorize; 
 
 #[derive(Clone,Debug)]
struct ClawMachine {
    // Locations of the prize
    x: usize, 
    y: usize,
    // Movement vectors of the A and B buttons
    a_x: usize,
    a_y: usize,
    b_x: usize,
    b_y: usize
}

pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 13".bright_green().bold()); 
    let mut claw_machines = parse_input(input);
    //* Part1
    let mut part1 = 0;
    for claw_machine in &claw_machines{
        if let Some((a,b)) = get_a_b_counts(&claw_machine){
            if a > 100 || b > 100 {continue;}
            part1 += a*3 + b;
        }
    }
    //* Part2, Now add 10000000000000
    let mut part2 = 0;
    for claw_machine in claw_machines.iter_mut(){
        claw_machine.x += 10000000000000;
        claw_machine.y += 10000000000000;
        if let Some((a,b)) = get_a_b_counts(&claw_machine){
            if !check(&claw_machine,a,b) { continue;}
            part2 += a*3 + b;
        } 
    }
    return (part1, part2); 
}

fn parse_input(input: String) -> Vec<ClawMachine> {
    let mut claw_machines = Vec::new();
    let mut claw_machine = ClawMachine{x:0,y:0,a_x:0,a_y:0,b_x:0,b_y:0};
    let mut count: u8 = 0;
    for line in input.lines(){
        let mut num_str = String::new();
        for c in line.chars(){
            if c.is_digit(10){
                num_str.push(c);
            }
            else if num_str.len() > 0{
                count += 1;
                match count {
                    1 => claw_machine.a_x = num_str.parse::<usize>().unwrap(),
                    2 => claw_machine.a_y = num_str.parse::<usize>().unwrap(),
                    3 => claw_machine.b_x = num_str.parse::<usize>().unwrap(),
                    4 => claw_machine.b_y = num_str.parse::<usize>().unwrap(),
                    5 => claw_machine.x   = num_str.parse::<usize>().unwrap(),
                    6 => claw_machine.y   = num_str.parse::<usize>().unwrap(),
                    _ => {count = 0; claw_machines.push(claw_machine.clone());},
                }
                num_str.clear();
            }
        }
        count += 1;
        match count {
            1 => claw_machine.a_x = num_str.parse::<usize>().unwrap(),
            2 => claw_machine.a_y = num_str.parse::<usize>().unwrap(),
            3 => claw_machine.b_x = num_str.parse::<usize>().unwrap(),
            4 => claw_machine.b_y = num_str.parse::<usize>().unwrap(),
            5 => claw_machine.x   = num_str.parse::<usize>().unwrap(),
            6 => claw_machine.y   = num_str.parse::<usize>().unwrap(),
            _ => {count = 0; claw_machines.push(claw_machine.clone());},
        }
        num_str.clear();
    }
    claw_machines.push(claw_machine);

    return claw_machines;
}

/// ### Get A and B counts <br>
/// We know X = a_x * A + b_x * B <br>
/// and Y = a_y * A + b_y * B <br>
/// Two unknown (A and B) and two equation <br>
/// So solve for A and B
fn get_a_b_counts(claw_machine: &ClawMachine) -> Option<(usize, usize)>{
    let b = ((claw_machine.a_x*claw_machine.y) as f64 - (claw_machine.a_y*claw_machine.x) as f64)/((claw_machine.a_x*claw_machine.b_y) as f64 - (claw_machine.a_y*claw_machine.b_x) as f64);
    if b != b.round() {return None;}
    if b < 0_f64 {return None;}
    let a = (claw_machine.x/claw_machine.a_x).checked_sub((b as usize)*claw_machine.b_x/claw_machine.a_x)?;
    return Some((a, b as usize));
}

/// ### Check if A and B are valid
fn check(claw_machine: &ClawMachine, a: usize, b: usize) -> bool{
    let x = a*claw_machine.a_x + b*claw_machine.b_x;
    let y = a*claw_machine.a_y + b*claw_machine.b_y;
    return x == claw_machine.x && y == claw_machine.y;
}