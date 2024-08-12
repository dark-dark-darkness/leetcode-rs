struct Solution;

// 3131. 找出与数组相加的整数 I
// https://leetcode.cn/problems/find-the-integer-added-to-array-i

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let min1 = *nums1.iter().min().unwrap_or(&0);
        let min2 = *nums2.iter().min().unwrap_or(&0);
        return min2 - min1;
    }
}
