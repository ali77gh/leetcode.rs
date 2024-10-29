struct Solution;
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
            Some(x) => x.try_into().unwrap(),
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
    }
}
