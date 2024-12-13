use std::{collections::HashMap, fmt::{self, Display}};
use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 12".bright_green().bold()); 
    let matrix = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut has_visited = vec![vec![false; matrix[0].len()]; matrix.len()];
    let mut regions = vec![];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if has_visited[i][j] {continue;}
            // A new cell, explore surroundings 
            let region = explore_region(&matrix, i, j, &mut has_visited);
            regions.push(region);
        }
    }

    // Find area and perimeter
    let mut part1 = 0;
    let mut part2 = 0;
    // let mut regions_info: HashMap<(usize,usize),(usize,usize,Option<Vec<(usize,usize)>>)> = HashMap::new();  // Index by topleft most point, values: (area,outer_sides, inner regions)
    for region in &regions{
        let (area,perimeter) = find_area_perimeter(region);
        part1 += area * perimeter;
        let sides = find_sides(region);
        // regions_info.insert(region[0], (area,sides,some_inner_regions));
        println!("{}: {}*{}",matrix[region[0].0][region[0].1],area,sides);
        part2 += area*sides;
    }

    // // Calculate total sides
    // for (key,(area, outer_sides, some_inner_regions)) in regions_info.iter(){
    //     let mut sides: usize = *outer_sides;
    //     if let Some(inner_regions) = some_inner_regions{
    //         // Need to add the inner edges
    //         for region in inner_regions{
    //             if let Some((_,inner_side,_)) = regions_info.get(&region){
    //                 sides += *inner_side;
    //             }
    //         }
    //         println!("{}: It has inner sides", matrix[key.0][key.1])
    //     }
    //     part2 += area*sides;
    // }

    return (part1, part2); 
} 

/// Explores the region of touching chars, and returns the list of visited cells
fn explore_region(matrix: &Vec<Vec<char>>, i: usize, j: usize, has_visited: &mut Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let mut region: Vec<(usize, usize)> = vec![];
    let mut to_visit: Vec<(usize, usize)> = vec![(i,j)];

    while !to_visit.is_empty() {
        let (i,j) = to_visit.pop().unwrap();
        has_visited[i][j] = true;
        region.push((i,j));
        // Look around
        if i > 0 && matrix[i-1][j] == matrix[i][j] && !has_visited[i-1][j] && !to_visit.contains(&(i-1,j)){to_visit.push((i-1,j));}
        if j > 0 && matrix[i][j-1] == matrix[i][j] && !has_visited[i][j-1] && !to_visit.contains(&(i,j-1)){to_visit.push((i,j-1));}
        if i < matrix.len()-1 && matrix[i+1][j] == matrix[i][j] && !has_visited[i+1][j] && !to_visit.contains(&(i+1,j)){to_visit.push((i+1,j));}
        if j < matrix[i].len()-1 && matrix[i][j+1] == matrix[i][j] && !has_visited[i][j+1] && !to_visit.contains(&(i,j+1)){to_visit.push((i,j+1));}
    }
    return region
}


/// Finds the area and perimeter <br>
/// Returns (area, perimeter)
fn find_area_perimeter(region: &Vec<(usize, usize)>) -> (usize, usize){ 
    let mut perimeter = 0;
    for i in 0..region.len(){
        // Find out how many cells are touching this cell
        let touching = region.iter().fold(0, |acc, (k,n)| {
            if      region[i].1 == *n && region[i].0 > 0 && region[i].0-1 == *k {return acc + 1}
            else if region[i].1 == *n &&                    region[i].0+1 == *k {return acc + 1}
            else if region[i].0 == *k && region[i].1 > 0 && region[i].1-1 == *n {return acc + 1}
            else if region[i].0 == *k &&                    region[i].1+1 == *n {return acc + 1}
            return acc;
        });
        if touching > 4 {println!("[Error] Cell is touching more than 4 cells, region {}, ({},{})",i, region[i].0, region[i].1)}
        perimeter += 4-touching
    }

    return (region.len(), perimeter); 
}

/// Finds how many (outer) sides it has <br>
/// It also returns which zones it encloses, so that the inner edges can be a simple lookup later
/// Returns (area, sides, Vec[enclosed zones(top left most point)])
fn find_sides(region: &Vec<(usize, usize)>) -> usize{
    let (mut r_mat, mut sides) = match find_outer_sides(region) {
        Ok(value) => value,
        Err(value) => return value,  // Quick escape
    };

    // Find enclosed zones
    let mut inner_zones = Vec::new();
    // Start by flooding the outside
    let max_len = (r_mat.len(), r_mat[0].len());
    for k in 0..if max_len.0 > max_len.1 {max_len.0} else {max_len.1} {
        if k < max_len.1{
            if !r_mat[0][k]             { flood(&mut r_mat, (0,k)); }
            if !r_mat[max_len.0-1][k] { flood(&mut r_mat, (max_len.0-1,k)); }
        }
        if k < max_len.0{
            if !r_mat[k][0]                { flood(&mut r_mat, (k,0)); }
            if !r_mat[k][max_len.1-1] { flood(&mut r_mat, (k,max_len.1-1)); }
        }
    }

    // Now find the enclosed zones, and fill the ones that where found
    for i in 0..r_mat.len(){
        for j in 0..r_mat[0].len(){
            if r_mat[i][j] { continue; }
            // Found a zone
            inner_zones.push(flood(&mut r_mat, (i,j)));
        }
    }
    // Find the edges of the extracted zones:
    for zone in inner_zones{
        sides += match find_outer_sides(&zone) {
            Ok((_,value)) => value,
            Err(value) => value,
        };
    }
    
    return sides
}

