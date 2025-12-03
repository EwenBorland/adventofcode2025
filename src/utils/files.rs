use std::fs;

pub fn parse_file(path:String) -> String{
    let contents = fs::read_to_string(path).unwrap();
    return contents;
}