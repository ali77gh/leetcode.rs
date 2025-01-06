struct Solution;
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let s = s.trim();
        s.chars().rev().position(|x| x == ' ').unwrap_or(s.len()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::length_of_last_word("a".to_string()), 1);
    }

    #[test]
    fn case_5() {
        assert_eq!(Solution::length_of_last_word(" a".to_string()), 1);
    }
}
