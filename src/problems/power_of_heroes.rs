use crate::Solution;

// https://leetcode.cn/problems/power-of-heroes/

impl Solution {
    pub fn sum_of_power(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        const N: u64 = 1e9 as u64 + 7;
        let mut ans = 0;
        let mut buffer: u64 = 0;
        for num in nums {
            let num = num as u64;
            let min_value: u64 = num + buffer;
            ans = (ans + ((num * num) % N) * min_value) % N;
            buffer = (buffer * 2) % N;
            buffer += num;
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
