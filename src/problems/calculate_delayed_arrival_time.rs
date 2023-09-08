use crate::Solution;

// 计算列车到站时间
// https://leetcode.cn/problems/calculate-delayed-arrival-time

impl Solution {
    pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
        (arrival_time + delayed_time) % 24
    }
}
