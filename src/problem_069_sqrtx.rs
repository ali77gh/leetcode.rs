struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
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
}
