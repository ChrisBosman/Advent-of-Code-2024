use std::fmt::Display;
use colored::Colorize; 
 
const FIELD_SIZE: (i32,i32) = (101,103);

#[derive(Clone,Debug)]
struct Robot {
    x: usize,
    y: usize,
    v_x: i32,
    v_y: i32
}

pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 14".bright_green().bold()); 
    let mut robots = parse_input(input);
    let part1 = solve_part1(robots.clone());

    //* Part2
    // Keep stepping until they arrange themselves as a christmas tree
    let mut part2 = 0;
    for i in 0..100000{
        for robot in robots.iter_mut() { robot.move_robot(1);}
        if check_tree(&robots) {
            display_field(&robots);
            part2 = i+1;
            break;
        }

    }

    return (part1, part2); 
}

fn solve_part1(mut robots: Vec<Robot>) -> usize {
    for i in 0..robots.len(){
        robots[i].move_robot(100);
    }
    // display_field(&robots);
    let mut quadrants = [0;4];
    for robot in robots.iter(){
        if robot.x < FIELD_SIZE.0 as usize/2 && robot.y < FIELD_SIZE.1 as usize/2 {quadrants[0] += 1;}
        if robot.x < FIELD_SIZE.0 as usize/2 && robot.y > FIELD_SIZE.1 as usize/2 {quadrants[1] += 1;}
        if robot.x > FIELD_SIZE.0 as usize/2 && robot.y < FIELD_SIZE.1 as usize/2 {quadrants[2] += 1;}
        if robot.x > FIELD_SIZE.0 as usize/2 && robot.y > FIELD_SIZE.1 as usize/2 {quadrants[3] += 1;}
    }
    let part1 = quadrants.iter().fold(1, |acc,x| acc*x);
    part1
} 

fn parse_input(input: String) -> Vec<Robot> {
    let mut robots = Vec::new();
    let mut robot = Robot{x:0,y:0,v_x:0,v_y:0};
    for line in input.lines(){
        let mut count: u8 = 0;
        let mut num_str = String::new();
        let mut is_negative = false;
        for c in line.chars(){
            if c == '-' {
                is_negative = true;
            }
            else if c.is_digit(10){
                num_str.push(c);
            }
            else if num_str.len() > 0{
                count += 1;
                let num = num_str.parse::<usize>().unwrap();
                match count {
                    1 => robot.x = num,
                    2 => robot.y = num,
                    3 => robot.v_x = if is_negative {-(num as i32)} else {num as i32},
                    _ => {count = 0; robots.push(robot.clone());},
                }
                num_str.clear();
                is_negative = false;
            }
        }
        let num = num_str.parse::<usize>().unwrap();
        robot.v_y = if is_negative {-(num as i32)} else {num as i32};
        num_str.clear();
        robots.push(robot.clone());
    }
    return robots;
}

impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, v_x: {}, v_y: {}", self.x, self.y, self.v_x, self.v_y)
    }
}

impl Robot {
    /// Move robot and keep it on the field using the modulus operator
    fn move_robot(&mut self, steps: i32) {
        let mut pos = (self.x as i32 + self.v_x*steps, self.y as i32 + self.v_y*steps);
        // Keep it in the field
        pos.0 = pos.0 % FIELD_SIZE.0;
        pos.1 = pos.1 % FIELD_SIZE.1;
        self.x = if pos.0 >= 0 {pos.0 as usize} else {(pos.0 + FIELD_SIZE.0) as usize};
        self.y = if pos.1 >= 0 {pos.1 as usize} else {(pos.1 + FIELD_SIZE.1) as usize};
    }
}

fn display_field(robots: &Vec<Robot>) {
    let mut field = [['.'; FIELD_SIZE.0 as usize]; FIELD_SIZE.1 as usize];

    // Add robots
    for robot in robots {
        field[robot.y as usize][robot.x as usize] = '#';
    }

    // Print field
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            if is_tree_high(&field, j,i) {
                if is_peak(&field, j,i){
                    print!("{}", "#".to_string().yellow().bold());continue;
                }
                print!("{}", "#".to_string().green().bold());continue;
            }
            print!("{}", field[i][j]);
        }
        println!("");
    }
}

fn is_tree_high(field: &[[char; 101]; 103], x: usize, y: usize) -> bool {
    // Check if there are 3 places cells
    if x > 0 && x < FIELD_SIZE.0 as usize-1 && y < FIELD_SIZE.1 as usize-1{
        return field[y][x] == '#' && field[y+1][x] == '#' && field[y+1][x+1] == '#' && field[y+1][x-1] == '#';
    }
    return false;
}

fn is_peak(field: &[[char; 101]; 103], x: usize, y: usize) -> bool {
    // Check if there are 3 places cells
    if x > 0 && x < FIELD_SIZE.0 as usize-1 && y > 0 && y < FIELD_SIZE.1 as usize-1{
        return field[y][x] == '#' && field[y+1][x] == '#' && field[y+1][x+1] == '#' && field[y+1][x-1] == '#' 
                && field[y][x-1] == '.' && field[y][x+1] == '.' && field[y-1][x] == '.' && field[y-1][x+1] == '.' && field[y-1][x-1] == '.';
    }
    return false;
}

fn check_tree(robots: &Vec<Robot>) -> bool {
    let mut field: [[i32; 101]; 103] = [[0; FIELD_SIZE.0 as usize]; FIELD_SIZE.1 as usize];

    // Add robots
    for robot in robots {
        field[robot.y as usize][robot.x as usize] += 1;
    }


    // Find head
    let mut potential_head = Vec::new();
    for i in 0..robots.len(){
        
        if check_left(&field, robots[i].x, robots[i].y, 5) && check_right(&field,  robots[i].x, robots[i].y, 5){
            potential_head.push(i);
        }
    }

    return !potential_head.is_empty()
}

fn check_left(field: &[[i32; 101]; 103], x: usize, y: usize, depth: usize )-> bool {
    if depth == 0 { return true;}
    if x < depth || y >= FIELD_SIZE.1 as usize-depth {return false}

    if field[y+1][x-1] == 0 {return false}

    return check_left(field, x-1, y+1, depth-1)
}

fn check_right(field: &[[i32; 101]; 103], x: usize, y: usize, depth: usize )-> bool {
    if depth == 0 { return true;}
    if x >= FIELD_SIZE.0 as usize-depth || y >= FIELD_SIZE.1 as usize-depth {return false}

    if field[y+1][x+1] == 0 {return false}

    return check_right(field, x+1, y+1, depth-1)
}