struct Solution;
impl Solution {
    // 0ms
    // no shifts!, with allocation!
    // creating a new list is faster than doing it in place
    // because we should shift elements at right every time we remove something
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let temp: Vec<i32> = nums.iter().filter(|n| **n != val).map(|n| *n).collect();
        *nums = temp;
        nums.len() as i32
    }

    // 0ms
    // no shifts, no allocation
    pub fn remove_element_2(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        while let Some(n) = nums.get(i) {
            if *n == val {
                nums[i] = *nums.last().unwrap();
                nums.pop();
            } else {
                i += 1;
            }
        }

        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut vec = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element_2(&mut vec, 3), 2);
        // assert_eq!(vec, vec![2, 2]);
    }

    #[test]
    fn case_2() {
        let mut vec = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element_2(&mut vec, 2), 5);
        // assert_eq!(vec, vec![0, 1, 4, 0, 3]);
    }
}
