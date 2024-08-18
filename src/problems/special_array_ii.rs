use core::num;
use std::collections::HashMap;

struct Solution;

// 3152. 特殊数组 II
// https://leetcode.cn/problems/special-array-ii/description/

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();
        let mut dp = vec![1; n];
        for i in 1..n {
            if (nums[i] ^ nums[i - 1]) & 1 != 0 {
                dp[i] = dp[i - 1] + 1;
            }
        }

        queries
            .iter()
            .map(|x| (x[0] as usize, x[1] as usize))
            .map(|(x, y)| dp[y] > y - x)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = [3, 4, 1, 2, 6];
        let queries = [[0, 4]];
        let right = [false];

        let result =
            Solution::is_array_special(nums.to_vec(), queries.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right.to_vec());
    }

    #[test]
    fn case_2() {
        let nums = [4, 3, 1, 6];
        let queries = [[0, 2], [2, 3]];
        let right = [false, true];

        let result =
            Solution::is_array_special(nums.to_vec(), queries.map(|x| x.to_vec()).to_vec());

        assert_eq!(result, right.to_vec());
    }
}
