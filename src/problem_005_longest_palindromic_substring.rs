struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut max_length = 0usize;
        let mut max_i = 0usize;
        let mut max_j = 0usize;
        for i in 0..len {
            for j in i..len + 1 {
                // dbg!(i, j);
                dbg!(&bytes[i..j]);
                if (&bytes[i..j]).is_palindrome() {
                    let size = j - i;
                    if size > max_length {
                        max_length = size;
                        max_i = i;
                        max_j = j;
                    }
                }
            }
        }

        s[max_i..max_j].to_string()
    }
}

pub trait Palindrome {
    fn is_palindrome(&self) -> bool;
}

impl Palindrome for &[u8] {
    fn is_palindrome(&self) -> bool {
        for left_ptr in 0..(self.len() / 2) {
            let right_ptr = self.len() - left_ptr - 1;
            if self[left_ptr] != self[right_ptr] {
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
    fn case_1() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::longest_palindrome("ac".to_string()),
            "a".to_string()
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::longest_palindrome("bb".to_string()),
            "bb".to_string()
        );
    }
}
