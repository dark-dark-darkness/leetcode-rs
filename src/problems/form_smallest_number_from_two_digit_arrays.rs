use crate::Solution;

// https://leetcode.cn/problems/form-smallest-number-from-two-digit-arrays/

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut min_value = i32::MAX;
        for n1 in nums1 {
            for &n2 in &nums2 {
                if n1 == n2 {
                    min_value = n1.min(min_value);
                } else {
                    min_value = (if n1 < n2 { n1 * 10 + n2 } else { n2 * 10 + n1 }).min(min_value)
                }
            }
        }

        min_value
    }
}
