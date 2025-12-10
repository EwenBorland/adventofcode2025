use std::ptr::{self, null};

use crate::utils::files;

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
        joltages += get_max_joltage(bank);
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