struct Solution;
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        // in case k is bigger than len we limit k to len
        let k = k as usize % nums.len();

        // temp variable prevent value miss on overwrite
        let leftovers: Vec<i32> = nums
            .iter()
            .skip(nums.len() - k)
            .take(k)
            .map(|x| *x)
            .collect();

        // shift
        for i in (0..(nums.len() - k)).rev() {
            nums[i + k] = nums[i];
        }

        // put left overs to the beginning of array
        for (i, leftover) in leftovers.iter().enumerate() {
            nums[i] = *leftover;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut array = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut array, 3);
        assert_eq!(array, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn case_2() {
        let mut array = vec![-1, -100, 3, 99];
        Solution::rotate(&mut array, 2);
        assert_eq!(array, vec![3, 99, -1, -100]);
    }

    #[test]
    fn case_3() {
        let mut array = vec![-1];
        Solution::rotate(&mut array, 2);
        assert_eq!(array, vec![-1]);
    }

    #[test]
    fn case_4() {
        let mut array = vec![1, 2];
        Solution::rotate(&mut array, 3);
        assert_eq!(array, vec![2, 1]);
    }
}