/// Find the outer edges <br>
/// If it is size 0,1,2 it doesn't bother calculating the logical matrix centered on the region. And so returns Err()
fn find_outer_sides(region: &Vec<(usize, usize)>) -> Result<(Vec<Vec<bool>>, usize), usize> {
    match region.len() {
        0 => return Err(0),
        1 => return Err(4),
        2 => return Err(4),
        _ => {},
    }
    let (max_i,max_j, min_i, min_j) = region.iter().fold((0,0,usize::MAX,usize::MAX),|mut res, (i,j)| {
        if *i > res.0 { res.0 = *i}
        if *j > res.1 { res.1 = *j}
        if *i < res.2 { res.2 = *i}
        if *j < res.3 { res.3 = *j}
        res
    });
    let mut r_mat = vec![vec![false;max_j-min_j+1];max_i-min_i+1];
    for (i,j) in region{
        r_mat[*i-min_i][*j-min_j] = true;
    }
    let mut sides = 0;
    let mut current = region[0];
    let mut dir = Direction::Left;
    loop {
        // First check cw, then continue checking ccw
        let mut rotations = -1;
        while rotations < 3{
            if let Some((i,j)) = dir.get_index_ccw(rotations, current.0, current.1, max_i+1, max_j+1,min_i, min_j){
                if r_mat[i-min_i][j-min_j] {
                    // println!("({},{}) -> ({i},{j})",current.0,current.1);
                    dir.rotate_ccw_overwrite(rotations);
                    current.0 = i;
                    current.1 = j;
                    break;
                }
            }
            rotations += 1
        }

        // Calculate how many sides have passed
        sides += rotations.abs() as usize;
        if current == region[0]{
            // Two cases, either it did truly reach the end, or it still has to move further (can only reach here if dir is left or up)
            if dir == Direction::Left {break;}
            // check if there is a block left of it
            if let Some((i,j)) = dir.get_index_ccw(-1, current.0, current.1, max_i+1, max_j+1,min_i, min_j){
                if r_mat[i-min_i][j-min_j] {
                    // There is a block to the left, we are not done yet
                    continue;
                }
            }
            // We did truly reach the end, but we are still missing the top side
            sides += 1;
            break;
        }
    }
    Ok((r_mat, sides))
}

/// Flood a logic matrix with 1, starting at start replacing all the neighbouring 0s (left,right,up and down, Not diagonally)
fn flood(matrix: &mut Vec<Vec<bool>>, start: (usize,usize)) -> Vec<(usize,usize)>{
    let mut region = Vec::new();
    let mut to_visit: Vec<(usize, usize)> = vec![start];
    while !to_visit.is_empty() {
        let (i,j) = to_visit.pop().unwrap();
        matrix[i][j] = true;
        region.push((i,j));
        // Look around
        if i > 0 && !matrix[i-1][j] && !to_visit.contains(&(i-1,j)){to_visit.push((i-1,j));}
        if j > 0 && !matrix[i][j-1] && !to_visit.contains(&(i,j-1)){to_visit.push((i,j-1));}
        if i < matrix.len()-1 && !matrix[i+1][j] && !to_visit.contains(&(i+1,j)){to_visit.push((i+1,j));}
        if j < matrix[i].len()-1 && !matrix[i][j+1] && !to_visit.contains(&(i,j+1)){to_visit.push((i,j+1));}
    }
    return region;
}

#[derive(PartialEq,PartialOrd,Clone, Copy)]
enum Direction {
    Up = 0,
    Left = 1,
    Down = 2,
    Right = 3,
}

impl Direction {
    #[allow(dead_code)]
    /// Calculates the difference in angle steps in the counter clockwise direction
    fn diff(&self, other: &Direction) -> u8{
        let mut add: u8 = 0;
        if self > other {add += 4}
        *other as u8 + add - *self as u8
    }

    /// Rotate Counter-clockwise by count*90degrees
    /// Count can not be higher than 252
    fn rotate_ccw(&self, count: i8) -> Direction{
        let new = *self as i8 + count;
        return Direction::from_number(new);
    }
    /// Rotate Counter-clockwise by count*90degrees
    /// Count can not be higher than 252
    fn rotate_ccw_overwrite(&mut self, count: i8) {
        let new = *self as i8 + count;
        *self = Direction::from_number(new);
    }

    /// Gets the index of the cell 90deg Counter-clockwise <br>
    /// max values are exclusive
    /// Return the index if it is in [min,max)
    fn get_index_ccw(&self, count: i8,i: usize, j: usize, max_i: usize, max_j: usize, min_i: usize, min_j: usize) -> Option<(usize,usize)>{
        // See which direction to check
        let dir = self.rotate_ccw(count);

        match dir {
            Direction::Up    if i > min_i   => {Some((i-1,j))},
            Direction::Left  if j > min_j   => {Some((i,j-1))},
            Direction::Down  if i+1 < max_i => {Some((i+1,j))},
            Direction::Right if j+1 < max_j => {Some((i,j+1))},
            _ => None
        }
    }

    fn from_number(mut num: i8) -> Direction{
        while num > 3 { num -= 4; }
        while num < 0 { num += 4; }
        match num {
            0 => {Direction::Up},
            1 => {Direction::Left},
            2 => {Direction::Down},
            3 => {Direction::Right},
            _ => {println!("{} Could not parse number to direction num: {num}","[from_number]".bright_red()); Direction::Up}
        }
    }
}

impl Into<u8> for Direction {
    fn into(self) -> u8{
        match self {
            Direction::Up => {0},
            Direction::Left => {1},
            Direction::Down => {2},
            Direction::Right => {3},
        }
    }
}

impl Display for Direction{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Direction::Up => "Up",
            Direction::Left => "Left",
            Direction::Down => "Down",
            Direction::Right => "Right",
        })
    }
}

