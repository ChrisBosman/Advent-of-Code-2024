use std::{env, fs, time::Instant};
use download_input::download_input;
use rand::Rng;
use colored::*;
mod days;
mod download_input;

const MAX_TERMINAL_LENGTH: usize = 100;
const STARS: usize = 8;


fn main() {
    print_begin_text();
    // Command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Pass in the day number (1, 2, etc)\nOr pass in \"test\" + day number to run the test of that day");
        return;
    }
    let test_mode = args.contains(&"test".to_string()) || args.contains(&"t".to_string());
    // if test_mode {env::set_var("RUST_BACKTRACE", "1");}
    let mut day: u8 = 0;
    for arg in args.iter().skip(1) {
        if arg == "test" || arg == "t" { continue; }
        day = arg.parse::<u8>().unwrap_or(0);
        break;
    } 
    let path = if test_mode
    { format!("inputs_tests/day{}.txt", day) } else { format!("inputs/day{}.txt", day) };

    // Read input
    let input = match fs::read_to_string(&path) {
        Ok(input) => {input},
        Err(_) => {
            print!("{}\n{}",format!("Input for day {} was not found", day).bright_red(),"Fetching file online ...".truecolor(100, 100, 100));
            download_input(day, "2024");
            println!("{}"," Done".truecolor(100, 100, 100));
            fs::read_to_string(&path).expect("Something gone wrong with downloading the file")
        },
    };
 
    // Get the true solution of the test
    let sol: Vec<usize> =
    if test_mode {
        // Find the solution
        let sols = fs::read_to_string("test_solutions.txt").expect("File {test_solutions.txt} not found");
        let sols = sols.lines().collect::<Vec<&str>>();
        let sols = sols.get((day-1) as usize).unwrap_or(&"Day00:").split(&[':',','][..]).skip(1)
            .map(|s| s.trim().parse::<usize>().unwrap_or_else(|_| {println!(" {}, invalid number: \"{}\"", format!("Invalid solution for day {}",day).bright_red(),s); std::process::exit(0)}))
            .collect::<Vec<usize>>();
        if sols.len() > 2 {
            println!("{}, more than 2 solutions were provided", format!("Invalid solution for day {}",day).bright_red());
            return;
        }
        sols
    } else {vec![]};

    let now = Instant::now();    
    // Running the day
    let result = match day {
        0 => {println!("{}","Invalid day argument".bright_red());return;},
        1 => days::day1::run(input),
        2 => days::day2::run(input),
        3 => days::day3::run(input),
        4 => days::day4::run(input),
        5 => days::day5::run(input),
        6 => days::day6::run(input),
        7 => days::day7::run(input),
        8 => days::day8::run(input),
        9 => days::day9::run(input),
        10 => days::day10::run(input),
        11 => days::day11::run(input),
        12 => days::day12::run(input),
        13 => days::day13::run(input),
        14 => days::day14::run(input),
        15 => days::day15::run(input),
        16 => days::day16::run(input),
        17 => days::day17::run(input),
        18 => days::day18::run(input),
        19 => days::day19::run(input),
        20 => days::day20::run(input),
        21 => days::day21::run(input),
        22 => days::day22::run(input),
        23 => days::day23::run(input),
        24 => days::day24::run(input),
        25 => days::day25::run(input),
        _ => {println!("{}",format!("Day {} was not found", day).bright_red());return;},
    };

    // Validate the outputs
    if test_mode {
        if sol[0] != result.0 {
            println!("Part 1: ❌\n\t{}",format!("Expected: {}, got: {}", sol[0], result.0).truecolor(100,100,100));
        }else{
            println!("Part 1: ✅");
        }
        if sol.len() > 1 {
            if sol[1] != result.1 {
                println!("Part 2: ❌\n\t{}",format!("Expected: {}, got: {}", sol[1], result.1).truecolor(100,100,100));
            }else{
                println!("Part 2: ✅");
            }
        }
    }else{
        // Print the results
        println!("Part 1:\n  {}",format!("{}", result.0).truecolor(100,100,100));
        println!("Part 2:\n  {}",format!("{}", result.1).truecolor(100,100,100));
    }
    println!("\n{}",format!("Elapsed time {}s",now.elapsed().as_secs_f32()).truecolor(0, 100, 100));
}

fn print_begin_text() {
    println!("\n{:⭐<2$}{:🌑<3$}","","",STARS,50-STARS);
    print_snow();
    let title = " ⁎⁑⁎⁎   ⁎⁑⁑⁎ Advent of Code 2024! ⁑⁑⁎ ⁎⁑⁎⁑⁎  ⁎";
    let spacing = (MAX_TERMINAL_LENGTH+title.chars().count())/2;
    println!("{}",format!("⫷{:≡<1$}⫸","",MAX_TERMINAL_LENGTH-2).truecolor(122, 122, 122));
    println!("{:>spacing$}",title.truecolor(0, 255, 136).bold());
}

fn print_snow() {
    let mut rng = rand::thread_rng();
    let char_arr = ['⁜','※','⁑','⁑','⁑','⁎','⁎','⁎','⁎','⁎','*','*'];
    let char_arr2 = ['⁂','⁂','⁑','⁎','⁎','⁎'];
    
    for _ in 0..6 {
        for _ in 0..MAX_TERMINAL_LENGTH{
            print!("{}",char_arr.get(rng.gen_range(0..200)).unwrap_or(&' '));
        }
        print!("\n");
    }
    for _ in 0..MAX_TERMINAL_LENGTH{
        print!("{}",char_arr2.get(rng.gen_range(0..15)).unwrap_or(&' '));
    }
    print!("\n");
    }
