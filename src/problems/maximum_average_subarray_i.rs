struct Solution;
// 643. 子数组最大平均数 I
// https://leetcode.cn/problems/maximum-average-subarray-i/

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let len = nums.len();
        let k = k as usize;
        let kf = k as f64;
        let mut avg: f64 = nums[0..k].iter().sum::<i32>() as f64 / kf;
        let mut max = avg;

        for i in 1..len - k + 1 {
            println!("{}", avg);
            avg = avg - nums[i - 1] as f64 / kf + nums[i + k - 1] as f64 / kf;
            max = max.max(avg);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let (nums, k) = ([1, 12, -5, -6, 50, 3], 4);
        let right = 12.75;
        let result = Solution::find_max_average(nums.to_vec(), k);
        assert_eq!(result, right);
    }

    #[test]
    fn case_2() {
        let (nums, k) = ([5], 1);
        let right = 5.0;
        let result = Solution::find_max_average(nums.to_vec(), k);
        assert_eq!(result, right);
    }

    #[test]
    fn case_3() {
        let (nums, k) = ([0, 1, 1, 3, 3], 4);
        let right = 2.0;
        let result = Solution::find_max_average(nums.to_vec(), k);
        assert_eq!(result, right);
    }
}
