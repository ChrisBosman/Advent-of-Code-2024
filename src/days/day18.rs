use colored::Colorize; 

const FIELD_SIZE: usize = 71;
const CHECK_ANSWER: bool = false;

pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 18".bright_green().bold()); 
    let bytes = input.lines().map(|line| line.split(",").map(|x| x.parse::<u8>().expect("Could not parse \"{x}\" to u8")).collect::<Vec<u8>>()).collect::<Vec<Vec<u8>>>();

    //* Part 1
    // Drop 1024 bytes, and then find shortest path from top left to bottom right
    let mut is_wall = vec![vec![false; FIELD_SIZE]; FIELD_SIZE];
    let mut dropped = 1024;
    drop_bytes(&bytes, &mut is_wall,dropped,0);

    // Find shortest path
    let part1 = solve_maze(&is_wall, (0,0), (FIELD_SIZE-1, FIELD_SIZE-1)).unwrap();    

    //* Part 2
    // Keep dropping more bytes until there is no path left to the exit
    let mut step = 64;
    let mut last_is_wall = is_wall.clone();
    let part2 = loop{
        if let Some(_) = solve_maze(&is_wall, (0,0), (FIELD_SIZE-1, FIELD_SIZE-1)){
            // If successful
            last_is_wall = is_wall.clone();  //? This is probably quite slow
            drop_bytes(&bytes, &mut is_wall, dropped+step,dropped);
            dropped += step;
        } else {
            dropped -= step;
            // If failed with a step size of 1, then it reached the end
            if step == 1 {break &bytes[dropped];}
            // If failed, decrease the step size
            step /= 2;
            is_wall = last_is_wall.clone();
        }
    };
    if CHECK_ANSWER{
        // Check answer
        let mut is_wall = vec![vec![false; FIELD_SIZE]; FIELD_SIZE];
        drop_bytes(&bytes, &mut is_wall,dropped,0);
        println!("Now it is still possible: {}, with last byte: {:?}", if let Some(res) = solve_maze(&is_wall, (0,0), (FIELD_SIZE-1, FIELD_SIZE-1)) {res} else {0}, bytes[dropped-1]);
        let mut is_wall = vec![vec![false; FIELD_SIZE]; FIELD_SIZE];
        drop_bytes(&bytes, &mut is_wall,dropped+1,0);
        println!("Now it isn't possible anymore: {}, with last byte: {:?}", if let Some(res) = solve_maze(&is_wall, (0,0), (FIELD_SIZE-1, FIELD_SIZE-1)) {res} else {0}, bytes[dropped]);
    }

    println!("Part 2:\n\t{}",format!("({},{})",part2[0],part2[1]).truecolor(150, 150, 150));

    return (part1, part2[0] as usize); 
}

fn drop_bytes(bytes: &Vec<Vec<u8>>, is_wall: &mut Vec<Vec<bool>>, amount: usize, start: usize) {
    for i in start..amount{
        if i == bytes.len() {break}
        is_wall[bytes[i][0] as usize][bytes[i][1] as usize] = true;
    }
} 

fn solve_maze(is_wall: &Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut scores = vec![vec![usize::MAX; is_wall[0].len()]; is_wall.len()];
    let mut to_visit = vec![start];
    scores[start.0][start.1] = 0;
    while let Some((i,j)) = to_visit.pop() {
        if (i,j) == end {return Some(scores[end.0][end.1])}
        // Look around
        if i > 0 && !is_wall[i-1][j] && scores[i-1][j] > scores[i][j]+1 {scores[i-1][j] = scores[i][j]+1; to_visit.push((i-1,j))}
        if i < is_wall.len()-1 && !is_wall[i+1][j] && scores[i+1][j] > scores[i][j]+1 {scores[i+1][j] = scores[i][j]+1; to_visit.push((i+1,j))}
        if j < is_wall[0].len()-1 && !is_wall[i][j+1] && scores[i][j+1] > scores[i][j]+1 {scores[i][j+1] = scores[i][j]+1; to_visit.push((i,j+1))}
        if j > 0 && !is_wall[i][j-1] && scores[i][j-1] > scores[i][j]+1 {scores[i][j-1] = scores[i][j]+1; to_visit.push((i,j-1))}

        // Sort to_visit
        to_visit.sort_by(|a,b| scores[b.0][b.1].cmp(&scores[a.0][a.1]));
    }

    return None;
}