use crate::utils::files;

const UPPER_BOUND: i32 = 100;

pub fn over(mut pos:i32, counter:&mut i32) -> i32{
    while pos >= UPPER_BOUND{
        *counter += 1;
        pos -= UPPER_BOUND;
    }
    
    return pos;
}

pub fn under(mut pos:i32, counter: &mut i32) -> i32{
    while pos < 0{
        *counter += 1;
        pos += UPPER_BOUND;
    }
    if pos == 0{
        *counter += 1;
    }
    return pos;
}
pub fn solution(){
    let input = files::parse_file("./data/01.txt".to_string());
    part_1_solution(input.clone());
    part_2_solution(input);
}

pub fn part_1_solution(input:String) {
    println!("Starting day 1 part 1");

    let mut cur_pos = 50;
    let mut zero_counter:i32 = 0;
    let p2counter= &mut 0;

    for line in input.lines(){
        if line.chars().nth(0).unwrap() == 'R'{
            cur_pos += line[1 .. ].parse::<i32>().unwrap();
        } else {
            cur_pos -= line[1 .. ].parse::<i32>().unwrap();
        }
        // println!("new pos is {}", cur_pos);

        if cur_pos >= UPPER_BOUND{
            cur_pos = over(cur_pos,p2counter);
            // println!("new pos ammended to {}",cur_pos);
        }
        if cur_pos < 0{
            cur_pos = under(cur_pos,p2counter);
            // println!("new pos ammended to {}",cur_pos);
        }
        if cur_pos == 0{
            zero_counter += 1
        }
    }
    println!("Password: {}", zero_counter) // 1123
}

pub fn part_2_solution(input:String) {
    println!("Starting day 1 part 2"); 

    let mut cur_pos = 50;
    let zero_counter = &mut 0;

    for line in input.lines(){
        if line.chars().nth(0).unwrap() == 'R'{
            cur_pos += line[1 .. ].parse::<i32>().unwrap();
        } else {
            if cur_pos == 0{
                *zero_counter -= 1;
            }
            cur_pos -= line[1 .. ].parse::<i32>().unwrap();
        }
        // println!("new pos is {}", cur_pos);

        if cur_pos >= UPPER_BOUND{
            cur_pos = over(cur_pos, zero_counter);
            // println!("new pos ammended to {}",cur_pos);
        }
        else if cur_pos < 0{
            cur_pos = under(cur_pos, zero_counter);
            // println!("new pos ammended to {}",cur_pos);
        }
        else if cur_pos == 0{
            *zero_counter += 1;
        }
    }
    println!("Password: {}",zero_counter) // 6695
}