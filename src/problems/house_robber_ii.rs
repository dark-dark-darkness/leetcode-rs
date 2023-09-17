struct Solution;

// 213. 打家劫舍 II
// https://leetcode.cn/problems/house-robber-ii

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.as_slice() {
            [] => 0,
            &[num] => num,
            &[a, b] => a.max(b),
            _ => {
                let a = rob(&nums[1..]);
                let b = rob(&nums[2..nums.len() - 1]) + nums[0];

                i32::max(a, b)
            }
        }
    }
}

fn rob(nums: &[i32]) -> i32 {
    let mut dp = [0; 2];
    for num in nums {
        dp = [dp[1], (dp[0] + num).max(dp[1])];
    }
    dp[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums = [2, 3, 2];
        let right = 3;
        let result = Solution::rob(nums.into());
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let nums = [1, 2, 3, 1];
        let right = 4;
        let result = Solution::rob(nums.into());
        assert_eq!(result, right);
    }
}
