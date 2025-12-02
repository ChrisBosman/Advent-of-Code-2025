use colored::Colorize; 
 
pub(crate) 
fn run(input: String) -> (usize, usize){ 
    println!("{}","Day 2".bright_green().bold()); 
    let parsed_input = input.split(',').map(|ranges| ranges.split('-').map(|id| id.parse::<usize>().expect(&format!("Can't parse {}",id))).collect::<Vec<usize>>()).collect::<Vec<Vec<usize>>>();

    let part1_sol = part1(&parsed_input);
    let part2_sol = part2(&parsed_input);

    return (part1_sol, part2_sol); 
}

fn part1(parsed_input: &Vec<Vec<usize>>) -> usize {
    // Plan: check length and them divide by 11 (11 up to 99), 101 (1010 up to 9999), 1001 (100100-999999), 10001
    //                                                      2                  4                      6
    let mut sum = 0;
    for range_inc in parsed_input{
        for id in range_inc[0]..=range_inc[1]{
            let length = id.ilog10();
            if length % 2 == 0 {continue}
            let divisor = usize::pow(10, (length+1)/2) + 1;
            if id % divisor == 0{
                sum += id;
            }
        }
    }
    sum
}

fn part2(parsed_input: &Vec<Vec<usize>>) -> usize {
    let mut sum = 0;
    for range_inc in parsed_input{
        for id in range_inc[0]..=range_inc[1]{
            if is_sequence_num(id) {
                sum += id;
            }
        }
    }
    sum
} 

// 111111111111 / 1001001001 = 111
// 111111111111 / 111111111111 = 1
// 212121212121 / 10101010101 = 21
// 123123123123 / 1001001001 = 123
// 123412341234 / 100010001 = 1234
// 123456123456 / 1000001 = 123456

// Check if it is a sequence
fn is_sequence_num(id: usize) -> bool{
    let length = id.ilog10()+1; 
    // println!("length: {length}");
    for seq_length in 1..=length/2{
        if length % seq_length != 0 {continue;}
        // print!("seq_length: {seq_length}\t");
        let nr_sequences = length/seq_length;
        if nr_sequences < 2 {println!("Why it be less than two, id: {id}")}
        let base_length = length-seq_length;
        let mut divisor = usize::pow(10,base_length) + 1;
        for i in 2..nr_sequences{
            let this_length = base_length - (i-1)*seq_length;
            divisor += usize::pow(10,this_length);
        }
        // println!("Divisor: {divisor}");
        if id % divisor == 0 {
            return true;
        }
    }  
    return false;
}


/*
String based alternative:
fn is_sequence(id_str: String) -> bool{
    let mut pattern = "".to_string();
    for (index, char) in id_str.char_indices(){
        if index > id_str.len()/2 || index >= id_str.len()-1 {return false}
        pattern += &char.to_string();
        if id_str.split(&pattern).all(|part| part.is_empty()) {
            return true
        }
    }
    return false;
}
*/