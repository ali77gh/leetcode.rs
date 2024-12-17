struct Solution;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::{HashMap, HashSet};
        let map: HashMap<i32, usize> = HashMap::from_iter((0..nums.len()).map(|i| (nums[i], i)));
        let mut set = HashSet::new();

        for i in 0..(nums.len()) - 2 {
            for j in i + 1..(nums.len()) - 1 {
                let k = -(nums[i] + nums[j]);
                if let Some(kv) = map.get(&k) {
                    if *kv > j {
                        let mut v = vec![nums[i], nums[j], k];
                        v.sort();
                        set.insert(v);
                    }
                }
            }
        }

        set.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn case_1() {
    //     assert_eq!(
    //         Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
    //         vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    //     )
    // }

    // #[test]
    // fn case_2() {
    //     assert_eq!(Solution::three_sum(vec![0, 1, 1]), vec![] as Vec<Vec<i32>>)
    // }

    // #[test]
    // fn case_3() {
    //     assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![vec![0, 0, 0]])
    // }
}
