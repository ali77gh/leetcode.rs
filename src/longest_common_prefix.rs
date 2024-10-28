pub struct Solution;
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prefix = match strs.first() {
            Some(x) => x.clone(),
            None => return String::from(""),
        };

        loop {
            if strs.iter().all(|x| x.starts_with(&prefix)) {
                return prefix;
            } else {
                prefix.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(Solution::longest_common_prefix(vec![]), "");
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "car".to_string(),
                "racecar".to_string()
            ]),
            ""
        );
    }
}
