use colored::Colorize; 
 
#[derive(PartialEq,Eq)]
enum Object {
    Wall,
    Open,
    Box,
    Robot
}
#[derive(PartialEq,Eq,Clone,Default)]
enum ObjectP2 {
    Wall,
    #[default]
    Open,
    BoxL,  // Left side of a box
    BoxR,  // Right side of a box
    Robot
}


#[derive(PartialEq,Eq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 15".bright_green().bold()); 
    let (directions, mut field, mut robot_pos) = parse_input(&input);
    let mut field_P2 = upscale_field(&field);
    let mut robot_pos_P2 = (robot_pos.0,robot_pos.1*2);
    //* Part1
    for dir in &directions{
        move_robot(&mut robot_pos, dir, &mut field);
    }
    display_field(&field);
    let part1 = calculate_part1_score(&field);
    
    //* Part2
    for dir in &directions{
        move_robot_p2(&mut robot_pos_P2, dir, &mut field_P2);
    }
    return (part1, 0); 
} 

fn upscale_field(field: &Vec<Vec<Object>>) ->  Vec<Vec<ObjectP2>>{
    let mut upscaled_field = vec![vec![ObjectP2::Open; field[0].len()*2];field.len()];
    for (i,row) in field.iter().enumerate(){
        for (j,el) in row.iter().enumerate(){
            match el {
                Object::Wall => {upscaled_field[i][j*2] = ObjectP2::Wall; upscaled_field[i][j*2+1] = ObjectP2::Wall},
                Object::Open => {upscaled_field[i][j*2] = ObjectP2::Open; upscaled_field[i][j*2+1] = ObjectP2::Open},
                Object::Box => {upscaled_field[i][j*2] = ObjectP2::BoxL;upscaled_field[i][j*2+1] = ObjectP2::BoxR},
                Object::Robot => {upscaled_field[i][j*2] = ObjectP2::Robot;},
            }
        }
    }
    return upscaled_field;
}

fn calculate_part1_score(field: &Vec<Vec<Object>>) -> usize{
    let mut score = 0;
    for (i,row) in field.iter().enumerate(){
        for (j,el) in row.iter().enumerate(){
            if *el == Object::Box{
                score += 100*i + j;
            }
        }
    }
    return score
}

fn move_robot(robot_pos: &mut (usize,usize), direction: &Direction, field: &mut Vec<Vec<Object>>){
    // Check if it can move
    let mut k = 1;
    loop {
        let next_pos = match get_next_pos(robot_pos, &direction, field.len(), field[0].len(), k) {
            Some(value) => value,
            None => break,
        };
        // Check if it is a free space
        if field[next_pos.0][next_pos.1] == Object::Open{
            // Move everything over (aka. only the 'first box' and move robot)
            field[next_pos.0][next_pos.1] = Object::Box;
            let next_pos = get_next_pos(robot_pos, &direction, field.len(), field[0].len(), 1).unwrap();  // This has already happened before, so can safely unwrap
            field[next_pos.0][next_pos.1] = Object::Robot;
            field[robot_pos.0][robot_pos.1] = Object::Open;
            *robot_pos = next_pos;
            break;
        }
        if field[next_pos.0][next_pos.1] != Object::Box{
            break
        }

        k+=1
    }
}

fn get_next_pos(robot_pos: &(usize, usize), direction: &Direction, col_length: usize, row_length: usize, k: usize) -> Option<(usize, usize)> {
    let next_pos = match *direction {
        Direction::Up => {
            if robot_pos.0.checked_sub(k)? > 0 {
                (robot_pos.0-k,robot_pos.1)
            } else {return None}},
        Direction::Left => {
            if robot_pos.1.checked_sub(k)? > 0 {
                (robot_pos.0,robot_pos.1-k)
            } else {return None}},
        Direction::Down => {
            if robot_pos.0+k <col_length-1 {
                (robot_pos.0+k,robot_pos.1)
            } else {return None}},
        Direction::Right => {
            if robot_pos.1+k < row_length-1 {
                (robot_pos.0,robot_pos.1+k)
            } else {return None}},
    };
    Some(next_pos)
}

