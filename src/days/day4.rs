use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 4".bright_green().bold()); 
    let matrix = input.lines().map(|line | line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let part1 = count_xmas(&matrix)/2;  // Every word has been detected twice
    let part2 = count_x_mas(&matrix);
    return (part1, part2); 
} 

/// Find and Count all x shaped mas (written forward or backward) <br>
///  M . S <br>
///  . A . <br>
///  M . S <br>
fn count_x_mas(matrix: &Vec<Vec<char>>) -> usize{
    let mut count = 0;
    for i in 0..matrix.len(){
        for j in 0..matrix[i].len(){
            if matrix[i][j] == 'A'{
                let mut to_find = (' ',' ');
                // Look diagonally up left
                if i > 0 && j > 0 && (matrix[i-1][j-1] == 'M' || matrix[i-1][j-1] == 'S') {
                    to_find.0 = if matrix[i-1][j-1] == 'M' {'S'} else {'M'};
                } else {continue;}
                // Look diagonally up right
                if i > 0 && j < matrix[i].len()-1 && (matrix[i-1][j+1] == 'M' || matrix[i-1][j+1] == 'S') {
                    to_find.1 = if matrix[i-1][j+1] == 'M' {'S'} else {'M'};
                } else {continue;}
                // Look diagonally down right
                if !(i < matrix.len()-1 && j < matrix[0].len()-1 && matrix[i+1][j+1] == to_find.0){
                    continue;
                }
                // Look diagonally down left
                if !(i < matrix.len()-1 && j > 0 && matrix[i+1][j-1] == to_find.1){
                    continue;
                }
                count += 1;
            }
        }
    }
    return count
}


/// Find xmas horizontal, vertical, diagonal and backward 
fn count_xmas(matrix: &Vec<Vec<char>>) -> usize{
    let mut count = 0;
    for i in 0..matrix.len(){
        for j in 0..matrix[i].len(){
            let mut is_forward = None;
            let mut to_find = ' ';
            // Starts with either an X or S  (xmas or samx)
            if matrix[i][j] == 'X'{
                is_forward = Some(true);
                to_find = 'M';
            }
            if matrix[i][j] == 'S'{
                is_forward = Some(false);
                to_find = 'A';
            }
            if let Some(is_forward) = is_forward{
                // Go over all the directions
                if i > 0 && j > 0 && matrix[i-1][j-1] == to_find{
                    // Diagonal up left
                    if search_word(&matrix, "UL", i, j, if is_forward {0} else {3}, is_forward){
                        count += 1;
                    }
                }
                if i > 0 && matrix[i-1][j] == to_find{
                    // Up
                    if search_word(&matrix, "U", i, j, if is_forward {0} else {3}, is_forward){
                        count += 1;
                    }
                }
                if i > 0 && j < matrix[i].len()-1 && matrix[i-1][j+1] == to_find{
                    // Diagonal up right
                    if search_word(&matrix, "UR", i, j, if is_forward {0} else {3}, is_forward){
                        count += 1;
                    }
                }
                if j > 0 && matrix[i][j-1] == to_find{
                    // Left
                    if search_word(&matrix, "L", i, j, if is_forward {0} else {3}, is_forward){
                        count += 1;
                    }
                }
                if j < matrix[i].len()-1 && matrix[i][j+1] == to_find{
                    // Right
                    if search_word(&matrix, "R", i, j, if is_forward {0} else {3}, is_forward){
                        count += 1;
                    }
                }
                if i < matrix.len()-1 && j > 0 && matrix[i+1][j-1] == to_find{
                    // Diagonal down left
                    if search_word(&matrix, "DL", i, j, if is_forward {0} else {3}, is_forward){
                        count += 1;
                    }
                }
                if i < matrix.len()-1 && matrix[i+1][j] == to_find{
                    // Down
                    if search_word(&matrix, "D", i, j, if is_forward {0} else {3}, is_forward){
                        count += 1;
                    }
                }
                if i < matrix.len()-1 && j < matrix[i].len()-1 && matrix[i+1][j+1] == to_find{
                    // Diagonal down right
                    if search_word(&matrix, "DR", i, j, if is_forward {0} else {3}, is_forward){
                        count += 1;
                    }
                }
            }
        }
    }
    return  count;
}

fn search_word(matrix: &Vec<Vec<char>>, direction: &str, i: usize, j: usize, current_char_index: i8, is_forward: bool) -> bool{
    let next_char_index = current_char_index + if is_forward {1} else {-1};
    let next_char = "XMAS".chars().nth(next_char_index as usize).unwrap_or('d');
    if next_char == 'd' {return true}

    match direction {
        "UL" => if i > 0 && j > 0 && matrix[i-1][j-1] == next_char                              {return search_word(matrix, direction, i-1, j-1, next_char_index, is_forward)},
        "U" => if i > 0 && matrix[i-1][j] == next_char                                          {return search_word(matrix, direction, i-1, j, next_char_index, is_forward)},
        "UR" => if i > 0 && j < matrix[i].len()-1 && matrix[i-1][j+1] == next_char              {return search_word(matrix, direction, i-1, j+1, next_char_index, is_forward)},
        "L" => if j > 0 && matrix[i][j-1] == next_char                                          {return search_word(matrix, direction, i, j-1, next_char_index, is_forward)},
        "R" => if j < matrix[i].len()-1 && matrix[i][j+1] == next_char                          {return search_word(matrix, direction, i, j+1, next_char_index, is_forward)},
        "DL" => if i < matrix.len()-1 && j > 0 && matrix[i+1][j-1] == next_char                 {return search_word(matrix, direction, i+1, j-1, next_char_index, is_forward)},
        "D" => if i < matrix.len()-1 && matrix[i+1][j] == next_char                             {return search_word(matrix, direction, i+1, j, next_char_index, is_forward)},
        "DR" => if i < matrix.len()-1 && j < matrix[i].len()-1 && matrix[i+1][j+1] == next_char {return search_word(matrix, direction, i+1, j+1, next_char_index, is_forward)},
        _ => println!("[Search_word] Wrong direction"),
    }

    return false
}