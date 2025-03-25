struct Solution;

use std::cmp::max;
const ZERO: u8 = 48;
const ONE: u8 = 49;
impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        let a = unsafe { a.as_bytes_mut() };
        let b = unsafe { b.as_bytes_mut() };
        a.reverse();
        b.reverse();
        let mut carry = false;
        let len = max(a.len(), b.len());
        let mut answer: Vec<u8> = Vec::new();

        for i in 0..len {
            let a_bit = a.get(i).unwrap_or(&ZERO);
            let b_bit = b.get(i).unwrap_or(&ZERO);
            match (a_bit, b_bit, carry) {
                (&ZERO, &ZERO, false) => answer.push(ZERO),
                (&ZERO, &ZERO, true) => {
                    answer.push(ONE);
                    carry = false;
                }
                (&ZERO, &ONE, false) | (&ONE, &ZERO, false) => answer.push(ONE),
                (&ZERO, &ONE, true) | (&ONE, &ZERO, true) => {
                    answer.push(ZERO);
                    carry = true;
                }
                (&ONE, &ONE, false) => {
                    answer.push(ZERO);
                    carry = true;
                }
                (&ONE, &ONE, true) => {
                    answer.push(ONE);
                    carry = true;
                }
                _ => {
                    unreachable!();
                }
            }
        }
        if carry {
            answer.push(ONE);
        }

        answer.reverse();
        unsafe { String::from_utf8_unchecked(answer) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100".to_string()
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::add_binary("1010".to_string(), "1011".to_string()),
            "10101".to_string()
        );
    }
}
