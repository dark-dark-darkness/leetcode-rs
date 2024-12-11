struct Solution;
// 3254. 长度为 K 的子数组的能量值 I
// https://leetcode.cn/problems/find-the-power-of-k-size-subarrays-i
impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut result = vec![-1; nums.len() - k + 1];

        for i in 0..result.len() {
            let mut l = nums[i];
            for j in 1..k {
                let c = nums[i + j];
                if c <= l {
                    result[i] = -1;
                    break;
                }
                l = c;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = [1, 2, 3, 4, 3, 2, 5];
        let k = 3;
        let right = [3, 4, -1, -1, -1];
        let result = Solution::results_array(nums.to_vec(), k);
        assert_eq!(result, right.to_vec())
    }

    #[test]
    fn case_2() {
        let nums = [2, 2, 2, 2, 2];
        let k = 4;
        let right = [-1, -1];
        let result = Solution::results_array(nums.to_vec(), k);
        assert_eq!(result, right.to_vec())
    }

    #[test]
    fn case_3() {
        let nums = [3, 2, 3, 2, 3, 2];
        let k = 2;
        let right = [-1, 3, -1, 3, -1];
        let result = Solution::results_array(nums.to_vec(), k);
        assert_eq!(result, right.to_vec())
    }
}