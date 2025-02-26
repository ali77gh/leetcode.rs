struct Solution;
impl Solution {
    // time complexity: O(n)
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .skip(1)
            .fold((prices[0], 0i32), |acc, x| match acc {
                (b, p) if *x < b => (*x, p),       // change of buy
                (b, p) if x - b > p => (b, x - b), // change of profit
                (b, p) => (b, p),                  // no change
            })
            .1
    }

    // time complexity: O(n)
    pub fn max_profit_loop(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut min = prices[0];

        for i in prices {
            if i < min {
                min = i;
            } else if i - min > max {
                max = i - min;
            }
        }

        max
    }

    // Time complexity: O(nlogn)
    pub fn max_profit_slow(prices: Vec<i32>) -> i32 {
        let mut max = 0;

        for i in 0..prices.len() {
            for j in i..prices.len() {
                let profit = prices[j] - prices[i];
                if profit > max {
                    max = profit;
                }
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn case_2() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
