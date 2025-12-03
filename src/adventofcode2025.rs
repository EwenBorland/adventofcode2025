// use std::cmp::Ordering;
// use std::io;

// use rand::Rng;

mod day1;
mod utils;

fn main(){
    day1::day1::part_1_solution();
    let file_name: String = "./data/test_input.txt".to_string();
    let my_array = utils::utils::parse_file(file_name);
    println!("{my_array}");
}


// fn main() {
//     let secret_number = rand::rng().random_range(1..=10);

//     println!("Guess the number!");

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue,
//         };

//         match guess.cmp(&secret_number){
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break
//             }
//         };
//     }
// }