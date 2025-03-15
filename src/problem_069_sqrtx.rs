struct Solution;
impl Solution {
    // O(log(n)) binary search
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as usize;
        let (mut l, mut r) = (1, x);

        while r - l > 1 {
            let mid = (r + l) / 2;
            match (mid * mid).cmp(&x) {
                std::cmp::Ordering::Less => l = mid,
                std::cmp::Ordering::Greater => r = mid,
                std::cmp::Ordering::Equal => return mid as i32,
            }
        }
        l as i32
    }

    // O(n) => 5ms
    pub fn my_sqrt_brute_force(x: i32) -> i32 {
        let x = x.try_into().unwrap();
        let mut counter = 0usize;
        while counter * counter <= x {
            counter += 1;
        }
        (counter - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::my_sqrt(2147395600), 46340);
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::my_sqrt(1), 1);
    }
}
