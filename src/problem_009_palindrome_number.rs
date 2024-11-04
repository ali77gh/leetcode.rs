struct Solution;
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s = x.to_string();
        let mut r1 = s.chars().rev();
        let mut r2 = s.chars();

        while let (Some(c1), Some(c2)) = (r1.next(), r2.next()) {
            if c1 != c2 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
