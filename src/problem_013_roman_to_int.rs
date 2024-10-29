struct Solution;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut answer = 0i32;
        let mut pointer = 0;
        while pointer < s.len() {
            if pointer < s.len() - 1 {
                // is not last
                let slice = &s[pointer..pointer + 2];
                if let Some(x) = Self::map2(&slice) {
                    answer += x;
                    pointer += 2;
                    continue;
                }
            }

            let slice = &s[pointer..pointer + 1];
            answer += Self::map1(&slice).unwrap();
            pointer += 1;
        }

        answer
    }

    fn map1(s: &str) -> Option<i32> {
        Some(match s {
            "I" => 1,
            "V" => 5,
            "X" => 10,
            "L" => 50,
            "C" => 100,
            "D" => 500,
            "M" => 1000,
            _ => return None,
        })
    }

    fn map2(s: &str) -> Option<i32> {
        Some(match s {
            "IV" => 4,
            "IX" => 9,
            "XL" => 40,
            "XC" => 90,
            "CD" => 400,
            "CM" => 900,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
