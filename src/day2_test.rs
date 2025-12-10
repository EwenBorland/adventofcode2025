#[cfg(test)]
mod tests{
    use crate::day2::IdRange;


    #[test]
    fn test_parse_ranges(){
        struct TestCase{
            name: String,
            input_string: String,
            output_ranges: Vec<crate::day2::IdRange>,
        }
        let test_cases = vec![
            TestCase{name: String::from("11-22"), input_string: String::from("11-22"), output_ranges:vec![IdRange{start:11, end:22}]},
            TestCase{name: String::from("11-22,33-44"), input_string: String::from("11-22,33-44"), output_ranges:vec![IdRange{start:11, end:22},IdRange{start:33, end:44}]},
        ];        
        
        for tc in test_cases  {
            assert_eq!(crate::day2::parse_ranges(tc.input_string), tc.output_ranges, "{}",tc.name);
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