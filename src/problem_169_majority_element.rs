use std::collections::HashMap;

struct Solution;
impl Solution {
    // 1ms (can be faster)
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::new();
        let max = nums.len() / 2;
        for num in nums {
            let x = match map.get_mut(&num) {
                Some(x) => {
                    *x += 1;
                    *x
                }
                None => {
                    map.insert(num, 1);
                    1
                }
            };
            if x > max {
                return num;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3)
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2)
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::majority_element(vec![1]), 1)
    }
}
