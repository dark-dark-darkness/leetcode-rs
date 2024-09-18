struct Solution;

// 3176. 求出最长好子序列 I
// https://leetcode.cn/problems/find-the-maximum-length-of-a-good-subsequence-i

/*
给你一个整数数组 nums 和一个 非负 整数 k 。
如果一个整数序列 seq 满足在下标范围 [0, seq.length - 2] 中 最多只有 k 个下标 i 满足 seq[i] != seq[i + 1] ，那么我们称这个整数序列为 好 序列。
请你返回 nums 中 好子序列的最长长度。
*/

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![vec![-1; 51]; n];
        let mut ans = 0;

        for i in 0..n {
            dp[i][0] = 1;
            for l in 0..=k as usize {
                for j in 0..i {
                    let add = if nums[i] != nums[j] { 1 } else { 0 };
                    if l >= add && dp[j][l - add] != -1 {
                        dp[i][l] = dp[i][l].max(dp[j][l - add] + 1);
                    }
                }
                ans = ans.max(dp[i][l]);
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
        let nums = [1, 2, 1, 1, 3];
        let k = 2;
        let right = 4;

        let result = Solution::maximum_length(nums.to_vec(), k);

        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let nums = [1, 2, 3, 4, 5, 1];
        let k = 0;
        let right = 2;

        let result = Solution::maximum_length(nums.to_vec(), k);

        assert_eq!(result, right)
    }
}
