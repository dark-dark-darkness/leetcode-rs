struct Solution;

// 3153. 所有数对中数位不同之和
// https://leetcode.cn/problems/sum-of-digit-differences-of-all-pairs

impl Solution {
    pub fn sum_digit_differences(mut nums: Vec<i32>) -> i64 {
        let mut res: i64 = 0;
        let n = nums.len();
        while nums[0] > 0 {
            let mut cnt = [0; 10];
            for i in 0..n {
                cnt[nums[i] as usize % 10] += 1;
                nums[i] /= 10;
            }
            for c in cnt {
                res += (n - c) as i64 * c as i64;
            }
        }
        res / 2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let nums = [13, 23, 12];
        let right = 4;
        let result = Solution::sum_digit_differences(nums.to_vec());
        assert_eq!(result, right)
    }

    #[test]
    fn case_2() {
        let nums = [10, 10, 10];
        let right = 0;
        let result = Solution::sum_digit_differences(nums.to_vec());
        assert_eq!(result, right)
    }
}
