use std::fs;
use std::env;

pub fn parse_file(path:String) -> String{
    let cwd = env::current_dir().expect("huh");
    println!("current dir: {}",cwd.display());
    let contents = fs::read_to_string(path).expect("aaaaa");
    return contents;
}