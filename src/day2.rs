use crate::utils::files;

pub fn solution(){
    let input = files::parse_file("./data/02.txt".to_string());
    println!("Sum of invalid IDs: {}",part_1_solution(input.clone()));
    println!("Sum of invalid IDs part 2:{}",part_2_solution(input));
}

#[derive(Debug,PartialEq)]
pub struct IdRange {
    pub start: i64,
    pub end: i64,
}

impl IdRange{
    fn sum_invalid_ids(ids: Vec<i64>) -> i64 {
        let sum: i64 = ids.iter().sum();
        return sum;
    }

    fn find_invalid_ids(&self) -> Vec<i64>{
        let mut invalid_ids:Vec<i64> = Vec::new();
        for i in self.start..self.end+1{
            let i_str = i.to_string();
            let i_len = i_str.len();
            if i_len % 2 != 0 {
                continue;
            }
            if i_str[0..i_len/2] == i_str[(i_len/2)..i_len] {
                invalid_ids.push(i);
            }
            // println!("checking {}, new invalid ids: {:?}",i,invalid_ids);
        }
        return invalid_ids;
    }

    

    fn find_invalid_ids_2(&self) -> Vec<i64>{
        let mut invalid_ids:Vec<i64> = Vec::new();
        for i in self.start..self.end+1{
            let i_str = i.to_string();
            let i_len = i_str.len();
            // get list of divisors
            let divisors = get_divisors(i_len);
            let mut matching = false;
            for d in divisors{
                matching = true;
                for di in 0..i_len/d-1 {
                    if i_str[di*d..(di+1)*d] != i_str[(di+1)*d..(di+2)*d]{
                        matching = false;
                    }
                }
                if matching{break};
            }
            if matching {
                invalid_ids.push(i);
            }
            // println!("checking {}, new invalid ids: {:?}",i,invalid_ids);
        }
        return invalid_ids;
    }

    fn calc(&self) -> i64 {
        return Self::sum_invalid_ids(Self::find_invalid_ids(self))
    }

    fn calc_2(&self) -> i64 {
        return Self::sum_invalid_ids(Self::find_invalid_ids_2(self))
    }
}

pub fn get_divisors(len:usize) -> Vec<usize>{
    let mut divisors: Vec<usize> = Vec::new();
    let mid = len/2;
    // println!("mid {}",mid);
    for j in 1..mid+1{
        // println!("checking {}",j);
        if len % j == 0{
            divisors.push(j);
        }

    }
    return divisors;
}

pub fn parse_ranges(input:String) -> Vec<IdRange> {
    let mut ranges: Vec<IdRange> = Vec::new();
    for r in input.split(',') {
        let range: Vec<&str> = r.split('-').collect();
        ranges.push(IdRange { start: range[0].parse::<i64>().unwrap(), end: range[1].parse::<i64>().unwrap() });
    }
    return ranges
}


pub fn part_1_solution(input:String) -> i64 {
    println!("Starting day 2 part 1");
    let mut sum: i64 = 0;
    let ranges = parse_ranges(input);
    for r in ranges{
        sum += r.calc()
    }
    return sum
}

pub fn part_2_solution(input:String) -> i64 {
    println!("Starting day 2 part 2"); 
    let mut sum: i64 = 0;
    let ranges = parse_ranges(input);
    for r in ranges{
        sum += r.calc_2()
    }
    return sum
}