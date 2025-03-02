struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut temp = s
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        temp.reverse();
        temp.join(" ").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the".to_string()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::reverse_words("  hello world  ".to_string()),
            "world hello".to_string()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::reverse_words("a good   example".to_string()),
            "example good a".to_string()
        );
    }
}
