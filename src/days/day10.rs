use colored::Colorize; 

#[derive(PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
    None
}
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 10".bright_green().bold()); 
    let (position_0, matrix) = parse_input(input);
    //* Part1
    // See how many paths go from a 0 to a unique 9, can only increase by 1 value, and can not go diagonally
    let mut part1 = 0;
    let mut part2 = 0;
    for pos in position_0 {
        part1 += explore_tile(pos, &matrix, Direction::None, &mut vec![]);
        part2 += explore_tile_part2(pos, &matrix, Direction::None);
    }
    return (part1, part2); 
}


/// Returns the position of all the 0s and a matrix of the height map
fn parse_input(input: String) -> (Vec<(usize, usize)>, Vec<Vec<u32>>) {
    let mut position_0: Vec<(usize,usize)> = Vec::new();
    let matrix = input.lines().enumerate().map(|(i,line)| line.chars().enumerate().map(|(j,c)| {
            let num = c.to_digit(10).expect(&format!("Could not parse \"{c}\" to digit"));
            if num == 0 {position_0.push((i,j))};
            num
        }).collect::<Vec<u32>>()).collect::<Vec<Vec<u32>>>();
    (position_0, matrix)
} 

/// ## Explore a tile<br>
/// Returns the number of paths towards a unique 9<br>
/// arrived_from: Direction from where you entered the tile (for instance, you moved down, then you arrived from the top)<br>
fn explore_tile(pos: (usize, usize), matrix: &Vec<Vec<u32>>, arrived_from: Direction, visited_9s: &mut Vec<(usize, usize)>) -> usize {
    if matrix[pos.0][pos.1] == 9 { 
        if visited_9s.contains(&pos) {return 0;}
        visited_9s.push(pos);
        return 1; 
    }

    // Look around
    let mut sum = 0;
    if pos.0 > 0 && arrived_from != Direction::Up && matrix[pos.0-1][pos.1] == matrix[pos.0][pos.1] + 1                         {sum += explore_tile((pos.0-1, pos.1), matrix, Direction::Down, visited_9s)}
    if pos.1 < matrix[pos.0].len()-1 && arrived_from != Direction::Right && matrix[pos.0][pos.1+1] == matrix[pos.0][pos.1] + 1  {sum += explore_tile((pos.0, pos.1+1), matrix, Direction::Left, visited_9s)}
    if pos.0 < matrix.len()-1 && arrived_from != Direction::Down && matrix[pos.0+1][pos.1] == matrix[pos.0][pos.1] + 1          {sum += explore_tile((pos.0+1, pos.1), matrix, Direction::Up, visited_9s)}
    if pos.1 > 0 && arrived_from != Direction::Left && matrix[pos.0][pos.1-1] == matrix[pos.0][pos.1] + 1                       {sum += explore_tile((pos.0, pos.1-1), matrix, Direction::Right, visited_9s)}

    return sum
}

/// ## Explore a tile, same as part1, but now go to all 9s <br>
/// Returns the number of paths towards a 9 <br>
/// arrived_from: Direction from where you entered the tile (for instance, you moved down, then you arrived from the top)<br>
fn explore_tile_part2(pos: (usize, usize), matrix: &Vec<Vec<u32>>, arrived_from: Direction) -> usize {
    if matrix[pos.0][pos.1] == 9 { return 1; }

    // Look around
    let mut sum = 0;
    if pos.0 > 0 && arrived_from != Direction::Up && matrix[pos.0-1][pos.1] == matrix[pos.0][pos.1] + 1                         {sum += explore_tile_part2((pos.0-1, pos.1), matrix, Direction::Down)}
    if pos.1 < matrix[pos.0].len()-1 && arrived_from != Direction::Right && matrix[pos.0][pos.1+1] == matrix[pos.0][pos.1] + 1  {sum += explore_tile_part2((pos.0, pos.1+1), matrix, Direction::Left)}
    if pos.0 < matrix.len()-1 && arrived_from != Direction::Down && matrix[pos.0+1][pos.1] == matrix[pos.0][pos.1] + 1          {sum += explore_tile_part2((pos.0+1, pos.1), matrix, Direction::Up)}
    if pos.1 > 0 && arrived_from != Direction::Left && matrix[pos.0][pos.1-1] == matrix[pos.0][pos.1] + 1                       {sum += explore_tile_part2((pos.0, pos.1-1), matrix, Direction::Right)}

    return sum
}