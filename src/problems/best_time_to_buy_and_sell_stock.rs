struct Solution;
// 121. 买卖股票的最佳时机
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() == 1 {
            return 0;
        }
        let mut min = prices[0];
        let mut max = 0;

        for p in prices[1..].iter() {
            if *p < min {
                min = *p;
            } else {
                max = max.max(p - min);
            }
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = vec![7, 1, 5, 3, 6, 4];
        let right = 5;
        let result = Solution::max_profit(nums);
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let nums = vec![7, 6, 4, 3, 1];
        let right = 0;
        let result = Solution::max_profit(nums);
        assert_eq!(result, right)
    }
}
