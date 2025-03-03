use std::cmp::min;

struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut min_length = usize::MAX;
        let (mut l, mut r, mut sum) = (0usize, 0usize, 0i32);
        while r != nums.len() || l != nums.len() {
            if sum < target && r != nums.len() {
                // expand to right
                sum += nums[r];
                r += 1;
            } else {
                if sum >= target {
                    min_length = min(min_length, r - l);
                }
                // collapse from left
                sum -= nums[l];
                l += 1;
            }
        }

        (if min_length == usize::MAX {
            0
        } else {
            min_length
        }) as i32
    }

    pub fn min_sub_array_len_slow(target: i32, nums: Vec<i32>) -> i32 {
        for size in 1..nums.len() + 1 {
            if nums
                .windows(size)
                .any(|sub_array| sub_array.iter().fold(0, |sum, x| sum + x) >= target)
            {
                return size as i32;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::min_sub_array_len(11, vec![1, 2, 3, 4, 5]), 3);
    }

    #[test]
    fn case_5() {
        assert_eq!(Solution::min_sub_array_len(15, vec![1, 2, 3, 4, 5]), 5);
    }
}
