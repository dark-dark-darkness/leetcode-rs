use crate::Solution;

impl Solution {
    pub fn sum_of_power(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        const N: u64 = 1e9 as u64 + 7;
        let mut ans = 0;
        let mut buffer: u64 = 0;
        for i in 0..nums.len() {
            let min_value: u64 = nums[i] as u64 + buffer;
            ans = (ans + ((nums[i] as u64 * nums[i] as u64) % N) * min_value) % N;
            buffer = (buffer * 2) % N;
            buffer += nums[i] as u64;
        }
        ans as i32
    }
}

// 1 2 4

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let nums = vec![2, 1, 4];
        let sum = Solution::sum_of_power(nums);
        assert_eq!(sum, 141);
    }
}
