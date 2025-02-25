struct Solution;
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        if digits.is_empty() {
            return digits;
        }

        *digits.last_mut().unwrap() += 1;

        let mut carry = false;
        for digit in digits.iter_mut().rev() {
            if *digit > 9 {
                *digit = 0;
                carry = true;
            } else if carry {
                *digit += 1;
                if *digit > 9 {
                    *digit = 0;
                } else {
                    carry = false;
                }
            }
        }

        if carry {
            digits.insert(0, 1);
        }

        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 9]), vec![4, 3, 3, 0]);
    }
}
