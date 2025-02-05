struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_numeric())
            .collect();
        let s = s.to_lowercase();
        let r: String = s.chars().rev().collect();
        s == r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    }
}
