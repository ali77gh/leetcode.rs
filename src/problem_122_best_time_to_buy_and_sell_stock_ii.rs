struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        (1..prices.len()).fold(0, |sum, i| {
            let (left, right) = (prices[i - 1], prices[i]);
            sum + if right > left { right - left } else { 0 }
        })
    }
    pub fn max_profit_loop(prices: Vec<i32>) -> i32 {
        let mut profit = 0;

        for (i, today) in prices.iter().skip(1).enumerate() {
            let yesterday = prices[i];
            if yesterday < *today {
                profit += today - yesterday;
            }
        }
        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn case_3() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
