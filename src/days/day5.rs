use colored::Colorize; 
 
pub(crate) 
fn run(input: &String) -> (usize, usize){ 
    println!("{}","Day 5".bright_green().bold()); 

    let (ranges, ids) = parse_input(input);

    let sol1 = part1(&ranges, &ids);
    let merged = merge_ranges(&ranges);
    let sol2 = merged.iter().fold(0, |acc,el| acc + el[1]-el[0]+1);
    return (sol1, sol2 as usize); 
}

fn part1(ranges: &Vec<Vec<u64>>, ids: &Vec<u64>) -> usize {
    let mut sol1 = 0;
    for &id in ids{
        for range in ranges{
            if id >= range[0] && id <= range[1]{
                sol1 += 1;
                break;
            }
        }
    }
    sol1
}

fn parse_input(input: String) -> (Vec<Vec<u64>>, Vec<u64>) {
    let mut ranges: Vec<Vec<u64>> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();
    let mut collect_ids = false;
    for line in input.lines(){
        if line == "" {collect_ids = true; continue;}
        if collect_ids {
            ids.push(line.parse().expect(&format!("[Error] Can't parse {line}")));
        } else {
            ranges.push(line.split("-").map(|s| s.parse::<u64>().expect(&format!("[Error] Can't parse {s}"))).collect());
        }
    }
    (ranges, ids)
} 


fn merge_ranges(ranges: &Vec<Vec<u64>>) -> Vec<Vec<u64>>{
    let mut new_ranges = Vec::new();
    // Sort based on start
    let mut ranges = ranges.clone();
    ranges.sort_unstable_by_key(|v| v[0]);
    // Now the start is always after the start of the last one
    new_ranges.push(ranges[0].clone());
    for r in ranges{
        let last = new_ranges.last_mut().expect("[Error] Cannot get last value of \"new_range\"");
        if r[0] > last[1]{  // No Overlap
            new_ranges.push(r);
            continue;
        }
        // Overlap
        if r[1] <= last[1] {continue;} // Fully enclosed
        last[1] = r[1];
    }
    return new_ranges;
}


// Two ways to do part 2.
//   - Either merge all the overlapping ranges
//   - Or calculate the overlap and remove it

