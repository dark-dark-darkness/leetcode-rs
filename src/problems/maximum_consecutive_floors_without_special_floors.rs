struct Solution;
// 2274. 不含特殊楼层的最大连续楼层数
// https://leetcode.cn/problems/maximum-consecutive-floors-without-special-floors/
impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, mut special: Vec<i32>) -> i32 {
        special.sort_unstable();

        special
            .windows(2)
            .map(|arr| arr[1] - arr[0])
            .max()
            .unwrap_or(i32::MIN)
            .max(top - special[special.len() - 1] - 1)
            .max(special[0] - bottom - 1)
    }
}
