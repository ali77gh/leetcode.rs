use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut set: HashSet<char> = HashSet::new();
        let len = chars.len();

        let mut max = 0;

        for i in 0..(len) {
            for j in i..(len) {
                let c = unsafe { chars.get(j).unwrap_unchecked() }; // cache chars
                if set.contains(&c) {
                    max = max.max(set.len());
                    set.clear();
                    break;
                } else {
                    set.insert(*c);
                }
            }
        }
        max = max.max(set.len());

        max as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        )
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        )
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        )
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0)
    }

    #[test]
    fn case_5() {
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1)
    }

    #[test]
    fn case_6() {
        assert_eq!(Solution::length_of_longest_substring("aa".to_string()), 1)
    }
}
