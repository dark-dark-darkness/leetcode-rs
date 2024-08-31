struct Solution;
// 1984. 学生分数的最小差值
// https://leetcode.cn/problems/minimum-difference-between-highest-and-lowest-of-k-scores/
impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return 0;
        }
        let k = k as usize;
        let len = nums.len();
        nums.sort_unstable();
        let mut ans = nums[k - 1] - nums[0];

        for i in 1..len - k + 1 {
            ans = ans.min(nums[i + k - 1] - nums[i]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let (k, nums) = (1, [90]);
        let right = 0;
        let result = Solution::minimum_difference(nums.to_vec(), k);
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let (k, nums) = (2, [9, 4, 1, 7]);
        let right = 2;
        let result = Solution::minimum_difference(nums.to_vec(), k);
        assert_eq!(result, right);
    }
}
