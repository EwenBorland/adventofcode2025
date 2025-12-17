use crate::utils::files;
use substring::Substring;

pub fn solution(){
    let input = files::parse_file("./data/03.txt".to_string());
    println!("Sum of joltages: {}",part_1_solution(input.clone()));
    println!("Sum of joltages part 2:{}",part_2_solution(input));
}

pub fn part_1_solution(input:String) -> i64 {
    println!("Starting day 3 part 1");
    let mut joltages = 0;
    for bank in input.lines(){
        joltages += get_max_joltage(bank);
    }
    return joltages
}

pub fn part_2_solution(input:String) -> i64 {
    println!("Starting day 3 part 2"); 
    let mut joltages = 0;
    for bank in input.lines(){
        joltages += get_max_joltage_p2(bank);
    }
    return joltages
}

pub fn str_bank_to_vec(bank_str: &str) -> Vec<i64>{
    let mut bank:Vec<i64> = Vec::new();
    for j in bank_str.chars(){
        let i:i64 = j.to_digit(10).unwrap() as i64;
        bank.push(i);
    }
    return bank;
}

pub fn get_max_joltage(bank:&str ) -> i64{
    let v_bank: Vec<i64> = str_bank_to_vec(bank);
    let mut max_cursor1_val: &i64;
    let max_cursor1_pos: usize;
    let max_cursor2_val: i64 ;

    max_cursor1_val = v_bank.iter().max().unwrap();
    max_cursor1_pos = v_bank.iter().position(|x| x == max_cursor1_val).unwrap();

    if max_cursor1_pos == v_bank.len()-1{
        max_cursor2_val = *max_cursor1_val;
        max_cursor1_val = v_bank[0..max_cursor1_pos].iter().max().unwrap();
    } else {
        max_cursor2_val = *v_bank[(max_cursor1_pos+1)..].iter().max().unwrap();
    }

    let output: i64 = ((*max_cursor1_val).to_string() + &max_cursor2_val.to_string()).parse().unwrap();
    return output
}

pub fn find_max_indices_in_sub(sub: &str) -> Vec<(usize, &str)>{
    let mut founds: Vec<(usize, &str)> = vec![];
    for dig in ["9","8","7","6","5","4","3","2","1","0"]{
        founds = sub.match_indices(dig).collect();
        if founds.len() > 0{
            break;
        }
    }
    return founds;
}

pub fn get_max_joltage_p2(bank:&str) -> i64{
    /*
    plan:
    have a moving window that covers the digits we are allowed to chose from
    leftmost index in the window is the previous chosen digit's index
    rightmost is the max index minus the remaining required digits
    within the window search for a 9:
        if found then add it to the chosen digits and generate a new window
        if not found then repeat for 8, 7, 6 etc
    if multiple of the digit are found then 'chose' them all and adjust the new window accordingly
    
    assuming we handle the available digits as a vector (of chars?)
    then the window represents a slice of the vector
     */
    let mut chosen_index: usize = 0;
    let mut required_digits: usize = 12;
    let mut bank_sub: &str = bank.substring(chosen_index, bank.len()-required_digits+1);
    let mut fin: String = "".to_string();

    while required_digits > 0{
        let new_indices = find_max_indices_in_sub(bank_sub);
        if new_indices.len() == 0{
            print!("no indices :(");
            break;
        }
        fin.push_str(new_indices[0].1);
        required_digits -= 1;
        // println!("AAAAAAAAAAAAA: {}, {}, {}", bank_sub, bank_sub.len(), required_digits);
        bank_sub = bank.substring(chosen_index+new_indices[0].0+(12-required_digits), bank.len()-required_digits+1);
        chosen_index += new_indices[0].0;
    }

    return fin.parse::<i64>().unwrap();
}