use colored::Colorize;

use super::day12::Direction; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 16".bright_green().bold()); 
    let mut start_point = (0,0);
    let mut end_point = (0,0);
    let is_wall = input.lines().enumerate().map(|(i,line)| line.chars().enumerate().map(|(j,c)| match c {
        '#' => true,
        'S' => {start_point = (i,j); false},
        'E' => {end_point = (i,j); false},
        _ => false
    }).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();
    println!("Start: {:?}, End: {:?}", start_point, end_point);
    //*Part 1, maze
    // Turning costs 1000, moving costs 1
    let (part1, scores) = solve_maze(&is_wall, start_point, end_point);
    //* Part 2, find used path
    let visited = depth_first_walk(start_point, Direction::Right, &scores,&end_point).unwrap();
    let part2 = visited.len();
    print_maze(&visited, &is_wall);
    return (part1, part2); 
} 

/// Solve the maze <br>
/// Returns the lowest possible score, moving cost 1, turning cost 1000
fn solve_maze(is_wall: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> (usize, Vec<Vec<[usize;4]>>) {
    let mut scores = vec![vec![[usize::MAX;4]; is_wall[0].len()]; is_wall.len()];
    let mut to_visit = vec![(start,Direction::Right,0)];
    scores[start.0][start.1][3] = 0;
    while let Some((point,dir,_)) = to_visit.pop(){
        if point == end { break; }
        // Check ahead:
        match dir.get_next_index(point.0, point.1, scores.len(), scores[0].len(), 0, 0) {
            Some((i,j)) => {
                if !is_wall[i][j] && scores[i][j][dir.as_number()] > scores[point.0][point.1][dir.as_number()]+1 {
                    scores[i][j][dir.as_number()] = scores[point.0][point.1][dir.as_number()]+1;
                    to_visit.push(((i,j),dir,scores[i][j][dir.as_number()]));
                }
            },
            None => {},
        }
        // Rotate left and right
        let new_dir = dir.rotate_ccw(1);
        if scores[point.0][point.1][new_dir.as_number()] > scores[point.0][point.1][dir.as_number()]+1000 {
            scores[point.0][point.1][new_dir.as_number()] = scores[point.0][point.1][dir.as_number()]+1000;
            to_visit.push((point,new_dir,scores[point.0][point.1][new_dir.as_number()]));
        }
        let new_dir = dir.rotate_ccw(-1);
        if scores[point.0][point.1][new_dir.as_number()] > scores[point.0][point.1][dir.as_number()]+1000 {
            scores[point.0][point.1][new_dir.as_number()] = scores[point.0][point.1][dir.as_number()]+1000;
            to_visit.push((point,new_dir,scores[point.0][point.1][new_dir.as_number()]));
        }
        // sort to_visit
        to_visit.sort_by(|(_,_,a),(_,_,b)| b.cmp(a));
    }
    return (*scores[end.0][end.1].iter().min().unwrap(),scores);
}

fn depth_first_walk(current: (usize,usize), dir: Direction, scores: &Vec<Vec<[usize;4]>>, end: &(usize,usize)) -> Option<Vec<(usize,usize)>>{
    if current == *end {return Some(vec![*end])}
    let mut has_visited = Vec::new();
    // Look around, and go in every lower score
    if let Some((i,j)) = dir.get_next_index(current.0, current.1, scores.len(), scores[0].len(), 0, 0){
        if scores[i][j][dir.as_number()] != usize::MAX && scores[i][j][dir.as_number()] > scores[current.0][current.1][dir.as_number()]{
            if let Some(new_points) = depth_first_walk((i,j), dir, scores,end){
                insert_sorted(&mut has_visited, &new_points);
            }
        }
    }
    // Rotate left and right
    let new_dir = dir.rotate_ccw(1);
    if scores[current.0][current.1][new_dir.as_number()] != usize::MAX && scores[current.0][current.1][new_dir.as_number()] > scores[current.0][current.1][dir.as_number()] {
        if let Some(new_points) = depth_first_walk(current, new_dir, scores,end){
            insert_sorted(&mut has_visited, &new_points);
        }
    }
    let new_dir = dir.rotate_ccw(-1);
    if scores[current.0][current.1][new_dir.as_number()] != usize::MAX && scores[current.0][current.1][new_dir.as_number()] > scores[current.0][current.1][dir.as_number()] {
        if let Some(new_points) = depth_first_walk(current, new_dir, scores, end){
            insert_sorted(&mut has_visited, &new_points);
        }
    }
    if has_visited.is_empty() {
        return None
    } else {
        insert_sorted(&mut has_visited, &vec![current]);
        Some(has_visited)
    }
}

fn insert_sorted(has_visited: &mut Vec<(usize, usize)>, new_points: &Vec<(usize, usize)>) {
    for el in new_points{
        match has_visited.binary_search(&el) {
            Ok(_) => {/* Already exists in the vector */},
            Err(pos) => {has_visited.insert(pos, *el);},
        }
    }
}

fn print_maze(has_visited: &Vec<(usize, usize)>, is_wall: &Vec<Vec<bool>>) {
    let mut maze = vec![vec!['.';is_wall[0].len()];is_wall.len()];
    for cell in has_visited{
        maze[cell.0][cell.1] = 'O';
    }
    for i in 0..is_wall.len() {
        for j in 0..is_wall[i].len() {
            if is_wall[i][j] {print!("{}", "#".to_string().truecolor(100, 100, 100));continue;}
            if maze[i][j] == '.' {print!("{}", ".".to_string().truecolor(70, 70, 70));continue;}
            print!("{}",maze[i][j].to_string().truecolor(200, 100, 0));
        }
        println!("");
    }
}