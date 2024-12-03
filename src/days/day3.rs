use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 3".bright_green().bold()); 
    let part1 = extract_correct_mul(&input);
    let part2 = extract_correct_mul_part2(&input);
    return (part1, part2); 
} 

fn extract_correct_mul(input: &String) -> usize{
    let mut sum = 0;
    let mut stack: Vec<char> = vec![];
    let mut number: Vec<String> = vec!["".to_string();2];
    let mut i = 0;
    for c in input.chars(){
        if stack.first() == None {
            if c == 'm' {
                stack.push(c);
            }
            continue;
        }
        match stack.last().unwrap_or(&' ') {
            'm' => if c == 'u' {stack.push(c); continue;},
            'u' => if c == 'l' {stack.push(c); continue;},
            'l' => if c == '(' {stack.push(c); continue;},
            '(' => if c.is_digit(10) {stack.push('n'); number[i].push(c); continue;},
            'n' => if c.is_digit(10) {stack.push('n'); number[i].push(c); continue;} else if c == ',' && i==0 {i = 1; continue;} else if i==1 && c == ')' {
                // parse numbers
                let a = number[0].parse::<usize>().expect("Could not parse into usize");
                let b = number[1].parse::<usize>().expect("Could not parse into usize");
                sum += a*b;
            },
            _ => {},
        }
        // Invalid character Reset stack
        number[0].clear();
        number[1].clear();
        stack.clear();
        i = 0;

    }   
    return sum
}

/// Same as before, but now don't extract when it says don't() and continue when it says do()
fn extract_correct_mul_part2(input: &String) -> usize{
    let mut sum = 0;
    let mut is_dont = false;    // is don't
    let mut stack: Vec<char> = vec![];  // 'g' denotes a digit
    let mut number: Vec<String> = vec!["".to_string();2];
    let mut i = 0;
    for c in input.chars(){
        if stack.first() == None {
            if c == 'm' && !is_dont {
                stack.push(c);
            }
            if c == 'd' {
                stack.push(c);
            }
            continue;
        }
        match stack.last().unwrap_or(&' ') {
            'm' => if c == 'u' {stack.push(c); continue;},
            'u' => if c == 'l' {stack.push(c); continue;},
            'l' => if c == '(' {stack.push(c); continue;},
            '(' => if stack[stack.len()-2] == 'o' && c == ')' {is_dont = false;} else if stack[stack.len()-2] == 't' && c == ')' {is_dont = true;} else if c.is_digit(10) {stack.push('g'); number[i].push(c); continue;},
            'g' => if c.is_digit(10) {stack.push('g'); number[i].push(c); continue;} else if c == ',' && i==0 {i = 1; continue;} else if i==1 && c == ')' {
                // parse numbers
                let a = number[0].parse::<usize>().expect("Could not parse into usize");
                let b = number[1].parse::<usize>().expect("Could not parse into usize");
                sum += a*b;
            },
            'd' => if c == 'o' {stack.push(c); continue;},
            'o' => if c == '(' || c == 'n' {stack.push(c); continue;},
            'n' => if c == '\'' {stack.push(c); continue;},
            '\'' => if c == 't' {stack.push(c); continue;},
            't' => if c == '(' {stack.push(c); continue;},
            _ => {},
        }
        // Invalid character Reset stack
        // println!("Clearing stack: {}, Stack: {:?}\t{}", c,stack,is_dont);
        number[0].clear();
        number[1].clear();
        stack.clear();
        i = 0;

    }   
    return sum
}
