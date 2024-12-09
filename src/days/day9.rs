use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 9".bright_green().bold()); 
    let (expanded_storage, free_spaces, file_blocks) = parse_input(input);
    let part1 = solve_part1(&expanded_storage);
    let part2 = solve_part2(expanded_storage, file_blocks, free_spaces);
    return (part1, part2); 
}

fn solve_part2(expanded_storage: Vec<usize>, file_blocks: Vec<(usize, usize)>,mut free_spaces: Vec<(usize, usize)>) -> usize {
    let mut expanded_storage = expanded_storage.clone();
    for (index, length) in file_blocks.iter().rev() {
        // Find where to move file block
        for (index_free_spaces,(i_free, length_free)) in free_spaces.iter().enumerate() {
            if *i_free > *index { break; }
            if *length_free >= *length{
                // Move file block to free space
                for i in 0..*length {
                    expanded_storage.swap(*i_free+i, *index+i);
                }
                // Update free spaces
                if *length_free == *length {free_spaces.remove(index_free_spaces); break;}
                free_spaces[index_free_spaces] = (i_free+length,*length_free-length);
                break;
            }
        }
    }
    let part2 = calc_checksum(&expanded_storage);
    part2
}

fn solve_part1(expanded_storage: &Vec<usize>) -> usize {
    let mut expanded_storage = expanded_storage.clone();
    // Swap the last file (not block) with the first empty space:  0..111 -> 01.11. -> 0111..
    for i in 0..expanded_storage.len() {
        if i == expanded_storage.len() {break}
        while expanded_storage[i] == usize::MAX {
            if i == expanded_storage.len() {break}
            expanded_storage.swap_remove(i);
        }
    }
    // Calculate filesystem checksum
    let part1 = calc_checksum(&expanded_storage);
    part1
}

fn calc_checksum(expanded_storage: &Vec<usize>) -> usize {
    let mut sum = 0;
    for i in 0..expanded_storage.len() {
        if expanded_storage[i] == usize::MAX {continue;}
        sum += i*expanded_storage[i];
    }
    return sum;
} 

/// Parse into a vector of file blocks. The every other number is the length of a file block (the other is length of empty space). Parse this into a vec that contains the indices of the files: 123 -> 0..111   (where '.' resembles empty space)
fn parse_input(input: String) -> (Vec<usize>, Vec<(usize,usize)>,Vec<(usize,usize)>) {
    let mut free_spaces: Vec<(usize,usize)> = vec![];  // Store where the free spaces are  (index, length)
    let mut file_blocks: Vec<(usize,usize)> = vec![];  // Store where the file blocks are  (index, length)
    let mut index = 0;
    let expanded_storage = input.chars().enumerate().map(|(i,c)| {
        let num = c.to_digit(10).expect("Could not parse char to digit") as usize;
        index += num;
        if i % 2 == 0 {if num > 0 {file_blocks.push((index-num,num))}; vec![i/2;num]} else {if num > 0 {free_spaces.push((index-num,num))}; vec![usize::MAX;num]} 
    }).flatten().collect::<Vec<usize>>();
    // Trim free space
    if free_spaces.last().unwrap().0 > file_blocks.last().unwrap().0 {free_spaces.pop();}
    (expanded_storage, free_spaces, file_blocks)
}