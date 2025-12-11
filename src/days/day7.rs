use std::collections::VecDeque;

struct Node {
    x: usize,
    y: usize,
    multiplier: usize
}

pub(crate) 
fn run(input: &String) -> (usize, usize){ 
    let matrix: Vec<Vec<u8>> = input.lines().map(|line| line.chars().map(|c| if c == '^' {1} else if c=='S' {2} else {0}).collect()).collect();
    let val = matrix.iter().flatten().position(|v| *v==2).expect("[Error] No Starting position found");
    let starting_index = (val / matrix[0].len(), val % matrix[0].len());

    let sol1 = part1(&matrix, starting_index);
    let sol2 = breath_first(starting_index, &matrix);

    return (sol1, sol2); 
}

fn breath_first(starting_index: (usize, usize), matrix: &Vec<Vec<u8>>) -> usize{
    let mut sum = 0;
    let mut frontier: VecDeque<Node> = VecDeque::new();
    frontier.push_back(Node{x: starting_index.1, y: starting_index.0, multiplier: 1});

    loop {
        // Take top element
        let mut node;
        match frontier.pop_front() {
            Some(val) => {node=val},
            None => {break;},
        }
        // Check if it reached the bottom
        if node.y == matrix.len()-1 {
            sum += node.multiplier; 
            continue;
        }

        // Increment one step
        let mut new_nodes = Vec::new();
        node.y += 1;
        match matrix[node.y][node.x] {
            1 => { // '^'
                    if node.x < matrix[0].len()-1 { new_nodes.push(Node { x: node.x+1, y: node.y, multiplier: node.multiplier }); }
                    if node.x > 0 { node.x -= 1; new_nodes.push(node); }
                },
            _ => new_nodes.push(node),
        }

        // Check if there is already one on this position in the frontier, if not push
        for new_node in new_nodes {
            match frontier.iter().position(|n| n.x == new_node.x && n.y == new_node.y) {
                Some(index) => frontier[index].multiplier += new_node.multiplier,
                None => frontier.push_back(new_node),
            }
        }

        // No need to sort, since all path are the same length
    }

    return sum;
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