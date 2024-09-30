struct Solution;

// 2073. 买票需要的时间
// https://leetcode.cn/problems/time-needed-to-buy-tickets

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = tickets[k];
        tickets
            .iter()
            .map(|v| v.min(&n))
            .enumerate()
            .map(|(i, &v)| if i <= k || v != n { v } else { v - 1 })
            .sum()
    }
}
