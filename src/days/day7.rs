pub(crate) 
fn run(input: &String) -> (usize, usize){ 
    let matrix: Vec<Vec<u8>> = input.lines().map(|line| line.chars().map(|c| if c == '^' {1} else if c=='S' {2} else {0}).collect()).collect();
    let val = matrix.iter().flatten().position(|v| *v==2).expect("[Error] No Starting position found");
    let starting_index = (val / matrix[0].len(), val % matrix[0].len());

    let sol1 = part1(&matrix, starting_index);
    let sol2 = part2(&matrix, starting_index);

    // for i in 0..matrix.len() {
    //     for j in 0..matrix[i].len() {
    //         print!(" {}",match mod_matrix[i][j] {
    //             1 => {'^'},
    //             2 => {'S'},
    //             3 => {'v'},
    //             _ => {'.'},
    //         })
    //     }
    //     print!("\n");
    // }

    return (sol1, sol2); 
    // 3178 too low
}

fn part2(matrix: &Vec<Vec<u8>>, starting_index: (usize, usize)) -> usize {
    // Part 2, Just depth first and increment count when reaching the bottom
    let mut mod_matrix: Vec<Vec<usize>> = vec![vec![0; matrix[0].len()];matrix.len()];
    depth_first(starting_index.0, starting_index.1, matrix, &mut mod_matrix)
}

fn breath_first(starting_index: (usize, usize), matrix: &Vec<Vec<u8>>) -> usize{
    let mut sum = 0;
    // let frontier = 


    return sum;
}

fn depth_first(i: usize, j: usize, matrix: &Vec<Vec<u8>>, mod_matrix: &mut Vec<Vec<usize>>) -> usize{
    let mut sum = 0;
    // println!("Hi");
    for y in (i+1)..matrix.len(){
        if y == matrix.len()-1 {return 1;}
        if matrix[y][j] == 0 {continue;} 
        if mod_matrix[y][j] != 0 {
            sum += mod_matrix[y][j];
            break;
        }
        
        // Check left
        let mut left_val = 0;
        if j>0 {
            left_val = depth_first(y, j-1, matrix, mod_matrix);
        }
        // Check right
        let mut right_val = 0;
        if j<matrix[y].len()-1{
            right_val = depth_first(y, j+1, matrix, mod_matrix);
        }

        // Update mod_matrix
        mod_matrix[i][j] = left_val+right_val;

        // Update sum
        sum += mod_matrix[i][j];
        break;
    }
    sum
}

fn part1(matrix: &Vec<Vec<u8>>, starting_index: (usize, usize)) -> usize {
    // Part 1, Plan, start at the bottom, and move up, depth first, marking the point that are valid
    let mut mod_matrix = matrix.clone();
    
    for i in (0..matrix.len()).rev() {
        for j in 0..matrix[i].len(){
            if mod_matrix[i][j] == 1 {
                check_depth_first(i, j, &mut mod_matrix, starting_index);
            }
        }
    }
    
    let sol1 = mod_matrix.iter().flatten().map(|val| if *val == 3 {1} else {0} ).sum();
    sol1
}

// Checks if the splitter gets hit
fn check_depth_first(i: usize,j: usize,mod_matrix: &mut Vec<Vec<u8>>,starting_index: (usize,usize)) -> bool{
    // Move up
    for y in (0..i).rev(){
        // If we pass the start line/point
        if y == starting_index.0{
            if j == starting_index.1 {
                mod_matrix[i][j] = 3;
                return true;
            }
            break;
        }
        // If we encounter a splitter
        if mod_matrix[y][j] != 0 {break;}

        if j<mod_matrix[y].len()-1 && mod_matrix[y][j+1] > 0 {
            // Point of interest
            if mod_matrix[y][j+1] > 1 || check_depth_first(y, j+1, mod_matrix, starting_index){
                mod_matrix[i][j] = 3;
                return true;
            }
        }
        if j>0 && mod_matrix[y][j-1] > 0 {
            // Point of interest
            if mod_matrix[y][j-1] > 1 || check_depth_first(y, j-1, mod_matrix, starting_index){
                mod_matrix[i][j] = 3;
                return true;
            }
        }
    }
    return false;
}



// 1: 2
// 2: 3
// 3: 4 
// 4: 5 or 6
// 6: 8 (two beams hit the same thing)