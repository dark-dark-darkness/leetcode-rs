struct Solution;

// 3264. K 次乘运算后的最终数组 I
// https://leetcode.cn/problems/final-array-state-after-k-multiplication-operations-i/
impl Solution {
    pub fn get_final_state(mut nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        for _ in 0..k {
            let min = nums.iter_mut().min();
            if let Some(min) = min {
                *min *= multiplier;
            }
        }
        nums
    }
}
