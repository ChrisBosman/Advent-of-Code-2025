use std::collections::{HashMap, VecDeque};

struct Node{
    id: usize,
    height: usize
}

pub(crate) 
fn run(input: &String) -> (usize, usize){ 
    // parse input
    let mut name2index = HashMap::new();
    input.lines().map(|l| l.split(':').next().unwrap()).enumerate().for_each(|(i,v)| {name2index.insert(v, i);});
    name2index.insert("out", name2index.len());
    let mut map: Vec<Vec<usize>> = input.lines().map(|l| {
        l.split_once(':').unwrap().1.split_whitespace().map(|s| name2index[s]).collect()
    }).collect();
    map.push(vec![]);
    // Part 1, Brute force depth first
    let mut sol1 = 0;
    check_node(&name2index["you"], &map, &mut sol1, &name2index["out"]);
    
    // Part 2
    // inverse the map to save a bit of time later
    let mut inv_map: Vec<Vec<usize>> = vec![Vec::new();map.len()];
    map.iter().enumerate().for_each(|(n, c)| c.iter().for_each(|cn| inv_map[*cn].push(n)));
    let height_map = create_height_map(name2index["svr"], &map, &inv_map);

    let sol2 = find_paths_optimized(name2index["svr"], &name2index["fft"], &map, &height_map)
                    * find_paths_optimized(name2index["fft"], &name2index["dac"], &map, &height_map)
                    * find_paths_optimized(name2index["dac"], &name2index["out"], &map, &height_map);

    return (sol1, sol2); 
}

fn find_paths_optimized(start: usize, end: &usize, map: &Vec<Vec<usize>>, height_map: &Vec<usize>) -> usize {
    // Same as day 7, run through and merge the streams where possible (height variable is used to denote how many streams have been merged)
    let mut sum = 0;
    let mut frontier = VecDeque::new();
    frontier.push_back(Node {id: start, height: 1});
    let mut last_height = height_map[start];
    loop {
        // Take top element
        let node;
        match frontier.pop_front() {
            Some(val) => {node=val},
            None => {break;},
        }
        if node.id == *end {
            sum += node.height;
            continue;
        }
        if height_map[node.id] >= height_map[*end]{
            continue;
        }
        
        // Check if it is going too fast, if so let it wait
        if height_map[node.id] > last_height + 1{
            frontier.push_back(node);
            continue;
        }

        // Increment one step
        for new_node in &map[node.id] {
            if let Some(index) = frontier.iter().position(|n| n.id == *new_node ){
                frontier[index].height += node.height;
                continue;
            }
            frontier.push_back(Node { id: *new_node, height: node.height });
        }
        last_height = height_map[node.id];
    }
    return sum;
}


fn create_height_map(start: usize, map: &Vec<Vec<usize>>, inv_map: &Vec<Vec<usize>>) -> Vec<usize> {
    // Work my way up, but if I reach one which has an inv_connection that doesn't have a height yet, kill the strand, the slower onces will reach it anyways
    let mut height_map: Vec<usize> = vec![usize::MAX;map.len()];
    height_map[start] = 0;
    let mut frontier = VecDeque::new();
    frontier.push_back(Node {id: start, height: 0});
    'outer: loop {
        // Take top element
        let node;
        match frontier.pop_front() {
            Some(val) => {node=val},
            None => {break;},
        }
        // Check if it is going too fast
        for n in &inv_map[node.id]{
            if height_map[*n] == usize::MAX {
                continue 'outer;
            }
        }
        if frontier.iter().any(|n| n.id == node.id){
            continue;
        }
        // Assign height value
        height_map[node.id] = node.height;

        // Increment one step
        for new_node in &map[node.id] {
            if !frontier.iter().any(|n| n.id == *new_node && n.height == node.height+1){
                frontier.push_back(Node { id: *new_node, height: node.height+1 });
            }
        }
    }
    return height_map
}

fn check_node(id: &usize, map: &Vec<Vec<usize>>, count: &mut usize, end: &usize){
    if id == end {
        *count += 1;
        return;
    }
    if *id >= map.len() {return}
    for next in &map[*id]{
        check_node(next, map, count, end);
    }
}
