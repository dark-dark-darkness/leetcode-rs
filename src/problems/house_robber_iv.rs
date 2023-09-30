struct Solution;

// 2560. 打家劫舍 IV
// https://leetcode.cn/problems/house-robber-iv

impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut min_val = *nums.iter().min().unwrap();
        let mut max_val = *nums.iter().max().unwrap();

        while min_val <= max_val {
            let mid_val = (max_val + min_val) / 2;
            let mut count = 0;
            let mut visited = false;
            for &num in nums.iter() {
                if num <= mid_val && !visited {
                    count += 1;
                    visited = true;
                } else {
                    visited = false;
                }
            }
            if count >= k {
                max_val = mid_val - 1;
            } else {
                min_val = mid_val + 1;
            }
        }

        min_val
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = [2, 3, 5, 9];
        let k = 2;
        let right = 5;
        let result = Solution::min_capability(nums.into(), k);
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let nums = [2, 7, 9, 3, 1];
        let k = 2;
        let right = 2;
        let result = Solution::min_capability(nums.into(), k);
        assert_eq!(result, right)
    }
}
