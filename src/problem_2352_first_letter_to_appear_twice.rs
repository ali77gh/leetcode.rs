struct Solution;
impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut memo = [false; 26];

        unsafe {
            s.bytes()
                .find(|c| {
                    let i = (*c - 97) as usize;
                    let last = memo[i];
                    memo[i] = !last;
                    return last;
                })
                .unwrap_unchecked() as char
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::repeated_character("abccbaacz".to_string()), 'c');
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::repeated_character("abcdd".to_string()), 'd');
    }
}
