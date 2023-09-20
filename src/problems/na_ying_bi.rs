struct Solution;

// LCP 06. 拿硬币
// https://leetcode.cn/problems/na-ying-bi

impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        coins.into_iter().map(|x| (x + 1) >> 1).sum()
    }
}
