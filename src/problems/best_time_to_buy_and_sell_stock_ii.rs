struct Solution;
// 122. 买卖股票的最佳时机 II
// https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-ii

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ans = 0;

        for i in 1..prices.len() {
            if prices[i - 1] < prices[i] {
                ans += prices[i] - prices[i - 1];
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = vec![7, 1, 5, 3, 6, 4];
        let right = 7;
        let result = Solution::max_profit(nums);
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let nums = vec![1, 2, 3, 4, 5];
        let right = 4;
        let result = Solution::max_profit(nums);
        assert_eq!(result, right)
    }
    #[test]
    fn case_3() {
        let nums = vec![7, 6, 4, 3, 1];
        let right = 0;
        let result = Solution::max_profit(nums);
        assert_eq!(result, right)
    }
}
