struct Problem{
    group: Vec<u128>,
    operator: char,
}

pub(crate) 
fn run(input: &String) -> (usize, usize){ 
    let sol1 = part1(input);
    let sol2 = part2(input);
    
    return (sol1 as usize, sol2 as usize); 
}

fn part2(input: &String) -> u128 {
    //Parse input
    let mut problems: Vec<Problem> = vec![];
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start_index = 0;
    while start_index < lines[0].len(){
        let mut end_index = start_index + 1;
        // Find next */+ on line 5
        let operator_line = lines.len()-1;
        while end_index < lines[operator_line].len() && lines[operator_line][end_index] != '*' && lines[operator_line][end_index] != '+'{
            end_index += 1;
        }
        if end_index == lines[operator_line].len() {end_index += 1;} // There is a empty line between the end of one group and the next

        let mut num_strings: Vec<String> = Vec::new();
        for i in 0..operator_line {
            for j in start_index..end_index-1{
                if i == 0 {num_strings.push(lines[i][j].to_string());}
                else { num_strings[j-start_index] += &lines[i][j].to_string(); }
            }
        }
        let mut group: Vec<u128> = Vec::new();
        for el in num_strings{
            group.push(el.trim().parse().expect("[Error] Can't parse"));
        }
        problems.push(Problem { group, operator: lines[operator_line][start_index] });

        start_index = end_index;
    }

    problems.iter().map(|p| if p.operator == '+' {p.group.iter().sum::<u128>()} else {p.group.iter().product()}).sum::<u128>()
}

fn part1(input: &String) -> u128 {
    //Parse input
    let mut problems: Vec<Problem> = vec![];
    for (line_nr, line) in input.lines().enumerate(){
        for (el_nr, el) in line.split_whitespace().enumerate() {
            if el == "+" || el == "*" {
                problems[el_nr].operator = el.chars().next().expect("[Error] Can't get first char");
            } else if line_nr == 0{
                problems.push(Problem { group: vec![el.parse().expect("[Error] Can't parse")], operator: ' ' });
            } else {
                problems[el_nr].group.push(el.parse().expect("[Error] Can't parse"));
            }
        }
    }

    let sol1 = problems.iter().map(|p| if p.operator == '+' {p.group.iter().sum::<u128>()} else {p.group.iter().product()}).sum::<u128>();
    sol1
} 