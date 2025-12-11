use std::{env, fs, time::Instant};
use download_input::download_input;
use rand::Rng;
use colored::*;
mod days;
mod download_input;
mod write_to_readme;

const MAX_TERMINAL_LENGTH: usize = 100;
const STARS: usize = 17;
const BENCHMARK_RUN_TIME: usize = 10000;  // Run time for the benchmarks (ms) (10s)

fn main() {
    print_begin_text();
    // Command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("How to use:\ncargo r <day number> [t] [b]\n\t- t or test\tTo run the test input\n\t- b\t\tTo run a benchmark");
        return;
    }
    let test_mode = args.contains(&"test".to_string()) || args.contains(&"t".to_string());
    let benchmark_mode = args.contains(&"b".to_string());
    // if test_mode {env::set_var("RUST_BACKTRACE", "1");}
    let mut day: u8 = 0;
    for arg in args.iter().skip(1) {
        if arg == "test" || arg == "t" { continue; }
        if arg == "b" { continue; }
        if arg == "all" { // Currently unused
            day = 255;
            break;
        }
        day = arg.parse::<u8>().unwrap_or(0);
        if day==0 {println!("{}: \"{arg}\"","[Error] Invalid day argument".bright_red());return;}
        break;
    } 
    let path = if test_mode
    { format!("inputs_tests/day{}.txt", day) } else { format!("inputs/day{}.txt", day) };

    // Read input
    let input = match fs::read_to_string(&path) {
        Ok(input) => {input},
        Err(_) => {
            print!("{}\n{}",format!("Input for day {} was not found", day).bright_red(),"Fetching file online ...".truecolor(100, 100, 100));
            download_input(day, "2025");
            println!("{}"," Done".truecolor(100, 100, 100));
            fs::read_to_string(&path).expect("[Error] Something gone wrong with downloading the file")
        },
    };
 
    if benchmark_mode {
        let score = match run_benchmark(day, input) {
            Some(val) => {val},
            None => {return;},
        };
        write_to_readme::write_benchmark(day, score);
        return;
    }

    // Get the true solution of the test
    let sol: Vec<usize> =
    if test_mode {
        // Find the solution
        let sols = fs::read_to_string("test_solutions.txt").expect("File {test_solutions.txt} not found");
        let sols = sols.lines().collect::<Vec<&str>>();
        let sols = sols.get((day-1) as usize).unwrap_or(&"Day00:").split(&[':',','][..]).skip(1)
            .map(|s| s.trim().parse::<usize>().unwrap_or_else(|_| {println!(" {}, invalid number: \"{}\"", format!("[Error] Invalid solution for day {}",day).bright_red(),s); std::process::exit(0)}))
            .collect::<Vec<usize>>();
        if sols.len() > 2 {
            println!("{}, more than 2 solutions were provided", format!("Invalid solution for day {}",day).bright_red());
            return;
        }
        sols
    } else {vec![]};

    let now = Instant::now();    
    // Running the day
    let result = match run_one_day(day, &input, true) {
        Some(value) => value,
        None => return,
    };
    let elapsed_time = now.elapsed().as_nanos();

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
    println!("\n{}",format!("Elapsed time {}",fancy_print_time_ns(elapsed_time as f32)).truecolor(0, 100, 100));
}

fn run_benchmark(day: u8, input: String) -> Option<String> {
    let now: Instant = Instant::now();    
    // Run onces to get the approximate time
    run_one_day(day, &input, false)?;
    let mut elapsed_time = now.elapsed().as_nanos();
    // See if we can rerun it within the allocated time
    let nr_runs = BENCHMARK_RUN_TIME as u128 / (elapsed_time/1e6 as u128);
    println!("Running {} times, it should take around: {}",&nr_runs, fancy_print_time_ns((nr_runs*elapsed_time) as f32));
    let now: Instant = Instant::now();    
    for _ in 0..nr_runs{
        run_one_day(day, &input, false)?;
    }
    elapsed_time += now.elapsed().as_nanos();
    elapsed_time = elapsed_time/(nr_runs+1);

    let time = elapsed_time as f32;
    let time_str = fancy_print_time_ns(time);
    println!("{}",format!("Average time: {time_str}").truecolor(0, 100, 100));
    return Some(time_str);
}

fn fancy_print_time_ns(mut time: f32) -> String{
    let mut units = "ns";
    if time > 1000_f32 {          
        time /= 1000_f32;
        units = "us"
    }
    if time > 1000_f32 {          
        time /= 1000_f32;
        units = "ms"
    }
    if time > 1000_f32 {
        time /= 1000_f32;
        units = "s"
    }
    if units == "s" && time > 60_f32 {
        time /= 60_f32;
        units = "m"
    }
    time = (time*100_f32).floor()/100_f32;
    time.to_string()+units
}

fn run_one_day(day: u8, input: &String, print_day_name: bool) -> Option<(usize, usize)> {
    if print_day_name {println!("{}",format!("Day {day}").bright_green().bold());}
    let result = match day {
        0 => {println!("{}","[Error] Invalid day argument".bright_red());return None;},
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
        _ => {println!("{}",format!("[Error] Day {} was not found", day).bright_red());return None;},
    };
    Some(result)
}

fn print_begin_text() {
    println!("\n\t\t\t  {:⭐<2$}{:🌑<3$}","","",STARS,24-STARS);
    print_snow();
    let title = " ⁎⁑⁎⁎   ⁎⁑⁑⁎ Advent of Code 2025! ⁑⁑⁎ ⁎⁑⁎⁑⁎  ⁎";
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
