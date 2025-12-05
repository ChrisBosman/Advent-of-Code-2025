pub(crate) 
fn run(input: &String) -> (usize, usize){ 
    // Parse input to a matrix, True if there is a paper roll (@)
    let mut matrix: Vec<Vec<bool>> = input.lines().map(|line| line.chars().map(|c| if c == '@' {true} else {false}).collect()).collect(); 
    // Part1
    let (solution1, mut removable) = check_removable_cells(&matrix);
    // Part2
    let mut solution2 = solution1;
    while !removable.is_empty(){
        update_matrix(&mut matrix, removable);
        let count;
        (count, removable) = check_removable_cells(&matrix);
        solution2 += count;
    }
    return (solution1, solution2); 
}

fn update_matrix(matrix: &mut Vec<Vec<bool>>, removable: Vec<(usize, usize)>) {
    for (i,j) in removable{
        matrix[i][j] = false;
    }
}

fn check_removable_cells(matrix: &Vec<Vec<bool>>) -> (usize,Vec<(usize, usize)>) {
    let mut removable: Vec<(usize, usize)> = Vec::new();
    let mut available_rolls = 0;
    for i in 0..matrix.len(){
        for j in 0..matrix[i].len(){
            if !matrix[i][j] {continue;}
            // Check the 8 surrounding cells
            let mut nearby_rolls = 0;
            if i>0 && j>0 && matrix[i-1][j-1] {nearby_rolls += 1;}
            if i>0 && matrix[i-1][j] {nearby_rolls += 1;}
            if i>0 && j<matrix[i].len()-1 && matrix[i-1][j+1] {nearby_rolls += 1;}
        
            if j>0 && matrix[i][j-1] {nearby_rolls += 1;}
            if j<matrix[i].len()-1 && matrix[i][j+1] {nearby_rolls += 1;}
            if nearby_rolls >= 4 {continue;}
        
            if i<matrix.len()-1 && j>0 && matrix[i+1][j-1] {nearby_rolls += 1;}
            if i<matrix.len()-1 && matrix[i+1][j] {nearby_rolls += 1;}
            if i<matrix.len()-1 && j<matrix[i].len()-1 && matrix[i+1][j+1] {nearby_rolls += 1;}
    
            if nearby_rolls >= 4 {continue;}
            available_rolls += 1;
            removable.push((i,j));
        }
    }
    (available_rolls, removable)
} 
