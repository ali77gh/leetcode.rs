struct Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut ptr = 0usize;

        for i in s {
            match t.iter().skip(ptr).position(|x| x == i) {
                Some(index) => ptr += index + 1,
                None => return false,
            }
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
            Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::is_subsequence("aaaaaa".to_string(), "bbaaaa".to_string()),
            false
        );
    }
}
