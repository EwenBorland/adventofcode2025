use crate::utils::files;
use array2d::{Array2D};

pub fn solution(){
    let input = files::parse_file("./data/04.txt".to_string());
    println!("p1 Accessible rolls: {}",part_1_solution(input.clone()));
    println!("p2 Accessible rolls:{}",part_2_solution(input));
}

pub fn part_1_solution(input:String) -> i64 {
    println!("Starting day 4 part 1");
    let diagram = parse_diagram(input);
    diagram.column_len();
    return 1;
}

pub fn part_2_solution(input:String) -> i64 {
    println!("Starting day 4 part 2");
    let diagram = parse_diagram(input);
    diagram.column_len();
    return 1;
}

pub fn parse_diagram(input:String) -> Array2D<bool> {
    let rows = input.lines().count();
    let cols = input.lines().nth(0).unwrap().len();
    println!("init diagram with {} rows and {} cols", rows, cols);
    let mut diagram:Array2D<bool> = Array2D::filled_with(false, rows, cols);
    for (i_row,line) in input.lines().enumerate(){
        // println!("row {}: {}", i_row, line);
        for (i_col,roll) in line.chars().enumerate(){
            // println!("col {}: {}", i_col, roll);
            diagram[(i_row,i_col)] = if roll == '@' {true} else {false};
        } 
    }
    return diagram;
}