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

    // two times slower!
    pub fn is_palindrome_byte_half_iter(x: i32) -> bool {
        let s = x.to_string();
        let bytes = s.as_bytes();

        let mut i: usize = 0;
        while i < bytes.len() / 2 {
            if bytes[i] != bytes[bytes.len() - i - 1] {
                return false;
            }
            i += 1;
        }
        true
    }

    // 0ms beats 100%
    pub fn is_palindrome_half_iter(x: i32) -> bool {
        let s = x.to_string(); // allocation
        let mut r1 = s.chars().rev(); // no allocation
        let mut r2 = s.chars(); // no allocation

        for _ in 0..(s.len() / 2) {
            if r1.next() != r2.next() {
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
    fn first_try() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }

    #[test]
    fn half_bytes() {
        assert_eq!(Solution::is_palindrome_byte_half_iter(121), true);
        assert_eq!(Solution::is_palindrome_byte_half_iter(-121), false);
        assert_eq!(Solution::is_palindrome_byte_half_iter(10), false);
    }

    #[test]
    fn half_str() {
        assert_eq!(Solution::is_palindrome_half_iter(121), true);
        assert_eq!(Solution::is_palindrome_half_iter(-121), false);
        assert_eq!(Solution::is_palindrome_half_iter(10), false);
    }
}
