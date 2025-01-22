struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut new_vec = Vec::<i32>::new();

        if nums.len() == 0 || nums.len() == 1 {
            return nums.len() as i32;
        }

        new_vec.push(*nums.get(0).unwrap_or(&0));
        new_vec.push(*nums.get(1).unwrap_or(&0));

        for num in nums.iter().skip(2) {
            let (a, b) = (new_vec[new_vec.len() - 1], new_vec[new_vec.len() - 2]);

            if !(b == *num && a == *num) {
                new_vec.push(*num);
            }
        }

        *nums = new_vec;
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut v = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut v), 5);
        assert_eq!(v, vec![1, 1, 2, 2, 3]);
    }

    #[test]
    fn case2() {
        let mut v = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut v), 7);
        assert_eq!(v, vec![0, 0, 1, 1, 2, 3, 3]);
    }

    #[test]
    fn case3() {
        let mut v = vec![];
        assert_eq!(Solution::remove_duplicates(&mut v), 0);
        // assert_eq!(v, vec![]); // not important
    }

    #[test]
    fn case4() {
        let mut v = vec![3];
        assert_eq!(Solution::remove_duplicates(&mut v), 1);
        assert_eq!(v, vec![3]); // not important
    }
}
