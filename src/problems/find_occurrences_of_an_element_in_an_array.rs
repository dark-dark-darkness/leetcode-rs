use std::collections::{HashMap, HashSet};

struct Solution;

// 3159. 查询数组中元素的出现位置
// https://leetcode.cn/problems/find-occurrences-of-an-element-in-an-array/
impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let coll: Vec<i32> = nums
            .into_iter()
            .enumerate()
            .filter(|(_, v)| *v == x)
            .map(|(i, _)| i as i32)
            .collect();

        queries
            .into_iter()
            .map(|n| match coll.get(n as usize - 1) {
                Some(i) => *i,
                None => -1,
            })
            .collect()
    }
}
