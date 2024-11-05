struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if *stack.last().unwrap_or(&'-') != '(' {
                        return false;
                    }
                    stack.pop();
                }
                ']' => {
                    if *stack.last().unwrap_or(&'-') != '[' {
                        return false;
                    }
                    stack.pop();
                }
                '}' => {
                    if *stack.last().unwrap_or(&'-') != '{' {
                        return false;
                    }
                    stack.pop();
                }
                _ => panic!("invalid char"),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // leet code tests
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([])".to_string()), true);
        assert_eq!(Solution::is_valid("[".to_string()), false);

        // my tests
        assert_eq!(Solution::is_valid(")".to_string()), false);
        assert_eq!(Solution::is_valid("())".to_string()), false);
    }
}
