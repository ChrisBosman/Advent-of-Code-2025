use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 3".bright_green().bold()); 

    let solution1 = part1(&input);
    let solution2 = part2(&input);

    
    return (solution1 as usize, solution2 as usize); 
}

fn part1(input: &String) -> u32 {
    let mut sum = 0;
    for line in input.lines(){
        let (first, index) = find_largest(line, 0, line.len()-1);
        let (second, _) = find_largest(line, index+1, line.len());
        sum += first*10 + second;
    }
    sum
}

fn part2(input: &String) -> usize {
    let mut sum = 0;
    const AMOUNT_BATTERIES: usize = 12;
    for line in input.lines(){
        let mut power: usize = 0;
        let mut start_index = 0;
        for count in 0..AMOUNT_BATTERIES{
            let (value, index) = find_largest(line, start_index, line.len()-(AMOUNT_BATTERIES-count-1));
            start_index = index + 1;
            power += 10_usize.pow((AMOUNT_BATTERIES-count-1) as u32) * value as usize;
        }
        sum += power;
    }
    sum
} 


fn find_largest(line: &str, start: usize, end:usize) -> (u32, usize){
    let mut largest: char = '0';
    let mut index = 0;
    let chars = line.char_indices().skip(start);
    for (i, c) in chars{
        if i == end {break;}
        if c > largest {
            largest = c;
            index = i;
        }
    }
    (largest.to_digit(10).expect(&format!("Line contains non-number: {line}")),index)
}