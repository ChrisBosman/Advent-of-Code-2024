use std::collections::HashMap;
use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 8".bright_green().bold()); 
    let (max_size, antenna_groups) = parse_input(input);
    //* Part 1
    // Look for anti_nodes (a node that is on the same line as two antennas but twice as far away from one compared to the other)
    let mut is_anti_node = vec![vec![false; max_size.0]; max_size.1];
    for (_, group) in antenna_groups.iter() {
        // Go over every possible combination between two antennas
        for i in 0..group.len(){
            for j in i+1..group.len(){
                // Find the anti-nodes
                let possible_places = find_anti_nodes(group[i], group[j], max_size);
                for k in 0..2{
                    if let Some(p) = possible_places[k]{
                        is_anti_node[p.0][p.1] = true;
                    }
                }
            }
        }
    }
    // Sum the anti-nodes
    let part1 = is_anti_node.iter().flatten().fold(0, |acc,&is_anti| if is_anti {acc+1} else {acc});

    //* Part 2
    // Now anti-nodes appear on every grid point that inline with two antennas
    is_anti_node = vec![vec![false; max_size.0]; max_size.1];
    for (_, group) in antenna_groups.iter() {
        // Go over every possible combination between two antennas
        for i in 0..group.len(){
            for j in i+1..group.len(){
                // Find the anti-nodes
                let possible_places = find_all_anti_nodes(group[i], group[j], max_size);
                
                for k in 0..2{
                    if let Some(vec1) = &possible_places[k]{
                        for p in vec1{
                            is_anti_node[p.0][p.1] = true;
                        }
                    }
                }
            }
        }
    }
    // Sum the anti-nodes
    let part2 = is_anti_node.iter().flatten().fold(0, |acc,&is_anti| if is_anti {acc+1} else {acc});

    return (part1, part2); 
}

fn find_all_anti_nodes(a1: (usize,usize), a2: (usize,usize),max_size: (usize,usize)) -> [Option<Vec<(usize,usize)>>;2]{
    let mut diff = sub_tuple_to_signed(a1, a2);
    // Reduce diff to lowest common denominator
    diff = reduce_to_lowest_common_denominator(diff);

    let mut c = a2;   // current position, starts at a2 instead of a1, so it also includes the points between the antennas
    let mut vec1 = vec![a2];
    while let Some(point) = add_tuple_checked(c, diff, max_size) {
        c = point;
        vec1.push(c);
    }
    let mut c = a2;   // current position
    let mut vec2 = vec![];
    while let Some(point) = sub_tuple_checked(c, diff, max_size) {
        c = point;
        vec2.push(c);
    }
    return [if vec1.is_empty() {None} else {Some(vec1)}, if vec2.is_empty() {None} else {Some(vec2)}];
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}
fn reduce_to_lowest_common_denominator(tuple: (isize, isize)) -> (isize, isize) {
    let gcd_value = gcd(tuple.0, tuple.1);
    (tuple.0 / gcd_value, tuple.1 / gcd_value)
}

fn find_anti_nodes(a1: (usize,usize), a2: (usize,usize),max_size: (usize,usize)) -> [Option<(usize,usize)>;2]{
    let diff = sub_tuple_to_signed(a1, a2);
    [add_tuple_checked(a1, diff, max_size),sub_tuple_checked(a2, diff, max_size)]
}

fn parse_input(input: String) -> ((usize, usize), HashMap<char, Vec<(usize, usize)>>) {
    let mut size = (0,0);
    let mut antenna_groups: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut i = 0;
    for line in input.lines() {
        if i == 0 {
            size.0 = line.len();
        }
        let mut chars = line.chars();
        let mut j = 0;
        while let Some(c) = chars.next() {
            if c == '.' { j += 1; continue; }
            antenna_groups.entry(c).and_modify(|group| group.push((i,j))).or_insert(vec![(i,j)]);
            j += 1;
        }
        i += 1;
    }
    size.1 = i;
    (size, antenna_groups)
} 

fn add_tuple_checked(t1: (usize,usize),t2: (isize,isize), max_size: (usize,usize)) -> Option<(usize,usize)>{
    let mut res = Some((
        if t2.0 > 0 { t1.0.checked_add(t2.0.abs() as usize)? } else { t1.0.checked_sub(t2.0.abs() as usize)? },
        if t2.1 > 0 { t1.1.checked_add(t2.1.abs() as usize)? } else { t1.1.checked_sub(t2.1.abs() as usize)? }
    ));

    if let Some((x, y)) = res {
        if x >= max_size.0 || y >= max_size.1 {
            res = None;
        }
    }
    return res;
}
fn sub_tuple_checked(t1: (usize,usize),t2: (isize,isize), max_size: (usize,usize)) -> Option<(usize,usize)>{
    let mut res = Some((
        if t2.0 > 0 { t1.0.checked_sub(t2.0.abs() as usize)? } else { t1.0.checked_add(t2.0.abs() as usize)? },
        if t2.1 > 0 { t1.1.checked_sub(t2.1.abs() as usize)? } else { t1.1.checked_add(t2.1.abs() as usize)? }
    ));

    if let Some((x, y)) = res {
        if x >= max_size.0 || y >= max_size.1 {
            res = None;
        }
    }
    return res;
}

fn sub_tuple_to_signed(t1: (usize,usize),t2: (usize,usize)) -> (isize,isize){
    (t1.0 as isize - t2.0 as isize,
     t1.1 as isize - t2.1 as isize)
}