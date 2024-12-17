struct Solution;
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..n + 1)
            .map(|x| match (x % 3 == 0, x % 5 == 0) {
                (true, true) => "FizzBuzz".to_string(),
                (true, false) => "Fizz".to_string(),
                (false, true) => "Buzz".to_string(),
                (false, false) => x.to_string(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"])
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"])
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        )
    }
}
