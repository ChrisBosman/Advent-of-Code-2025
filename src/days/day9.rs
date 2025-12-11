struct Square{
    p1:usize,
    p2:usize,
    area: usize
}

pub(crate) 
fn run(input: &String) -> (usize, usize){
    let points: Vec<Vec<usize>> = input.lines().map(|line| line.split(',').map(|s| s.parse().unwrap()).collect()).collect();
    let min_x = points.iter().map(|f| f[0]).min().unwrap();
    let min_y = points.iter().map(|f| f[1]).min().unwrap();
    let max_x = points.iter().map(|f| f[0]).max().unwrap();
    let max_y = points.iter().map(|f| f[1]).max().unwrap();
    println!("xlim [{}, {}]\t ylim [{}, {}]",min_x,max_x,min_y,max_y);
    let areas = part1(&points);
    // for p in &areas{
    //     println!("{}-{}: {}\t\t\t{},{} - {},{}",p.p1,p.p2,p.area,points[p.p1][0],points[p.p1][1],points[p.p2][0],points[p.p2][1]);
    // }
    let sol1 = areas.last().unwrap().area;
 
    return (sol1, 0); 
}

fn part1(points: &Vec<Vec<usize>>) -> Vec<Square> {
    let mut areas =  Vec::new();
    for i in 0..points.len()-1{
        for j in i+1..points.len(){ 
            areas.push(Square {p1: i, p2: j, area: calculate_area(&points[i],&points[j])});
        }
    };
    
    areas.sort_unstable_by_key(|v| v.area);
    areas
} 


fn calculate_area(p1: &Vec<usize>, p2: &Vec<usize>) -> usize{
    p1.iter().zip(p2.iter()).map(|(v1,v2)|{
        if v2 > v1 {v2-v1+1} else {v1-v2+1}
    }).product()
}