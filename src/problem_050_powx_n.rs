struct Solution;
impl Solution {
    /// O(log(n))
    /// x^n = x^(n/2) * x^(n/2)
    pub fn my_pow(x: f64, n: i64) -> f64 {
        if x == 1f64 {
            return x;
        }
        if n < 0 {
            return Self::my_pow(1f64 / x, -1 * (n + 1)) / x;
        }

        if n == 0 {
            return 1f64;
        }

        let half = Self::my_pow(x, n / 2);
        half * half * if n % 2 == 0 { 1f64 } else { x }
    }

    /// O(n) time out!
    pub fn my_pow_liner(x: f64, n: i32) -> f64 {
        if n > 0 {
            (0..n).fold(1f64, |r, _| r * x)
        } else {
            (0..(-1 * n)).fold(1f64, |r, _| r * (1f64 / x))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert!((Solution::my_pow(2f64, 10) - 1024f64).abs() < 0.001);
    }

    #[test]
    fn case_2() {
        assert!((Solution::my_pow(2.1f64, 3) - 9.261f64).abs() < 0.001);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::my_pow(2f64, -2), 0.25f64);
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::my_pow(5f64, 0), 1f64);
    }

    #[test]
    fn case_5() {
        assert_eq!(Solution::my_pow(0.00001f64, 2147483647), 0f64);
    }

    #[test]
    fn case_6() {
        assert_eq!(Solution::my_pow(1f64, -2147483648), 1f64);
    }

    #[test]
    fn case_7() {
        assert_eq!(Solution::my_pow(2f64, -2147483648), 0f64);
    }
}
