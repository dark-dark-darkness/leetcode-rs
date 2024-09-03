struct Solution;

// 2708. 一个小组的最大实力值
// https://leetcode.cn/problems/maximum-subarray/

impl Solution {
    pub fn max_strength(nums: Vec<i32>) -> i64 {
        let dc = nums.iter().filter(|&&x| x < 0).count();
        if dc % 2 == 0 {
            return nums
                .iter()
                .filter(|&&x| x != 0)
                .fold(1, |acc, &v| acc * v as i64);
        } else {
            let max = *nums.iter().filter(|&&x| x < 0).max().unwrap();
            return nums
                .iter()
                .filter(|&&x| x != 0)
                .fold(1_i64, |acc, &v| acc * v as i64)
                / max as i64;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = [3, -1, -5, 2, 5, -9];
        let right = 1350;
        let result = Solution::max_strength(nums.to_vec());
        assert_eq!(result, right);
    }
}
