#[derive(Default, Clone, Debug)]
struct Pair{
    p1:usize,
    p2:usize,
    dist: usize
}

pub(crate) 
fn run(input: &String) -> (usize, usize){ 
    let nr_connections_to_make = if input.len() > 250 {1000} else {10};
    let points: Vec<Vec<usize>> = input.lines().map(|line| line.split(',').map(|s| s.parse().unwrap()).collect()).collect();
    let mut distances =  Vec::new();
    for i in 0..points.len()-1{
        for j in i+1..points.len(){ 
            distances.push(Pair {p1: i, p2: j, dist: calculate_distance(&points[i],&points[j])});
        }
    };

    distances.sort_unstable_by_key(|v| v.dist);
    let mut networks = connect_points(&points, &distances, nr_connections_to_make).ok().unwrap();
    networks.sort_unstable_by_key(|a| a.len());

    let sol1 = networks.iter().skip(networks.len()-3).map(|arr| arr.len()).product();

    let sol2 = match connect_points(&points, &distances, 1000000) {
        Ok(_) => {println!("Not enough connections made"); 0},
        Err(val) => val,
    };
    return (sol1, sol2); 
} 

fn connect_points(points: &Vec<Vec<usize>>, distances: &Vec<Pair>, nr_connections_to_make: usize) -> Result<Vec<Vec<usize>>, usize>{
    let mut unconnected_points: Vec<usize> = (0..points.len()).collect();
    let mut networks: Vec<Vec<usize>> = Vec::new();
    for i in 0..nr_connections_to_make{
        let pair = &distances[i];
        // See if none are already in a network
        if unconnected_points.contains(&pair.p1) && unconnected_points.contains(&pair.p2){
            networks.push(vec![pair.p1,pair.p2]);
            unconnected_points.swap_remove(unconnected_points.iter().position(|v| *v == pair.p1).unwrap());
            unconnected_points.swap_remove(unconnected_points.iter().position(|v| *v == pair.p2).unwrap());
        } else 
        // If one is in a network
        if unconnected_points.contains(&pair.p1){
            for network in &mut networks{
                if !network.contains(&pair.p2) {continue;}
                network.push(pair.p1);
            }
            unconnected_points.swap_remove(unconnected_points.iter().position(|v| *v == pair.p1).unwrap());
        } else
        if unconnected_points.contains(&pair.p2){
            for network in &mut networks{
                if !network.contains(&pair.p1) {continue;}
                network.push(pair.p2);
            }
            unconnected_points.swap_remove(unconnected_points.iter().position(|v| *v == pair.p2).unwrap());
        } else {
            // If both are in a network
            let mut n1 = find_parent_network(&networks, &pair.p1);
            let mut n2 = find_parent_network(&networks, &pair.p2);
            if n1 == n2 {continue;}
            if n2 < n1 {let tmp = n1; n1 = n2; n2=tmp;}
            let mut network = networks.swap_remove(n2);
            networks[n1].append(&mut network);
        }
        if unconnected_points.is_empty() && networks.len() == 1 {
            return Err(points[pair.p1][0]*points[pair.p2][0]);
        }
    }
    return Ok(networks);
}

fn find_parent_network(networks: &Vec<Vec<usize>>, point: &usize) -> usize {
    for net_i in 0..networks.len(){
        if networks[net_i].contains(point) {return net_i;}
    }
    return usize::MAX;
}

fn calculate_distance(p1: &Vec<usize>, p2: &Vec<usize>) -> usize{
    p1.iter().zip(p2.iter()).map(|(v1,v2)|{
        if v2 > v1 {(v2-v1)*(v2-v1)} else {(v1-v2)*(v1-v2)}
    }).sum()
}