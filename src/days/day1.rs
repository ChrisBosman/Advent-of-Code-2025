pub(crate) 
fn run(input: &String) -> (usize, usize){  
    let solution1 = part1(&input);
    let solution2 = part2(&input);
    return (solution1, solution2); 
}

fn part1(input: &String) -> usize {
    let mut solution1: usize = 0;
    let mut position = 50;
    
    for line in input.lines(){
        let (dir, count) = line.split_at(1);
        position +=  count.parse::<i32>().expect(&format!("Couldn't parse: '{count}' from '{line}'")) * if dir=="R" {1} else {-1};
        while position > 99 {
            position -= 100;
        } 
        while position < 0 {
            position += 100;
        }
        if position == 0 {
            solution1 += 1;
        }
    }
    solution1
} 

fn part2(input: &String) -> usize {
    let mut solution1: usize = 0;
    let mut position = 50;
    let mut last_direction_was_left = false; // Stores the last direction
    
    for line in input.lines(){
        let (dir, count) = line.split_at(1);
        // We count the crossing of 99->0 and not 0 itself, so add one if it reaches zero, but not crosses 99/0
        if position == 0 && last_direction_was_left && dir == "R" {
            solution1 += 1;
        } 
        // Fixes if it just crossed and is resting at zero, and then rotates back
        if position == 0 && !last_direction_was_left && dir=="L"{
            solution1 -= 1;
        }
        last_direction_was_left = if dir == "R" {false} else {true};  
        position +=  count.parse::<i32>().expect(&format!("Couldn't parse: '{count}' from '{line}'")) * if dir=="R" {1} else {-1};
        while position > 99 {
            position -= 100;
            solution1 += 1;
        } 
        while position < 0 {
            position += 100;
            solution1 += 1;
        }
    }
    if position == 0 && last_direction_was_left {
            solution1 += 1;
    } 
    solution1
} 
