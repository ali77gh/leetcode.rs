use std::{collections::HashMap, convert::TryFrom};

struct Solution;

impl Solution {
    // this is 8x slower than hashmap
    // O( n * log(n) )
    pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums.get(i).unwrap() + nums.get(j).unwrap() == target {
                    return vec![i32::try_from(i).unwrap(), i32::try_from(j).unwrap()];
                }
            }
        }
        vec![]
    }

    // this is 8x faster than brute force
    // O( n )
    pub fn two_sum_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::<i32, i32>::with_capacity(nums.len());

        for (i, n) in nums.iter().enumerate() {
            hashmap.insert(*n, i.try_into().unwrap());
        }

        dbg!(&hashmap);

        for (i, n) in nums.iter().enumerate() {
            let diff = target - n;
            dbg!(i, n, diff);
            if let Some(x) = hashmap.get(&diff) {
                let i = i32::try_from(i).unwrap();
                if *x != i {
                    return vec![i, *x];
                }
            }
        }

        vec![]
    }

    // 2x faster that hashmap
    // O ( n )
    // use cast for type converting
    pub fn two_sum_hashmap2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::<i32, i32>::with_capacity(nums.len());

        for (i, n) in nums.iter().enumerate() {
            hashmap.insert(*n, i as i32);
        }

        for (i, n) in nums.iter().enumerate() {
            let diff = target - n;
            if let Some(x) = hashmap.get(&diff) {
                let i = i as i32;
                if *x != i {
                    return vec![i, *x];
                }
            }
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brute_force() {
        assert_eq!(
            Solution::two_sum_brute_force(vec![2, 7, 11, 15], 9),
            vec![0, 1]
        );
    }

    #[test]
    fn hashmap() {
        assert_eq!(Solution::two_sum_hashmap(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_hashmap(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn hashmap2() {
        assert_eq!(
            Solution::two_sum_hashmap2(vec![2, 7, 11, 15], 9),
            vec![0, 1]
        );
        assert_eq!(Solution::two_sum_hashmap2(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
