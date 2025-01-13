struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let nm = (n + m) as usize;
        let n1 = nums1.split_at_mut(m as usize).0;
        let n2 = nums2.split_at_mut(n as usize).0;
        n1.sort();
        n2.sort();

        let mut result: Vec<i32> = Vec::with_capacity(nm);
        let mut p1 = 0usize;
        let mut p2 = 0usize;

        while p1 + p2 != nm {
            let v1 = *n1.get(p1).unwrap_or(&i32::MAX);
            let v2 = *n2.get(p2).unwrap_or(&i32::MAX);
            if v1 < v2 {
                result.push(v1);
                p1 += 1;
            } else {
                result.push(v2);
                p2 += 1;
            }
        }

        *nums1 = result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn case_2() {
        let mut nums1 = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
        let mut nums2 = vec![1, 2, 2];
        Solution::merge(&mut nums1, 6, &mut nums2, 3);
        assert_eq!(nums1, [-1, 0, 0, 1, 2, 2, 3, 3, 3]);
    }
}
