struct Solution;
impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        const MAX: i32 = i32::MAX - 1; // prevent add overflow on + 1

        coins.sort();

        let amount = amount as usize;
        let mut dp = vec![MAX; amount + 1];
        dp[0] = 0;

        for i in 1..=amount {
            for coin in &coins {
                let coin = *coin as usize;
                if i < coin {
                    break;
                }

                let diff = i - coin;
                dp[i] = i32::min(dp[i], dp[diff] + 1);
            }
        }

        if dp[amount] != MAX {
            dp[amount]
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
    }

    #[test]
    fn case_4() {
        assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
    }
}
