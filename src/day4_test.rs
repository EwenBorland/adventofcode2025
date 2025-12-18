#[cfg(test)]
mod tests{
    use std::vec;

    use array2d::Array2D;

    #[test]
    fn test_part_1_solution(){
        struct TestCase{
            name: String,
            input_string: String,
            exp_count: i64,
        }
        let test_cases = vec![
            TestCase{
                name: String::from("example"),
                input_string: String::from(
                    "..@@.@@@@.
                    @@@.@.@.@@
                    @@@@@.@.@@
                    @.@@@@..@.
                    @@.@@@@.@@
                    .@@@@@@@.@
                    .@.@.@.@@@
                    @.@@@.@@@@
                    .@@@@@@@@.
                    @.@.@@@.@."
                ),
                exp_count: 13
            }
        ];        
        
        for tc in test_cases  {
            assert_eq!(crate::day4::part_1_solution(tc.input_string), tc.exp_count, "{}",tc.name);
        }
        
    }

    #[test]
    fn test_parse_diagram(){
        struct TestCase{
            name: String,
            input_string: String,
            exp_count: Array2D<bool>,
        }
        let test_cases = vec![
            TestCase{
                name: String::from("3x3 false"),
                input_string: String::from(
                    "...\n...\n..."
                ),
                exp_count: Array2D::filled_with(false, 3, 3)
            },
            TestCase{
                name: String::from("3x3 true"),
                input_string: String::from(
                    "@@@\n\
                    @@@\n\
                    @@@"
                ),
                exp_count: Array2D::filled_with(true, 3, 3)
            },
            TestCase{
                name: String::from("3x4 true"),
                input_string: String::from(
                    "@@@@\n\
                    @@@@\n\
                    @@@@"
                ),
                exp_count: Array2D::filled_with(true, 3, 4)
            },
            TestCase{
                name: String::from("3x3 mixed"),
                input_string: String::from(
                    ".@@\n\
                    @.@\n\
                    @@."
                ),
                exp_count: Array2D::from_rows(&vec![vec![false,true,true],vec![true,false,true],vec![true,true,false]]).unwrap()
            }
        ];        
        
        for tc in test_cases  {
            assert_eq!(crate::day4::parse_diagram(tc.input_string), tc.exp_count, "{}",tc.name);
        }
    }
}