#[cfg(test)]
mod tests{
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
    fn test_part_1_solution(){
        struct TestCase{
            name: String,
            input_string: String,
            exp_result: i64,
        }
        let test_cases = vec![
            TestCase{name: String::from("11-22"), input_string: String::from("11-22"), exp_result: 33},
            TestCase{name: String::from("11-22,33-44"), input_string: String::from("11-22,33-44"), exp_result: 110},
            TestCase{name: String::from("provided example"), input_string: String::from("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"), exp_result: 1227775554},
        ];        
        
        for tc in test_cases  {
            assert_eq!(crate::day2::part_1_solution(tc.input_string), tc.exp_result, "{}",tc.name);
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
    fn test_part_2_solution(){
        struct TestCase{
            name: String,
            input_string: String,
            exp_result: i64,
        }
        let test_cases = vec![
            TestCase{name: String::from("11-22"), input_string: String::from("11-22"), exp_result: 33},
            TestCase{name: String::from("11-22,33-44"), input_string: String::from("11-22,33-44"), exp_result: 110},
            TestCase{name: String::from("provided example"), input_string: String::from("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"), exp_result: 4174379265},
        ];        
        
        for tc in test_cases  {
            assert_eq!(crate::day2::part_2_solution(tc.input_string), tc.exp_result, "{}",tc.name);
        }
        
    }
}