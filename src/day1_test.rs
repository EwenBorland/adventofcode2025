


#[cfg(test)]
mod tests{
    struct TestCase{
        name: String,
        startpos: i32,
        exp_endpos: i32,
        exp_endcounter: i32
    }

    #[test]
    fn test_under(){
        let test_cases = vec![
            TestCase{name: String::from("-50 to 50"),startpos: -50, exp_endpos: 50, exp_endcounter: 1},
            TestCase{name: String::from("-75 to 25"),startpos: -75, exp_endpos: 25,exp_endcounter: 1},
            TestCase{name: String::from("-175 to 25"),startpos: -175, exp_endpos: 25,exp_endcounter: 2},
            TestCase{name: String::from("-275 to 25"),startpos: -275, exp_endpos: 25,exp_endcounter: 3},
            TestCase{name: String::from("-100 to 0"),startpos: -100, exp_endpos: 0,exp_endcounter: 2},
            TestCase{name: String::from("-200 to 0"),startpos: -200, exp_endpos: 0,exp_endcounter: 3},
        ];        
        
        for tc in test_cases  {
            let counter= &mut 0;
            assert_eq!(crate::day1::under(tc.startpos, counter),tc.exp_endpos, "{}: failed on position", tc.name);
            assert_eq!(*counter, tc.exp_endcounter, "{}: failed on counter",tc.name);
        }
        
    }

    #[test]
    fn test_over(){
        let test_cases = vec![
            TestCase{name: String::from("150 to 50"),startpos: 150, exp_endpos: 50, exp_endcounter: 1},
            TestCase{name: String::from("175 to 75"),startpos: 175, exp_endpos: 75,exp_endcounter: 1},
            TestCase{name: String::from("275 to 75"),startpos: 275, exp_endpos: 75,exp_endcounter: 2},
            TestCase{name: String::from("100 to 0"),startpos: 100, exp_endpos: 0,exp_endcounter: 1},
            TestCase{name: String::from("200 to 0"),startpos: 200, exp_endpos: 0,exp_endcounter: 2},
        ];        
        
        for tc in test_cases  {
            let counter= &mut 0;
            assert_eq!(crate::day1::over(tc.startpos, counter),tc.exp_endpos, "{}: failed on position", tc.name);
            assert_eq!(*counter, tc.exp_endcounter, "{}: failed on counter",tc.name);
        }
        
    }
}