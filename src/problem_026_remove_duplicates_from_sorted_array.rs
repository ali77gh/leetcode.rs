struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut ans: Vec<i32> = Vec::new();
        let mut current = i32::MIN; // consider: -100 <= nums[i] <= 100
        for i in &mut *nums {
            if *i != current {
                current = *i;
                ans.push(*i);
            }
        }

        *nums = ans;
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut input = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut input), 2);
        assert_eq!(input, vec![1, 2]);
    }

    #[test]
    fn case_2() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut input), 5);
        assert_eq!(input, vec![0, 1, 2, 3, 4]);
    }
}
