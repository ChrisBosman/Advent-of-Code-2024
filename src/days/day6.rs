use colored::Colorize; 
 
 
#[derive(Default)]
pub enum Direction {
    #[default] Up,
    Left,
    Down,
    Right
}

pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 6".bright_green().bold()); 
    // parse data
    let mut current_pos = (0, 0, Direction::Up);
    let is_obstacle = input.lines().enumerate().map(|(i, line)| line.chars().enumerate().map(|(j,char1)| match char1 {
        '#' => {true},
        '.' => {false},
        '^' => {current_pos = (i, j, Direction::Up); false},
        ch => {panic!("Unknown char \"{}\"", ch)},
    }).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();

    //* Part 1
    // Follow the guard until he is out of bounds
    let mut has_visited = vec![vec![false; is_obstacle[0].len()]; is_obstacle.len()];
    has_visited[current_pos.0][current_pos.1] = true;
    'outer: while current_pos.0 < is_obstacle.len() && current_pos.1 < is_obstacle[0].len() && current_pos.0 >= 0 && current_pos.1 >= 0 {
        // Scan till the next obstacle
        if let Some(mut next_index) = current_pos.2.get_next_index(&(current_pos.0, current_pos.1), is_obstacle.len() - 1) {
            while !is_obstacle[next_index.0][next_index.1] {
                current_pos.0 = next_index.0;
                current_pos.1 = next_index.1;
                has_visited[current_pos.0][current_pos.1] = true;
                if let Some(next) = current_pos.2.get_next_index(&(current_pos.0, current_pos.1), is_obstacle.len() - 1) {
                    next_index.0 = next.0;
                    next_index.1 = next.1;
                } else { break 'outer; }
            }
        } else { break; }
        // There is an obstacle, turn right
        current_pos.2.turn_right();
    }
    let part1 = has_visited.iter().flatten().fold(0, |acc, &x| if x {acc + 1} else {acc});
    return (part1, 0); 
} 

fn print_logic_matrix(matrix: &Vec<Vec<bool>>) {
    for line in matrix {
        for ch in line {
            if *ch {print!("#")} else {print!(".")};
        }
        println!();
    }
}

impl Direction {
    pub fn turn_left(&mut self) {
        match *self {
            Direction::Up => { *self = Direction::Left; },
            Direction::Left => { *self = Direction::Down; },
            Direction::Down => { *self = Direction::Right; },
            Direction::Right => { *self = Direction::Up; },
        }
    }
    pub fn turn_right(&mut self) {
        match *self {
            Direction::Up => { *self = Direction::Right; },
            Direction::Right => { *self = Direction::Down; },
            Direction::Down => { *self = Direction::Left; },
            Direction::Left => { *self = Direction::Up; },
        }
    }
    fn get_next_index(&self, current_pos: &(usize, usize), upper_bound: usize) -> Option<(usize, usize)> {
        match *self {
            Direction::Up => { Some((current_pos.0.checked_sub(1)?, current_pos.1)) },
            Direction::Left => { Some((current_pos.0, current_pos.1.checked_sub(1)?)) },
            Direction::Down => { let next = current_pos.0 + 1; if next > upper_bound {None} else { Some((next, current_pos.1)) }},
            Direction::Right => { let next = current_pos.1 + 1; if next > upper_bound {None} else { Some((current_pos.0, current_pos.1 + 1)) }},
        }    
    }
    fn turn_right_no_overwrite(&mut self) -> Direction{
        match *self {
            Direction::Up => { Direction::Right },
            Direction::Right => { Direction::Down },
            Direction::Down => { Direction::Left },
            Direction::Left => { Direction::Up },
        }
    }
    fn turn_180_no_overwrite(&mut self) -> Direction{
        match *self {
            Direction::Up => { Direction::Down },
            Direction::Right => { Direction::Left },
            Direction::Down => { Direction::Up },
            Direction::Left => { Direction::Right },
        }
    }
}