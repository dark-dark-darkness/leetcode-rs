use crate::Solution;

// 198. 打家劫舍
// https://leetcode.cn/problems/house-robber/

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = [0; 2];
        for num in nums {
            dp = [dp[1], (dp[0] + num).max(dp[1])];
        }
        dp[1]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn case_1() {
        let nums = [1, 2, 3, 1];
        let right = 4;
        let result = Solution::rob(nums.into());
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let nums = [2, 7, 9, 3, 1];
        let right = 12;
        let result = Solution::rob(nums.into());
        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let nums = [
            114, 117, 207, 117, 235, 82, 90, 67, 143, 146, 53, 108, 200, 91, 80, 223, 58, 170, 110,
            236, 81, 90, 222, 160, 165, 195, 187, 199, 114, 235, 197, 187, 69, 129, 64, 214, 228,
            78, 188, 67, 205, 94, 205, 169, 241, 202, 144, 240,
        ];
        let right = 4173;
        let result = Solution::rob(nums.into());
        assert_eq!(result, right);
    }
}
