struct Solution;
impl Solution {
    // fast implementation (10 min) slow runtime(1ms)
    pub fn is_palindrome_slow(s: String) -> bool {
        let s: String = s
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_numeric())
            .collect();
        let s = s.to_lowercase();
        let r: String = s.chars().rev().collect();
        s == r
    }

    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_numeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();
        let bytes = s.as_bytes();

        let mut left = 0usize;
        let mut right = s.len() - 1;

        for _ in 0..(s.len() / 2) {
            if bytes[left] != bytes[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
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