fn parse_input(input: &String) -> (Vec<Direction>, Vec<Vec<Object>>, (usize,usize)) {
    let mut field = Vec::new();
    let mut robot_pos = (0,0)
;   let mut lines = input.lines().enumerate();
    while let Some((i,line)) = lines.next(){
        if line.is_empty() { break;}
        let mut row = Vec::new();
        for (j,c) in line.chars().enumerate(){
            match c {
                '#' => row.push(Object::Wall),
                'O' => row.push(Object::Box),
                '@' => {
                    robot_pos = (i,j);
                    row.push(Object::Robot);
                },
                _ => row.push(Object::Open),
            }
        }
        field.push(row);
    }

    // Parse directions
    let mut directions = Vec::new();
    while let Some((_,line)) = lines.next(){
        for c in line.chars(){
            match c {
                '^' => directions.push(Direction::Up),
                '<' => directions.push(Direction::Left),
                'v' => directions.push(Direction::Down),
                '>' => directions.push(Direction::Right),
                _ => println!("Unexpected char \"{c}\""),
            }
        }
    }
    return (directions, field, robot_pos)
}

fn display_field(field: &Vec<Vec<Object>>){
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            match field[i][j] {
                Object::Wall => print!("#"),
                Object::Open => print!("."),
                Object::Box => print!("O"),
                Object::Robot => print!("@"),
            }
        }
        println!("");
    }
}

fn move_robot_p2(robot_pos: &mut (usize,usize), direction: &Direction, field: &mut Vec<Vec<ObjectP2>>){
    // Check if it can move
    let next_pos = match get_next_pos(robot_pos, &direction, field.len(), field[0].len(), 1) {
        Some(value) => value,
        None => return,
    };
    
}

fn check_next(current_pos: &(usize,usize), direction: &Direction, field: &Vec<Vec<ObjectP2>>) -> Option<Vec<(usize,usize)>>{
    let next_pos = match get_next_pos(current_pos, direction, field.len(), field[0].len(), 1) {
        Some(value) => value,
        None => return None,
    };

    match field[next_pos.0][next_pos.1] {
        ObjectP2::Open => return Some(vec![*current_pos]),
        ObjectP2::Wall => return None,
        ObjectP2::Robot => return None,
        ObjectP2::BoxL => {
            // If moving horizontally
            if direction.is_horizontal(){
                // Skip one, and check next cell
                let vec1 = check_next(&get_next_pos(&next_pos, direction, field.len(), field[0].len(), 1).expect("Part of the box is in the wall"), direction, field);
                match vec1 {
                    Some(mut vector) => {(unique_push(&mut vector, current_pos)); return Some(vector);},  // todo Is wrong, should not add current like this, should only add the left side of a box
                    None => {return None},
                }
            }
            
            // let l = check_next(current_pos, direction, field)
            // let r = check_next((current_pos.0, current_pos.1), direction, field)
            None
        
        }, // TODO
        ObjectP2::BoxR => {None}, // TODO
    }
    
}

fn unique_push(vector: &mut Vec<(usize, usize)> , x: &(usize, usize)) {
    if vector.contains(x) { return; }
    vector.push(*x);
}

/// Concatenate vec2 after vec1 for every element of vec2 that is not in vec1  (overwrites vec1)
fn unique_concat_vectors(vec1: &mut Vec<(usize, usize)>, vec2: Vec<(usize, usize)>) {
    for el in vec2{
        if vec1.contains(&el) { continue; }
        vec1.push(el);
    }
}

impl Direction {
    fn is_horizontal(&self) -> bool{
        return  *self == Direction::Left || *self == Direction::Right;
    }
}