#[cfg(test)]
mod tests{
    use std::vec;

    #[test]
    fn test_get_max_joltage(){
        struct TestCase<'a>{
            name: String,
            input_str: &'a str,
            exp_joltage: i64,
        }
        let test_cases = vec![
            TestCase{name: String::from("123811"), input_str: "123811", exp_joltage: 81},
            TestCase{name: String::from("9911"), input_str: "9911", exp_joltage: 99},
            TestCase{name: String::from("1189"), input_str: "1189", exp_joltage: 89},
            TestCase{name: String::from("1198"), input_str: "1198", exp_joltage: 98},
            TestCase{name: String::from("81119"), input_str: "81119", exp_joltage: 89},
            TestCase{name: String::from("11111"), input_str: "11111", exp_joltage: 11},
            TestCase{name: String::from("99999"), input_str: "99999", exp_joltage: 99},
        ];        
        
        for tc in test_cases  {
            assert_eq!(crate::day3::get_max_joltage(tc.input_str), tc.exp_joltage, "{}",tc.name);
        }
        
    }

    #[test]
    fn test_str_bank_to_vec(){
        struct TestCase<'a >{
            name: String,
            input_str: &'a str,
            exp_vec: Vec<i64>,
        }
        let test_cases = vec![
            TestCase{name: String::from("1"), input_str: "1", exp_vec: vec![1]},
            TestCase{name: String::from("123811"), input_str: "123811", exp_vec: vec![1,2,3,8,1,1]},
        ];        
        
        for tc in test_cases  {
            assert_eq!(crate::day3::str_bank_to_vec(tc.input_str), tc.exp_vec, "{}",tc.name);
        }
    }

    #[test]
    fn test_get_divisors(){
        struct TestCase{
            name: String,
            input_len: usize,
            exp_result: Vec<usize>,
        }
        let test_cases = vec![
            TestCase{name: String::from("2"), input_len: 2, exp_result: vec![1]},
            TestCase{name: String::from("3"), input_len: 3, exp_result: vec![1]},
            TestCase{name: String::from("4"), input_len: 4, exp_result: vec![1,2]},
            TestCase{name: String::from("5"), input_len: 5, exp_result: vec![1]},
            TestCase{name: String::from("6"), input_len: 6, exp_result: vec![1,2,3]},
        ];        
        
        for tc in test_cases  {
            assert_eq!(crate::day2::get_divisors(tc.input_len), tc.exp_result, "{}",tc.name);
        }
    }

    #[test]
    fn test_find_max_indices_in_sub(){
        struct TestCase<'a>{
            name: String,
            input_string: &'a str,
            exp_result: Vec<(usize, &'a str)>,
        }
        let test_cases = vec![
            TestCase{name: String::from("8"), input_string: "8", exp_result: vec!((0 ,"8"))},
            TestCase{name: String::from("909"), input_string: "909", exp_result: vec!((0 ,"9"),(2 ,"9"))},
            TestCase{name: String::from("989"), input_string: "989", exp_result: vec!((0 ,"9"),(2 ,"9"))},
        ];        
        
        for tc in test_cases  {
            assert_eq!(crate::day3::find_max_indices_in_sub(tc.input_string), tc.exp_result, "{}",tc.name);
        }
    }

    #[test]
    fn test_get_max_joltage_p2(){
        struct TestCase<'a>{
            name: String,
            input_str: &'a str,
            exp_joltage: i64,
        }
        let test_cases = vec![
            TestCase{name: String::from("987654321111111"), input_str: "987654321111111", exp_joltage: 987654321111},
            TestCase{name: String::from("811111111111119"), input_str: "811111111111119", exp_joltage: 811111111119},
            TestCase{name: String::from("234234234234278"), input_str: "234234234234278", exp_joltage: 434234234278},
            TestCase{name: String::from("818181911112111"), input_str: "818181911112111", exp_joltage: 888911112111},
        ];        
        
        for tc in test_cases  {
            assert_eq!(crate::day3::get_max_joltage_p2(tc.input_str), tc.exp_joltage, "{}",tc.name);
        }
        
    }
}